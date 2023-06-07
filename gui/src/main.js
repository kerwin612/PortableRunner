const { invoke } = window.__TAURI__.tauri;

let containerSet;
let containerCmd;
let inputTpath;
let inputLpath;
let inputHpath;
let buttonSave;
let inputCmd;
let cmdList;

async function cmdInput() {
  let cmdStr = inputCmd.value;
  inputCmd.value = null;
  await invoke("cmd_runner", { cmdStrs: cmdStr.trim().split(/\s+/) });
}

async function cmdClick(cmdStr) {
  await invoke("cmd_runner", { cmdStrs: cmdStr.trim().split(/\s+/) });
}

async function setLoad() {
    let data = await invoke("set_load");
    return data;
}

async function setSave() {
    let data = await invoke("set_save", {set: {tpath: inputTpath.value, lpath: inputLpath.value, hpath: inputHpath.value}});
    return data;
}

async function cmdLoad() {
    return await invoke("cmd_load");
}

async function loaded() {
    let s = await setLoad();
    if (!(s.tpath) || !(s.lpath)) {
        showSet();
    } else {
        // showCmd();
        inputTpath.value = s.tpath;
        inputLpath.value = s.lpath;
        inputHpath.value = s.hpath;
        if (await setSave()) {
            showCmd();
        }
    }
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

Element.prototype.insertChildAtIndex = function(child, index) {
    if (!index) index = 0;
    if (index >= this.children.length) {
        this.appendChild(child);
    } else {
        this.insertBefore(child, this.children[index]);
    }
}
function refreshCmd() {
    cmdLoad().then(list => {

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
                }
                ge.insertChildAtIndex(ie, iindex);

                ie.innerHTML = `<span>${i.label ? (i.label + '(' + i.key + ')') : i.key}</span>`;
                ie.onclick = (e) => {
                    if (i.parametersRequired) {
                        inputCmd.value = i.cmd + " ";
                        inputCmd.focus();
                    } else {
                        cmdClick(i.cmd);
                    }
                };

            });
        }

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
