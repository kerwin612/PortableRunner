import { invoke } from "@tauri-apps/api/core";
import { getMatches } from '@tauri-apps/plugin-cli';

export function readLnk(lnk: string) {
    return invoke("read_lnk", { lnk });
}

export function saveSet(set: any) {
    return invoke("set_mount", { set });
}

export function loadCmds() {
    return invoke<any[]>("load_cmds");
}

export function cfgEpoch() {
    return invoke<number>("get_cfg_epoch");
}

export function runCmd(cmdStr: string) {
    return invoke("run_cmd", { cmdStr });
}

export function addShortcut(shortcut: any) {
    return invoke("add_shortcut", { shortcut });
}

export function loadSet() {
    return new Promise(async (resolve) => {
        const matches = await getMatches();
        const args = matches.args;
        resolve({
            tpath: args?.tpath?.value ?? '',
            lpath: args?.lpath?.value ?? '',
            hpath: args?.hpath?.value ?? '',
        });
    });
}
