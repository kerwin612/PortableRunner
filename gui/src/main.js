const { invoke } = window.__TAURI__.tauri;
const { writeText } = window.__TAURI__.clipboard;

let containerSet;
let containerCmd;
let inputTpath;
let inputLpath;
let inputHpath;
let buttonSave;
let inputCmd;
let cmdList;
let cfgLMT;

Element.prototype.insertChildAtIndex = function(child, index) {
    if (!index) index = 0;
    if (index >= this.children.length) {
        this.appendChild(child);
    } else {
        this.insertBefore(child, this.children[index]);
    }
}

function setLoad() {
    return invoke("set_load");
}

function setSave() {
    return invoke("set_save", {set: {tpath: inputTpath.value, lpath: inputLpath.value, hpath: inputHpath.value}});
}

function cfgEpoch() {
    return invoke("cfg_epoch");
}

function cmdLoad() {
    return invoke("cmd_load");
}

function cmdInput() {
  let cmdStr = inputCmd.value;
  inputCmd.value = null;
  return invoke("cmd_runner", { cmdStrs: cmdStr.trim().split(/\s+/) });
}

function cmdClick(cmdStr) {
  return invoke("cmd_runner", { cmdStrs: cmdStr.trim().split(/\s+/) });
}

function loaded() {
    setLoad().then(s => {
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

function showSet() {
    containerCmd.classList.add("hide");
    containerSet.classList.remove("hide");
}

function showCmd() {
    refreshCmd();
    containerSet.classList.add("hide");
    containerCmd.classList.remove("hide");
}

function doRefreshCmd(list) {
    let groups = {};
    let items = {};
    list.forEach(i => {
        let group = i.group??'default';
        let value = groups[group]||[];
        value[value.length] = i;
        groups[group] = value;
        items[i.key] = i;
    });

    cmdList.querySelectorAll(`.cmd_sub_list`).forEach(g => {
        if (!(groups[g.id.substring(6)])) {
            g.remove();
        } else {
            g.querySelectorAll('.cmd_item').forEach(i => {
                if (!(items[i.id.substring(5)]) || (i.parentElement.id.substring(6) !== (items[i.id.substring(5)].group??'default'))) {
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
            ge.classList.add('cmd_sub_list');
            ge.setAttribute('id', gi);
        }
        cmdList.insertChildAtIndex(ge, gindex++);

        groups[group].forEach((i, iindex) => {

            let ii = `item_${i.key}`;
            let ie = document.getElementById(ii);
            if (ie == null) {
                ie = document.createElement("div");
                ie.classList.add('cmd_item');
                ie.setAttribute('id', ii);
                ie.setAttribute('style', i.style||'');
            }
            ge.insertChildAtIndex(ie, iindex);

            ie.innerHTML = `<span>${i.label ? (i.label + '(' + i.key + ')') : i.key}</span>`;
            ie.onclick = (e) => {
                if (ie.classList.contains('disabled'))    return;
                ie.classList.add("disabled");
                if (i.parametersRequired) {
                    inputCmd.value = i.cmd + " ";
                    inputCmd.focus();
                    ie.classList.remove("disabled");
                } else {
                    cmdClick(i.cmd).then(_ => {
                        ie.classList.remove("disabled");
                    });
                }
            };
            ie.onmousedown = (e) => {
                let isRightMB;
                e = e || window.event;

                if ("which" in e)  // Gecko (Firefox), WebKit (Safari/Chrome) & Opera
                    isRightMB = e.which == 3;
                else if ("button" in e)  // IE, Opera
                    isRightMB = e.button == 2;

                if (isRightMB) {
                    writeText(i.cmd).then(_ => {
                        //
                    });
                }
            }

        });
    }
}

function refreshCmd() {
    cfgEpoch().then(epoch => {
        if (cfgLMT === epoch)   return;
        cmdLoad().then(doRefreshCmd);
        cfgLMT = epoch;
        // console.log(new Date(cfgLMT).toLocaleString());
    });
}

window.addEventListener("DOMContentLoaded", () => {
    containerSet = document.getElementById("container_set");
    containerCmd = document.getElementById("container_cmd");
    inputTpath = document.getElementById("input_tpath");
    inputLpath = document.getElementById("input_lpath");
    inputHpath = document.getElementById("input_hpath");
    inputCmd = document.getElementById("input_cmd");
    buttonSave = document.getElementById("button_save");
    cmdList = document.getElementById("cmd_list");

    buttonSave.onclick = async () => {
        if (await setSave()) {
            showCmd();
        }
    }

    inputCmd.onkeydown = (e) => {
        if (e.keyCode != 13) return;
        cmdInput();
    };

    loaded();
});

window.addEventListener('focus', () => {
    refreshCmd();
});

document.addEventListener('contextmenu', event => event.preventDefault());
