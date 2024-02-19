const { writeText } = window.__TAURI__.clipboard;

export function createItemElement(i, index, parent, idPrefix, getCMD, onClick) {
    let ii = `${idPrefix}_item_${i.cmd}`;
    let ie = document.getElementById(ii);
    if (ie == null) {
        ie = document.createElement("div");
        ie.classList.add("cmd_item");
        ie.setAttribute("id", ii);
        ie.setAttribute("style", i.style||"");
    }
    parent.insertChildAtIndex(ie, index);

    ie.innerHTML = `<span>${i.label ? (i.label + "(" + i.cmd + ")") : i.cmd}</span>`;
    ie.onclick = async (e) => {
        if (ie.classList.contains("disabled"))    return;
        ie.classList.add("disabled");
        await onClick(ie, e);
        ie.classList.remove("disabled");
    };
    ie.onmousedown = (e) => {
        let isRightMB;
        e = e || window.event;

        if ("which" in e)  // Gecko (Firefox), WebKit (Safari/Chrome) & Opera
            isRightMB = e.which == 3;
        else if ("button" in e)  // IE, Opera
            isRightMB = e.button == 2;

        if (isRightMB) {
            writeText(getCMD(i)).then(() => {
                //
            });
        }
    }
}

export function getCmdlineWithFile(i, file, fileDrop) {
    return `${i.run} ` + fileDrop.parameters.replaceAll("{0}", file);
}
