import { FileSys } from "./FileSys";
import { FsPath } from "./FsPath";
import {
    FsResult,
    FsResultCode,
    FsResultCodes,
    setErrValue,
    setOkValue,
} from "./FsResult";
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
};

/// FileSys implementation that uses FileSystem Access API
/// This is only supported in Chrome/Edge
export class FileSystemAccessAPIFileSys implements FileSys {
    private rootPath: string;
    private rootHandle: FileSystemDirectoryHandle;

    private dirHandles: Record<string, FileSystemDirectoryHandle> = {};
    private fileHandles: Record<string, FileSystemFileHandle> = {};

    constructor(rootPath: string, rootHandle: FileSystemDirectoryHandle) {
        this.rootPath = rootPath;
        this.rootHandle = rootHandle;
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
        return await retryIfCacheFailed(async (useCache: boolean) => {
            const result = await this.resolveDir(path, useCache);
            if (result.code !== FsResultCodes.Ok) {
                return result;
            }
            const dir = result.value;
            const entries: string[] = [];

            try {
                // @ts-expect-error FileSystemDirectoryHandle should have a values() method
                for await (const entry of dir.values()) {
                    if (entry.kind === "directory") {
                        entries.push(entry.name + "/");
                    } else {
                        entries.push(entry.name);
                    }
                }
            } catch (e) {
                console.error(e);
                return setErrValue(result, FsResultCodes.Fail);
            }

            return setOkValue(result, entries);
        });
    }

    async resolveDir(
        path: FsPath,
        useCache: boolean,
    ): Promise<FsResult<FileSystemDirectoryHandle>> {
        const dirPath = path.path;
        if (useCache) {
            if (dirPath in this.dirHandles) {
                return {
                    code: FsResultCodes.Ok,
                    value: this.dirHandles[dirPath],
                };
            }
        } else {
            delete this.dirHandles[dirPath];
        }

        if (path.isRoot) {
            this.dirHandles[dirPath] = this.rootHandle;
            return {
                code: FsResultCodes.Ok,
                value: this.dirHandles[dirPath],
            };
        }

        const parentPathResult = path.parent;
        if (parentPathResult.code !== FsResultCodes.Ok) {
            return parentPathResult;
        }
        const parentDirResult = await this.resolveDir(
            parentPathResult.value,
            useCache,
        );
        if (parentDirResult.code !== FsResultCodes.Ok) {
            return parentDirResult;
        }
        const parentDirHandle = parentDirResult.value;
        const result = path.name;
        if (result.code !== FsResultCodes.Ok) {
            return result;
        }

        try {
            const dirHandle = await parentDirHandle.getDirectoryHandle(
                result.value,
            );
            this.dirHandles[dirPath] = dirHandle;
            return setOkValue(result, dirHandle);
        } catch (e) {
            console.error(e);
            return setErrValue(result, FsResultCodes.Fail);
        }
    }

    public async readFile(path: FsPath): Promise<FsResult<string>> {
        const result = await this.readFileAndModifiedTime(path);
        if (result.code !== FsResultCodes.Ok) {
            return result;
        }
        return setOkValue(result, result.value[0]);
    }

    public async readFileAndModifiedTime(
        path: FsPath,
    ): Promise<FsResult<[string, number]>> {
        const fileResult = await this.readFileInternal(path);
        if (fileResult.code !== FsResultCodes.Ok) {
            return fileResult;
        }
        const file = fileResult.value;
        const result = await decodeFile(file);
        if (result.code !== FsResultCodes.Ok) {
            return result;
        }
        return setOkValue(result, [result.value, file.lastModified]);
    }

    public async readIfModified(
        path: FsPath,
        lastModified?: number,
    ): Promise<FsResult<[string, number]>> {
        const fileResult = await this.readFileInternal(path);
        if (fileResult.code !== FsResultCodes.Ok) {
            return fileResult;
        }
        const file = fileResult.value;
        if (lastModified !== undefined && file.lastModified <= lastModified) {
            return setErrValue(fileResult, FsResultCodes.NotModified);
        }
        const result = await decodeFile(file);
        if (result.code !== FsResultCodes.Ok) {
            return result;
        }
        return setOkValue(result, [result.value, file.lastModified]);
    }

    async readFileInternal(path: FsPath): Promise<FsResult<File>> {
        return await retryIfCacheFailed(async (useCache: boolean) => {
            const result = await this.resolveFile(path, useCache);
            if (result.code !== FsResultCodes.Ok) {
                return result;
            }
            try {
                const file = await result.value.getFile();
                return setOkValue(result, file);
            } catch (e) {
                console.error(e);
                return setErrValue(result, FsResultCodes.Fail);
            }
        });
    }

    public async writeFile(
        path: FsPath,
        content: string,
    ): Promise<FsResultCode> {
        return await retryIfCacheFailed2(async (useCache: boolean) => {
            const fileHandleResult = await this.resolveFile(path, useCache);
            if (fileHandleResult.code !== FsResultCodes.Ok) {
                return fileHandleResult.code;
            }
            try {
                // @ts-expect-error FileSystemFileHandle should have a createWritable() method
                const file: FileSystemWritableFileStream =
                    await fileHandleResult.value.createWritable();
                await file.write(content);
                await file.close();
                return FsResultCodes.Ok;
            } catch (e) {
                console.error(e);
                return FsResultCodes.Fail;
            }
        });
    }

    async resolveFile(
        path: FsPath,
        useCache: boolean,
    ): Promise<FsResult<FileSystemFileHandle>> {
        const filePath = path.path;
        if (useCache) {
            if (filePath in this.fileHandles) {
                return {
                    code: FsResultCodes.Ok,
                    value: this.fileHandles[filePath],
                };
            }
        } else {
            delete this.fileHandles[filePath];
        }

        const parentDirResult = path.parent;
        if (parentDirResult.code !== FsResultCodes.Ok) {
            return parentDirResult;
        }

        const parentDirHandleResult = await this.resolveDir(
            parentDirResult.value,
            useCache,
        );
        if (parentDirHandleResult.code !== FsResultCodes.Ok) {
            return parentDirHandleResult;
        }

        const result = path.name;
        if (result.code !== FsResultCodes.Ok) {
            return result;
        }

        try {
            const fileHandle = await parentDirHandleResult.value.getFileHandle(
                result.value,
            );
            this.fileHandles[filePath] = fileHandle;
            return setOkValue(result, fileHandle);
        } catch (e) {
            console.error(e);
            return setErrValue(result, FsResultCodes.Fail);
        }
    }
}

const retryIfCacheFailed = async <T>(
    func: (useCache: boolean) => Promise<FsResult<T>>,
): Promise<FsResult<T>> => {
    const result = await func(true);
    if (result.code !== FsResultCodes.Fail) {
        return result;
    }
    return await func(false);
};

const retryIfCacheFailed2 = async (
    func: (useCache: boolean) => Promise<FsResultCode>,
): Promise<FsResultCode> => {
    const result = await func(true);
    if (result !== FsResultCodes.Fail) {
        return result;
    }
    return await func(false);
};
