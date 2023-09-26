import { FsPath, fsRootPath } from "low/fs";
import { Logger } from "low/utils";

export const EditorLog = new Logger("edt");

export const EditorContainerId = "editor-container";

export const toFsPath = (path: string[]): FsPath => {
    let fsPath = fsRootPath;
    for (let i = 0; i < path.length; i++) {
        fsPath = fsPath.resolve(path[i]);
    }
    return fsPath;
};

export const detectLanguageByFileName = (fileName: string): string => {
    if (fileName.match(/\.(j|t)s$/i)) {
        return "typescript";
    }
    if (fileName.match(/\.ya?ml/i)) {
        return "yaml";
    }
    if (fileName.match(/\.json/i)) {
        return "json";
    }
    return "text";
};
