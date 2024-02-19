
import * as API from '../libs/apis.js';
import { createItemElement, getCmdlineWithFile } from '../libs/utils.js';

const { listen } = window.__TAURI__.event;

const tmpl = `
    <div class="cmd_main">
        <input id="input_cmd" type="search" placeholder="cmd..." />
    </div>
    <div id="cmd_list" class="cmd_list">
    </div>
`;

export function MainPage({ onReloadRequired, onWithFile }) {
    let div = document.createElement("div");
    div.id = "container_cmd";
    div.innerHTML = tmpl;

    let inputCmd = div.querySelector("#input_cmd");
    let cmdList = div.querySelector("#cmd_list");
    let unListenFileDrop;

    function cmdInput() {
        let cmdStr = inputCmd.value.trim();
        inputCmd.value = null;
        if (cmdStr.isValidPath()) {
            div.runWithFile({ file: cmdStr });
        } else {
            API.cmdRunner(cmdStr);
        }
    }

    inputCmd.onkeydown = (e) => {
        if (e.keyCode != 13) return;
        cmdInput();
    };

    div.setData = function({ cmdValue }) {
        inputCmd.value = cmdValue;
        inputCmd.focus();
    }

    div.runWithFile = async function({ file }) {
        let isDir = await file.isDir();
        let matchs = []
        div.currentCmds.forEach(i => {
            if (!!!(i.withFile)) return;
            i.withFile.forEach(j => {
                if (j.pattern && new RegExp(j.pattern).test(file)) {
                    if (((j?.folderRequired??false) === true && !isDir) || ((j?.fileRequired??false) === true && isDir)) {
                        return;
                    }
                    matchs[matchs.length] = {...i, cmd: `${i.run} ${j.parameters}`, withFile: j};
                }
            });
        });
        if (matchs.length === 0) {
            div.setData({ cmdValue: `"${file}"` });
            return;
        } else if (matchs.length === 1) {
            let i = matchs[0];
            let cmdValue = getCmdlineWithFile(i, file, i.withFile);
            if (i.withFile.argumentsRequired) {
                div.setData({ cmdValue: `${cmdValue} ` });
                return;
            } else {
                await API.cmdRunner(cmdValue);
            }
        } else {
            onWithFile(file, matchs);
        }
    }

    div.doRefreshCmd = async function(list) {
        let groups = {};
        let items = {};
        (div.currentCmds = list).forEach(i => {
            let group = i.group??"default";
            let value = groups[group]||[];
            value[value.length] = i;
            groups[group] = value;
            items[i.cmd] = i;
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
            let ge = div.querySelector(gi);
            if (ge == null) {
                ge = document.createElement("div");
                ge.classList.add("cmd_sub_list");
                ge.setAttribute("id", gi);
            }
            cmdList.insertChildAtIndex(ge, gindex++);

            groups[group].forEach((i, iindex) => {
                createItemElement(i, iindex, ge, gi, () => i.run, async () => {
                    if (i.argumentsRequired) {
                        div.setData({ cmdValue: `${i.run} ` });
                    } else if (i.type === 'file') {
                        await div.runWithFile({ file: i.run });
                    } else {
                        await API.cmdRunner(i.run);
                    }
                })
            });
        }

        unListenFileDrop && unListenFileDrop();
        unListenFileDrop = await listen("tauri://file-drop", async event => {
            if ((event?.payload || []).length < 0)  return;
            let file = event.payload[0];
            if (file.endsWith('.lnk')) {
                API.readLnk(file).then(async (lnkInfo) => {
                    API.addShortcut({
                        cmd: (lnkInfo?.name ?? '') || await lnkInfo.target.getFileNameWithoutExt(),
                        run: `"${lnkInfo.target}" ${lnkInfo?.arguments ?? ''}`,
                    }).then(() => {
                        onReloadRequired();
                    });
                });
            } else {
                await div.runWithFile({ file });
            }
        });
    }

    return div;
}
