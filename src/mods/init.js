
import * as API from '../libs/apis.js';

const tmpl = `
<div class="set_main">
    <input id="input_tpath" placeholder="Target Path" />
    <input id="input_lpath" placeholder="Link Path" />
    <input id="input_hpath" placeholder="Home Path" />
    <button id="button_save">Save</button>
</div>
<div class="set_help">
    <p>
        <b>PortableRunner [Target Path] [Link Path] [Home Path]</b><br/>
        &nbsp;&nbsp;&nbsp;&nbsp;args:<br/>
        &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<b>Target Path</b>: Specifies the physical drive and path that you want to assign to a virtual drive.<br/>
        &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<b>Link Path</b>: Specifies the virtual drive and path to which you want to link the [Target Path].<br/>
        &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<b>Home Path</b>: The subdirectory of the [Link Path], will be specified as the value of %HOME%, which defaults to [.home].<br/>
    </p>
</div>
`;

export function InitPage({ onSaveSuccess }) {
    let div = document.createElement("div");
    div.id = "container_set";
    div.innerHTML = tmpl;

    let inputTpath = div.querySelector("#input_tpath");
    let inputLpath = div.querySelector("#input_lpath");
    let inputHpath = div.querySelector("#input_hpath");
    let buttonSave = div.querySelector("#button_save");

    function setSave() {
        return API.setSave({tpath: inputTpath.value, lpath: inputLpath.value, hpath: (inputHpath.value ?? "").trim() || ".home"});
    }

    buttonSave.onclick = async () => {
        if (await setSave()) {
            onSaveSuccess();
        }
    }

    div.doSave = async function({ tpath, lpath, hpath }) {
        inputTpath.value = tpath;
        inputLpath.value = lpath;
        inputHpath.value = hpath;
        if (await setSave()) {
            onSaveSuccess();
        }
    }

    return div;
}
