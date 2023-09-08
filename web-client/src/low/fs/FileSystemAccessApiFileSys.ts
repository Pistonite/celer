
import { FileSys } from "./FileSys";
import { FsPath } from "./FsPath";
import { FsOkResult, FsResult, FsResultCode } from "./FsResult";
import { decodeFile } from "./decode";

export const isFileSystemAccessAPISupported = (): boolean => {
    if (!window) {
        return false;
    }
    if (!window.isSecureContext) {
        // In Chrome, you can still access the APIs but they just crash the page entirely
        console.warn("FileSystemAccessAPI is only available in secure context");
        return false;
    }
    if (!window.FileSystemDirectoryHandle) {
        return false;
    }

    if (!window.FileSystemFileHandle) {
        return false;
    }

    // @ts-expect-error FileSystemDirectoryHandle should have a values() method
    if (!window.FileSystemDirectoryHandle.prototype.values) {
        return false;
    }

    // @ts-expect-error FileSystemFileHandle should have a createWritable() method
    if (!window.FileSystemFileHandle.prototype.createWritable) {
        return false;
    }

    return true;
}

/// FileSys implementation that uses FileSystem Access API
/// This is only supported in Chrome/Edge
export class FileSystemAccessAPIFileSys implements FileSys {
    private rootPath: string;

    private dirHandles: Record<string, FileSystemDirectoryHandle> = {};
    private fileHandles: Record<string, FileSystemFileHandle> = {};

    constructor(rootPath: string, rootHandle: FileSystemDirectoryHandle) {
        this.rootPath = rootPath;
        this.dirHandles = {
            "": rootHandle,
        };
        this.fileHandles = {};
    }

    public isWritable(): boolean {
        return isFileSystemAccessAPISupported();
    }

    public getRootName() {
        return this.rootPath;
    }

    public async listDir(path: FsPath): Promise<FsResult<string[]>> {
        const result = await this.listDirInternal(path, true);
        if (result.code === FsResultCode.Fail) {
            // If FS operation fails, try again without cache
            return await this.listDirInternal(path, false);
        }
        return result;
    }

    async listDirInternal(path: FsPath, useCache: boolean): Promise<FsResult<string[]>> {
        const dirHandle = await this.resolveDir(path, useCache);
        if (dirHandle.code !== FsResultCode.Ok) {
            return dirHandle;
        }
        const dir = dirHandle.value;
        const result: string[] = [];
        
        try {
            // @ts-expect-error FileSystemDirectoryHandle should have a values() method
            for await (const entry of dir.values()) {
                if (entry.kind === "directory") {
                    result.push(entry.name + "/");
                } else {
                    result.push(entry.name);
                }
            }
        } catch (e) {
            console.error(e);
            return {
                code: FsResultCode.Fail,
            };
        }

        return {
            code: FsResultCode.Ok,
            value: result,
        };
    }

    async resolveDir(path: FsPath, useCache: boolean): Promise<FsResult<FileSystemDirectoryHandle>> {
        const dirPath = path.path;
        if (useCache) {
            if (dirPath in this.dirHandles) {
                return {
                    code: FsResultCode.Ok,
                    value: this.dirHandles[dirPath],
                };
            }
        }
            const parentPath = path.parent;
            if (parentPath.code !== FsResultCode.Ok) {
                return parentPath;
            }
            const parentDir = await this.resolveDir(parentPath.value, useCache);
            if (parentDir.code !== FsResultCode.Ok) {
                return parentDir;
            }
            const parentDirHandle = parentDir.value;
            const dirName = path.name;
            if (dirName.code !== FsResultCode.Ok) {
                return dirName;
            }
            try {
                const dirHandle = await parentDirHandle.getDirectoryHandle(dirName.value);
                this.dirHandles[dirPath] = dirHandle;
                return {
                    code: FsResultCode.Ok,
                    value: dirHandle,
                }
            } catch (e) {
                console.error(e);
                return {
                    code: FsResultCode.Fail,
                };
            }
    }

    public async readFile(path: FsPath): Promise<FsResult<string>> {
        const result = await this.readFileInternal(path, true);
        if (result.code === FsResultCode.Fail) {
            // If FS operation fails, try again without cache
            return await this.readFileInternal(path, false);
        }
        return result;
    }

    async readFileInternal(path: FsPath, useCache: boolean): Promise<FsResult<string>> {
        const fileHandleResult = await this.resolveFile(path, useCache);
        if (fileHandleResult.code !== FsResultCode.Ok) {
            return fileHandleResult;
        }
        try {
            const file = await fileHandleResult.value.getFile();
            return await decodeFile(file);
        } catch (e) {
            console.error(e);
            return {
                code: FsResultCode.Fail,
            };
        }
    }

    public async writeFile(path: FsPath, content: string): Promise<FsResult<never>> {
        const result = await this.writeFileInternal(path, content, true);
        if (result.code === FsResultCode.Fail) {
            // If FS operation fails, try again without cache
            return await this.writeFileInternal(path, content, false);
        }
        return result;
    }

    async writeFileInternal(path: FsPath, content: string, useCache: boolean): Promise<FsResult<never>> {
        const fileHandleResult = await this.resolveFile(path, useCache);
        if (fileHandleResult.code !== FsResultCode.Ok) {
            return fileHandleResult;
        }
        try {
            // @ts-expect-error FileSystemFileHandle should have a createWritable() method
            const file: FileSystemWritableFileStream = await fileHandleResult.value.createWritable();
            await file.write(content);
            await file.close();
            return {
                code: FsResultCode.Ok,
            } as FsOkResult<never>;
        } catch (e) {
            console.error(e);
            return {
                code: FsResultCode.Fail,
            };
        }
    }

    async resolveFile(path: FsPath, useCache: boolean): Promise<FsResult<FileSystemFileHandle>> {
        const filePath = path.path;
        if (useCache) {
            if (filePath in this.fileHandles) {
                return {
                    code: FsResultCode.Ok,
                    value: this.fileHandles[filePath],
                };
            }
        }
        const parentDir = path.parent;
        if (parentDir.code !== FsResultCode.Ok) {
            return parentDir;
        }
        const parentDirHandle = await this.resolveDir(parentDir.value, useCache);
        if (parentDirHandle.code !== FsResultCode.Ok) {
            return parentDirHandle;
        }
        const fileName = path.name;
        if (fileName.code !== FsResultCode.Ok) {
            return fileName;
        }
        try {
            const fileHandle = await parentDirHandle.value.getFileHandle(fileName.value);
            this.fileHandles[filePath] = fileHandle;
            return {
                code: FsResultCode.Ok,
                value: fileHandle,
            };
        } catch (e) {
            console.error(e);
            return {
                code: FsResultCode.Fail,
            };
        }
    }

}

