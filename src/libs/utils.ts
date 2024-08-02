import { readDir } from "@tauri-apps/plugin-fs";
import { basename, extname } from "@tauri-apps/api/path";

export function isValidPath(path: string) {
    return /^(?:(?:[a-zA-Z]:[\\/])(?:[^<>:"|?*\r\n]+\\)*[^<>:"|?*\r\n]+$)|(?:\\\\?\\(?:[^<>:"|?*\r\n]+\\)*[^<>:"|?*\r\n]+$)|(?:[\\/])(?:[^<>:"|?*\r\n]+\\)*[^<>:"|?*\r\n]+$/.test(path);
}

export async function isDirPath(path: string): Promise<boolean> {
    try {
        let files = await readDir(path);
        return !!files;
    } catch (error) {
        console.warn(error);
    }
    return false;
}

export async function getFileNameWithoutExt(path: string): Promise<string | null> {
    try {
        let namewithext = await basename(path);
        let ext = await extname(path);
        return namewithext.substring(0, namewithext.length - ext.length - 1);
    } catch (error) {
        console.warn(error);
    }
    return null;
}

export function getCmdlineWithFile(i: any, file: string, fileDrop: any) {
    return `${i.run} ` + fileDrop.parameters.replaceAll("{0}", file);
}
