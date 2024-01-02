
import * as API from '../libs/apis.js';
import { createItemElement, getCmdlineWithFile } from '../libs/utils.js';

const tmpl = `
<div class="cmd_main">
    <input id="file_path" type="search" readonly/>
    <button id="button_back">Back</button>
</div>
<div id="match_list" class="cmd_list">
</div>
`;

export function TempPage({ onArgumentsRequired, onBackClick }) {
    let div = document.createElement("div");
    div.id = "container_with_file_drop";
    div.innerHTML = tmpl;

    let filePath = div.querySelector("#file_path");
    let matchList = div.querySelector("#match_list");
    let buttonBack = div.querySelector("#button_back");

    div.setData = function({ file, matchs }) {
        filePath.value = file;
        matchList.clearChildren();
        matchs.forEach((i, iindex) => {
            let cmd = getCmdlineWithFile(i, file, i.withFileDrop);
            createItemElement(i, iindex, matchList, "match_list", () => cmd, async () => {
                if (i.withFileDrop.argumentsRequired) {
                    await onArgumentsRequired(cmd);
                } else {
                    await API.cmdRunner(cmd);
                }
            });
        });
    }

    buttonBack.onclick = onBackClick;

    return div;
}
