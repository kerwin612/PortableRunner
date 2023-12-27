import './libs/extensions.js';
import * as API from './libs/apis.js';

const { listen } = window.__TAURI__.event;
const { writeText } = window.__TAURI__.clipboard;

let containerWithFileDropCmd;
let containerSet;
let containerCmd;
let inputTpath;
let inputLpath;
let inputHpath;
let buttonSave;
let inputCmd;
let cmdList;
let filePath;
let matchList;
let buttonBack;
let cfgLMT;
let unListenFileDrop;

function setSave() {
    return API.setSave({tpath: inputTpath.value, lpath: inputLpath.value, hpath: (inputHpath.value ?? "").trim() || ".home"});
}

function cmdInput() {
  let cmdStr = inputCmd.value;
  inputCmd.value = null;
  return API.cmdRunner(cmdStr.trim());
}

function loaded() {
    API.setLoad().then(s => {
        if (!(s.tpath) || !(s.lpath)) {
            showSet();
        } else {
            // showCmd();
            inputTpath.value = s.tpath;
            inputLpath.value = s.lpath;
            inputHpath.value = s.hpath;
            setSave().then(ok => ok && showCmd());
        }
    });
}

function showWithFileDropCmd() {
    containerSet.classList.add("hide");
    containerCmd.classList.add("hide");
    containerWithFileDropCmd.classList.remove("hide");
}

function showSet() {
    containerWithFileDropCmd.classList.add("hide");
    containerCmd.classList.add("hide");
    containerSet.classList.remove("hide");
    unListenFileDrop && unListenFileDrop();
}

function showCmd() {
    refreshCmd();
    containerWithFileDropCmd.classList.add("hide");
    containerSet.classList.add("hide");
    containerCmd.classList.remove("hide");
}

function createItemElement(i, index, parent, idPrefix, getCMD, onClick) {
    let ii = `${idPrefix}_item_${i.key}`;
    let ie = document.getElementById(ii);
    if (ie == null) {
        ie = document.createElement("div");
        ie.classList.add("cmd_item");
        ie.setAttribute("id", ii);
        ie.setAttribute("style", i.style||"");
    }
    parent.insertChildAtIndex(ie, index);

    ie.innerHTML = `<span>${i.label ? (i.label + "(" + i.key + ")") : i.key}</span>`;
    ie.onclick = (e) => {
        onClick(ie, e);
    };
    ie.onmousedown = (e) => {
        let isRightMB;
        e = e || window.event;

        if ("which" in e)  // Gecko (Firefox), WebKit (Safari/Chrome) & Opera
            isRightMB = e.which == 3;
        else if ("button" in e)  // IE, Opera
            isRightMB = e.button == 2;

        if (isRightMB) {
            writeText(getCMD(i)).then(_ => {
                //
            });
        }
    }
}

function getCmdlineWithFile(i, file, fileDrop) {
    return i.cmd + " " + fileDrop.parameters.replaceAll("{0}", file);
}

