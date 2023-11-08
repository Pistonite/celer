import { console, allocErr, allocOk } from "low/utils";
import { FileSys } from "./FileSys";
import { FsPath } from "./FsPath";
import { FsResult, FsResultCodes } from "./FsResult";

export const isFileEntriesAPISupported = (): boolean => {
    if (!window) {
        return false;
    }
    // Chrome/Edge has this but it's named DirectoryEntry
    // However, it doesn't work properly.
    if (
        navigator &&
        navigator.userAgent &&
        navigator.userAgent.includes("Chrome")
    ) {
        return false;
    }
    if (!window.FileSystemDirectoryEntry) {
        return false;
    }

    if (!window.FileSystemFileEntry) {
        return false;
    }

    if (!window.FileSystemDirectoryEntry.prototype.createReader) {
        return false;
    }

    if (!window.FileSystemDirectoryEntry.prototype.getFile) {
        return false;
    }

    if (!window.FileSystemFileEntry.prototype.file) {
        return false;
    }

    return true;
};

/// FileSys implementation that uses File Entries API
/// This is not supported in Chrome/Edge, but in Firefox
export class FileEntriesAPIFileSys implements FileSys {
    private rootPath: string;
    private rootEntry: FileSystemDirectoryEntry;

    constructor(rootPath: string, rootEntry: FileSystemDirectoryEntry) {
        this.rootPath = rootPath;
        this.rootEntry = rootEntry;
    }

    public async init(): Promise<FsResult<void>> {
        return allocOk();
    }

    public isWritable(): boolean {
        // Entries API does not support writing
        return false;
    }

    public isStale(): boolean {
        // Entries API can scan directories
        return false;
    }

    public getRootName() {
        return this.rootPath;
    }

    public async listDir(path: FsPath): Promise<FsResult<string[]>> {
        const result = await this.resolveDir(path);
        if (result.isErr()) {
            return result;
        }
        const dirEntry = result.inner();

        try {
            const entries: FileSystemEntry[] = await new Promise(
                (resolve, reject) => {
                    dirEntry.createReader().readEntries(resolve, reject);
                },
            );
            const names = entries.map((e) => {
                if (e.isDirectory) {
                    return e.name + "/";
                }
                return e.name;
            });
            return result.makeOk(names);
        } catch (e) {
            console.error(e);
            return result.makeErr(FsResultCodes.Fail);
        }
    }

    // public async readFile(path: FsPath): Promise<FsResult<string>> {
    //     const result = await this.readFileAndModifiedTime(path);
    //     if (result.code !== FsResultCodes.Ok) {
    //         return result;
    //     }
    //     return setOkValue(result, result.value[0]);
    // }
    //
    // public async readFileAndModifiedTime(
    //     path: FsPath,
    // ): Promise<FsResult<[string, number]>> {
    //     const fileResult = await this.readFileInternal(path);
    //     if (fileResult.code !== FsResultCodes.Ok) {
    //         return fileResult;
    //     }
    //     const file = fileResult.value;
    //     const result = await decodeFile(file);
    //     if (result.code !== FsResultCodes.Ok) {
    //         return result;
    //     }
    //     return setOkValue(result, [result.value, file.lastModified]);
    // }
    //
    // public async readIfModified(
    //     path: FsPath,
    //     lastModified?: number,
    // ): Promise<FsResult<[string, number]>> {
    //     const fileResult = await this.readFileInternal(path);
    //     if (fileResult.code !== FsResultCodes.Ok) {
    //         return fileResult;
    //     }
    //     const file = fileResult.value;
    //     if (lastModified && file.lastModified <= lastModified) {
    //         return setErrValue(fileResult, FsResultCodes.NotModified);
    //     }
    //     const result = await decodeFile(file);
    //     if (result.code !== FsResultCodes.Ok) {
    //         return result;
    //     }
    //     return setOkValue(result, [result.value, file.lastModified]);
    // }
    //
    // public async readModifiedTime(path: FsPath): Promise<FsResult<number>> {
    //     const fileResult = await this.readFileInternal(path);
    //     if (fileResult.code !== FsResultCodes.Ok) {
    //         return fileResult;
    //     }
    //     return {
    //         code: FsResultCodes.Ok,
    //         value: fileResult.value.lastModified,
    //     };
    // }

    public async readFile(path: FsPath): Promise<FsResult<File>> {
        const parentResult = path.parent;
        if (parentResult.isErr()) {
            return parentResult;
        }
        const nameResult = path.name;
        if (nameResult.isErr()) {
            return nameResult;
        }
        const result = await this.resolveDir(parentResult.inner());
        if (result.isErr()) {
            return result;
        }
        const dirEntry = result.inner();

        try {
            const fileEntry = (await new Promise<FileSystemEntry>(
                (resolve, reject) => {
                    dirEntry.getFile(nameResult.inner(), {}, resolve, reject);
                },
            )) as FileSystemFileEntry;
            const file = await new Promise<File>((resolve, reject) => {
                fileEntry.file(resolve, reject);
            });
            return result.makeOk(file);
        } catch (e) {
            console.error(e);
            return result.makeErr(FsResultCodes.Fail);
        }
    }

    public async writeFile(
        _path: FsPath,
        _content: string | Uint8Array,
    ): Promise<FsResult<void>> {
        // Entries API does not support writing
        return allocErr(FsResultCodes.NotSupported);
    }

    async resolveDir(
        path: FsPath,
    ): Promise<FsResult<FileSystemDirectoryEntry>> {
        let entry: FileSystemEntry;
        if (path.isRoot) {
            entry = this.rootEntry;
        } else {
            const fullPath = path.path;
            try {
                entry = await new Promise((resolve, reject) => {
                    this.rootEntry.getDirectory(fullPath, {}, resolve, reject);
                });
            } catch (e) {
                console.error(e);
                return allocErr(FsResultCodes.Fail);
            }
        }

        if (!entry.isDirectory) {
            return allocErr(FsResultCodes.IsFile);
        }
        return allocOk(entry as FileSystemDirectoryEntry);
    }
}
