import { invoke } from "@tauri-apps/api/core";

export function readLnk(lnk: string) {
    return invoke("read_lnk", { lnk });
}

export function loadSet() {
    return invoke("load_set");
}

export function saveSet(set: any) {
    return invoke("save_set", { set });
}

export function loadCmds() {
    return invoke<any[]>("load_cmds");
}

export function cfgEpoch() {
    return invoke<number>("cfg_epoch");
}

export function runCmd(cmdStr: string) {
    return invoke("run_cmd", { cmdStr });
}

export function addShortcut(shortcut: any) {
    return invoke("add_shortcut", { shortcut });
}