async function doRefreshCmd(list) {
    let groups = {};
    let items = {};
    list.forEach(i => {
        let group = i.group??"default";
        let value = groups[group]||[];
        value[value.length] = i;
        groups[group] = value;
        items[i.key] = i;
    });

    cmdList.querySelectorAll(`.cmd_sub_list`).forEach(g => {
        if (!(groups[g.id.substring(6)])) {
            g.remove();
        } else {
            g.querySelectorAll(".cmd_item").forEach(i => {
                if (!(items[i.id.substring(5)]) || (i.parentElement.id.substring(6) !== (items[i.id.substring(5)].group??"default"))) {
                    i.remove();
                }
            });
        }
    });

    let gindex = 0;
    for (let group in groups) {

        let gi = `group_${group}`;
        let ge = document.getElementById(gi);
        if (ge == null) {
            ge = document.createElement("div");
            ge.classList.add("cmd_sub_list");
            ge.setAttribute("id", gi);
        }
        cmdList.insertChildAtIndex(ge, gindex++);

        groups[group].forEach((i, iindex) => {
            createItemElement(i, iindex, ge, gi, () => i.cmd, (ie) => {
                if (ie.classList.contains("disabled"))    return;
                ie.classList.add("disabled");
                if (i.parametersRequired) {
                    inputCmd.value = i.cmd + " ";
                    inputCmd.focus();
                    ie.classList.remove("disabled");
                } else {
                    API.cmdRunner(i.cmd).then(_ => {
                        ie.classList.remove("disabled");
                    });
                }
            });
        });
    }

    unListenFileDrop && unListenFileDrop();
    unListenFileDrop = await listen("tauri://file-drop", async event => {
        if ((event?.payload || []).length < 0)  return;
        let file = event.payload[0];
        if (file.endsWith('.lnk')) {
            API.readLnk(file).then(async (lnkInfo) => {
                API.addShortcut({
                    key: (lnkInfo?.name ?? '') || await lnkInfo.target.getFileNameWithoutExt(),
                    cmd: `"${lnkInfo.target}" ${lnkInfo?.arguments ?? ''}`,
                }).then(() => {
                    refreshCmd();
                });
            });
        } else {
            let isDir = await file.isDir();
            let matchs = []
            list.forEach(i => {
                if (!!!(i.withFileDrop)) return;
                i.withFileDrop.forEach(j => {
                    if (j.pattern && new RegExp(j.pattern).test(file)) {
                        if (((j?.folderRequired??false) === true && !isDir) || ((j?.fileRequired??false) === true && isDir)) {
                            return;
                        }
                        matchs[matchs.length] = {...i, key: `${i.cmd} ${j.parameters}`, withFileDrop: j};
                    }
                });
            });
            if (matchs.length === 0) {
                inputCmd.value = `"${file}"`;
                return;
            } else if (matchs.length === 1) {
                let i = matchs[0];
                API.cmdRunner(getCmdlineWithFile(i, file, i.withFileDrop)).then(_ => {
                    //
                });
            } else {
                filePath.value = file;
                matchList.clearChildren();
                matchs.forEach((i, iindex) => {
                    createItemElement(i, iindex, matchList, "match_list", () => getCmdlineWithFile(i, file, i.withFileDrop), (ie) => {
                        if (ie.classList.contains("disabled"))    return;
                        ie.classList.add("disabled");
                        API.cmdRunner(getCmdlineWithFile(i, file, i.withFileDrop)).then(_ => {
                            ie.classList.remove("disabled");
                        });
                    });
                });
                showWithFileDropCmd();
            }
        }
    });
}

function refreshCmd() {
    API.cfgEpoch().then(epoch => {
        if (cfgLMT === epoch)   return;
        API.cmdLoad().then(doRefreshCmd);
        cfgLMT = epoch;
        // console.log(new Date(cfgLMT).toLocaleString());
    });
}

window.addEventListener("DOMContentLoaded", () => {
    containerWithFileDropCmd = document.getElementById("container_with_file_drop");
    containerSet = document.getElementById("container_set");
    containerCmd = document.getElementById("container_cmd");
    inputTpath = document.getElementById("input_tpath");
    inputLpath = document.getElementById("input_lpath");
    inputHpath = document.getElementById("input_hpath");
    inputCmd = document.getElementById("input_cmd");
    buttonSave = document.getElementById("button_save");
    cmdList = document.getElementById("cmd_list");
    filePath = document.getElementById("file_path");
    matchList = document.getElementById("match_list");
    buttonBack = document.getElementById("button_back");

    buttonSave.onclick = async () => {
        if (await setSave()) {
            showCmd();
        }
    }

    inputCmd.onkeydown = (e) => {
        if (e.keyCode != 13) return;
        cmdInput();
    };

    buttonBack.onclick = async () => {
        showCmd();
    }

    loaded();
});

window.addEventListener("focus", () => {
    refreshCmd();
});

document.addEventListener("contextmenu", event => event.preventDefault());
