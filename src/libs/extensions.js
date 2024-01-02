const { readDir } = window.__TAURI__.fs;
const { basename, extname } = window.__TAURI__.path;

Element.prototype.clearChildren = function() {
    while (this.firstChild) {
        this.removeChild(this.firstChild);
    }
    return this;
}

Element.prototype.setChild = function(child) {
    this.clearChildren().appendChild(child);
    return this;
}

Element.prototype.insertChildAtIndex = function(child, index) {
    if (!index) index = 0;
    if (index >= this.children.length) {
        this.appendChild(child);
    } else {
        this.insertBefore(child, this.children[index]);
    }
}

String.prototype.isDir = async function() {
    try {
        let files = await readDir(this);
        return !!files;
    } catch (error) {}
    return false;
}

String.prototype.getFileNameWithoutExt = async function() {
    try {
        let namewithext = await basename(this);
        let ext = await extname(this);
        return namewithext.substring(0, namewithext.length - ext.length - 1);
    } catch (error) {}
    return null;
}
