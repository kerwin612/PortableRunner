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

function refreshCmd() {
    cmdList.innerHTML = "";
    cmdLoad().then(list => {
        list.forEach(i => {
            let item = document.createElement("div");
            item.classList.add('cmd_item');
            item.setAttribute('id', 'cmd_item');
            item.innerHTML = `<span>${i.key}</span>`;
            item.onclick = (e) => {
                cmdClick(JSON.parse(i.cmd).toString());
            };
            cmdList.appendChild(item);
        });
    });
}

window.addEventListener("DOMContentLoaded", () => {
    containerSet = document.querySelector(".container_set");
    containerCmd = document.querySelector(".container_cmd");
    inputTpath = document.querySelector("#input_tpath");
    inputLpath = document.querySelector("#input_lpath");
    inputHpath = document.querySelector("#input_hpath");
    inputCmd = document.querySelector("#input_cmd");
    buttonSave = document.querySelector("#button_save");
    cmdList = document.querySelector("#cmd_list");

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
