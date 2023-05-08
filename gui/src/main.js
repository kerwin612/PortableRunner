const { invoke } = window.__TAURI__.tauri;

let containerSet;
let containerCmd;
let inputTpath;
let inputLpath;
let inputHpath;
let buttonSave;
let inputCmd;

async function cmd() {
  let cmdStr = inputCmd.value;
  inputCmd.value = null;
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
    containerSet.classList.add("hide");
    containerCmd.classList.remove("hide");
}

window.addEventListener("DOMContentLoaded", () => {
    containerSet = document.querySelector(".container_set");
    containerCmd = document.querySelector(".container_cmd");
    inputTpath = document.querySelector("#input_tpath");
    inputLpath = document.querySelector("#input_lpath");
    inputHpath = document.querySelector("#input_hpath");
    inputCmd = document.querySelector("#input_cmd");
    buttonSave = document.querySelector("#button_save");

    buttonSave.onclick = async () => {
        if (await setSave()) {
            showCmd();
        }
    }

    inputCmd.onkeydown = (e) => {
        if (e.keyCode != 13) return;
        cmd();
    };

    loaded();
});
