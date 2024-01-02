const { invoke } = window.__TAURI__.tauri;

export function readLnk(lnk) {
    return invoke("read_lnk", { lnk });
}

export function setLoad() {
    return invoke("set_load");
}

export function setSave(set) {
    return invoke("set_save", { set });
}

export function cmdLoad() {
    return invoke("cmd_load");
}

export function cfgEpoch() {
    return invoke("cfg_epoch");
}

export function cmdRunner(cmdStr) {
    return invoke("cmd_runner", { cmdStr });
}
