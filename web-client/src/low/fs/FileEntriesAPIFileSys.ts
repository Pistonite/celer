import { FileSys } from "./FileSys";
import { FsPath } from "./FsPath";
import { FsResult, FsResultCode } from "./FsResult";
import { decodeFile } from "./decode";

export const isFileEntriesAPISupported = (): boolean => {
    if (!window) {
        return false;
    }
    // Chrome/Edge has this but it's named DirectoryEntry
    // However, it doesn't work properly.
    if (navigator && navigator.userAgent && navigator.userAgent.includes("Chrome")) {
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
}

/// FileSys implementation that uses File Entries API
/// This is not supported in Chrome/Edge, but in Firefox
export class FileEntiresAPIFileSys implements FileSys {
    private rootPath: string;
    private rootEntry: FileSystemDirectoryEntry;

    constructor(rootPath: string, rootEntry: FileSystemDirectoryEntry) {
        this.rootPath = rootPath;
        this.rootEntry = rootEntry;
    }

    public isWritable(): boolean {
        // Entries API does not support writing
        return false;
    }

    public getRootName() {
        return this.rootPath;
    }

    public async listDir(path: FsPath): Promise<FsResult<string[]>> {
        const dirEntry = await this.resolveDir(path);

        try {
            const entries: FileSystemEntry[] = await new Promise((resolve, reject) => {
                // @ts-expect-error FileSystemDirectoryEntry should have a createReader() method
                dirEntry.createReader().readEntries(resolve, reject);
            });
            return {
                code: FsResultCode.Ok,
                value: entries.map(e => e.name),
            }
        } catch (e) {
            return {
                code: FsResultCode.Fail,
            };
        }
    }

    public async readFile(path: FsPath): Promise<FsResult<string>> {
        const parentResult = path.parent;
        if (parentResult.code !== FsResultCode.Ok) {
            return parentResult;
        }
        const dirEntry = await this.resolveDir(parentResult.value);

        try {
            const fileEntry = await new Promise<FileSystemEntry>((resolve, reject) => {
                // @ts-expect-error FileSystemDirectoryEntry should have a getFile() method
                dirEntry.getFile(path.name, {}, resolve, reject);
            });
            const file = await new Promise<File>((resolve, reject) => {
                // @ts-expect-error FileSystemFileEntry should have a file() method
                fileEntry.file(resolve, reject);
            });
            return await decodeFile(file);
        } catch (e) {
            return {
                code: FsResultCode.Fail,
            };
        }
    }

    public async writeFile(_path: FsPath, _content: string): Promise<FsResult<never>> {
        // Entries API does not support writing
        return {
            code: FsResultCode.NotSupported,
        };
    }

    async resolveDir(path: FsPath): Promise<FsResult<FileSystemDirectoryEntry>> {
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
                return {
                    code: FsResultCode.Fail,
                };
            }
        }

        if (!entry.isDirectory || !("createReader" in entry)) {
            return {
                code: FsResultCode.Fail,
            };
        }

        return {
            code: FsResultCode.Ok,
            value: entry as FileSystemDirectoryEntry,
        };
    }
}
