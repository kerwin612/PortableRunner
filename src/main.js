import './libs/extensions.js';
import * as API from './libs/apis.js';
import { InitPage } from './mods/init.js';
import { MainPage } from './mods/main.js';
import { TempPage } from './mods/temp.js';

let cfgLMT;
let initPage;
let mainPage;
let tempPage;

async function refreshCmd() {
    API.cfgEpoch().then(epoch => {
        if (cfgLMT === epoch)   return;
        API.cmdLoad().then(mainPage.doRefreshCmd);
        cfgLMT = epoch;
        // console.log(new Date(cfgLMT).toLocaleString());
    });
}

async function showSet() {
    document.body.setChild(initPage);
}

async function showCmd() {
    await refreshCmd();
    document.body.setChild(mainPage);
}

async function showWithFileDropCmd() {
    document.body.setChild(tempPage);
}

async function loaded() {
    initPage = InitPage({
        onSaveSuccess: async () => await showCmd()
    });

    mainPage = MainPage({
        onReloadRequired: async () => await refreshCmd(),
        onWithFile: async (file, matchs) => {
            tempPage.setData({ file, matchs });
            await showWithFileDropCmd();
        }
    });

    tempPage = TempPage({
        onArgumentsRequired: async (cmdValue) => {
            await mainPage.setData({ cmdValue });
            await showCmd();
        },
        onBackClick: async () => await showCmd()
    });

    API.setLoad().then(async s => {
        if (!(s.tpath) || !(s.lpath)) {
            await showSet();
        } else {
            // await showCmd();
            initPage.doSave({ tpath: s.tpath, lpath: s.lpath, hpath: s.hpath });
        }
    });
}

window.addEventListener("DOMContentLoaded", async () => {
    await loaded();
});

window.addEventListener("focus", async () => {
    await refreshCmd();
});

document.addEventListener("contextmenu", event => event.preventDefault());
