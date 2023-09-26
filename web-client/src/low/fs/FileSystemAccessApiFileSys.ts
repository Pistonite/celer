import { allocErr, allocOk, wrapAsync } from "low/utils";
import { FileSys } from "./FileSys";
import { FsPath } from "./FsPath";
import {
    FsResult,
    // FsResultCode,
    FsResultCodes,
    // setErrValue,
    // setOkValue,
} from "./FsResult";
// import { decodeFile } from "./decode";

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

type PermissionStatus = "granted" | "denied" | "prompt";

/// FileSys implementation that uses FileSystem Access API
/// This is only supported in Chrome/Edge
export class FileSystemAccessAPIFileSys implements FileSys {
    private rootPath: string;
    private rootHandle: FileSystemDirectoryHandle;
    private permissionStatus: PermissionStatus;

    // private dirHandles: Record<string, FileSystemDirectoryHandle> = {};
    // private fileHandles: Record<string, FileSystemFileHandle> = {};

    constructor(rootPath: string, rootHandle: FileSystemDirectoryHandle) {
        this.rootPath = rootPath;
        this.rootHandle = rootHandle;
        // this.dirHandles = {
        //     "": rootHandle,
        // };
        // this.fileHandles = {};
        this.permissionStatus = "prompt";
    }

    public async init(): Promise<FsResult<void>> {
        // @ts-expect-error ts lib does not have requestPermission
        this.permissionStatus = await this.rootHandle.requestPermission({
            mode: "readwrite",
        });
        if (this.permissionStatus !== "granted") {
            return allocErr(FsResultCodes.PermissionDenied);
        }
        return allocOk();
    }

    public isWritable(): boolean {
        return (
            isFileSystemAccessAPISupported() &&
            this.permissionStatus === "granted"
        );
    }

    public getRootName() {
        return this.rootPath;
    }

    public async listDir(path: FsPath): Promise<FsResult<string[]>> {
        // return await retryIfCacheFailed(async (useCache: boolean) => {
        const result = await this.resolveDir(path);
        if (result.isErr()) {
            return result;
        }
        // if (result.code !== FsResultCodes.Ok) {
        //     return result;
        // }
        const dir = result.inner();
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
            return result.makeErr(FsResultCodes.Fail);
        }

        return result.makeOk(entries);

        // });
    }

    async resolveDir(
        path: FsPath,
        // useCache: boolean,
    ): Promise<FsResult<FileSystemDirectoryHandle>> {
        // const dirPath = path.path;
        // if (useCache) {
        //     if (dirPath in this.dirHandles) {
        //         return {
        //             code: FsResultCodes.Ok,
        //             value: this.dirHandles[dirPath],
        //         };
        //     }
        // } else {
        //     delete this.dirHandles[dirPath];
        // }

        if (path.isRoot) {
            return allocOk(this.rootHandle);
            // this.dirHandles[dirPath] = this.rootHandle;
            // return {
            //     code: FsResultCodes.Ok,
            //     value: this.dirHandles[dirPath],
            // };
        }

        const parentPathResult = path.parent;
        if (parentPathResult.isErr()) {
            return parentPathResult; //.propagate();
        }

        const parentDirResult = await this.resolveDir(
            parentPathResult.inner(),
            // useCache,
        );
        if (parentDirResult.isErr()) {
            return parentDirResult;
        }

        const parentDirHandle = parentDirResult.inner();
        const pathNameResult = path.name;
        if (pathNameResult.isErr()) {
            return pathNameResult;
        }

        const result = await wrapAsync(() => {
            return parentDirHandle.getDirectoryHandle(pathNameResult.inner());
        });
        if (result.isErr()) {
            console.error(result.inner());
            return result.makeErr(FsResultCodes.Fail);
        }

        return result;
    }

    // public async readFile(path: FsPath): Promise<FsResult<File>> {
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
    //     if (lastModified !== undefined && file.lastModified <= lastModified) {
    //         return setErrValue(fileResult, FsResultCodes.NotModified);
    //     }
    //     const result = await decodeFile(file);
    //     if (result.code !== FsResultCodes.Ok) {
    //         return result;
    //     }
    //     return setOkValue(result, [result.value, file.lastModified]);
    // }
    //
    // public async readFileAsBytes(path: FsPath): Promise<FsResult<Uint8Array>> {
    //     const fileResult = await this.readFileInternal(path);
    //     if (fileResult.code !== FsResultCodes.Ok) {
    //         return fileResult;
    //     }
    //     const file = fileResult.value;
    //     try {
    //         const buffer = await file.arrayBuffer();
    //         const array = new Uint8Array(buffer);
    //         return setOkValue(fileResult, array);
    //     } catch (e) {
    //         console.error(e);
    //         return setErrValue(fileResult, FsResultCodes.Fail);
    //     }
    // }

    public async readFile(path: FsPath): Promise<FsResult<File>> {
        // return await retryIfCacheFailed(async (useCache: boolean) => {
        const result = await this.resolveFile(path);
        if (result.isErr()) {
            return result;
        }
        // if (result.code !== FsResultCodes.Ok) {
        //     return result;
        // }
        try {
            const file = await result.inner().getFile();
            return result.makeOk(file);
        } catch (e) {
            console.error(e);
            return result.makeErr(FsResultCodes.Fail);
        }
        // });
    }

    public async writeFile(
        path: FsPath,
        content: string | Uint8Array,
    ): Promise<FsResult<void>> {
        // return await retryIfCacheFailed2(async (useCache: boolean) => {
        const result = await this.resolveFile(path);

        if (result.isErr()) {
            return result;
        }
        try {
            // @ts-expect-error FileSystemWritableFileStream is not in tslib
            const file: FileSystemWritableFileStream =
                // @ts-expect-error FileSystemFileHandle should have a createWritable() method
                await result.inner().createWritable();
            await file.write(content);
            await file.close();
            return result.makeOk(undefined);
        } catch (e) {
            console.error(e);
            return result.makeErr(FsResultCodes.Fail);
        }
        // });
    }

    async resolveFile(
        path: FsPath,
        // useCache: boolean,
    ): Promise<FsResult<FileSystemFileHandle>> {
        // const filePath = path.path;
        // if (useCache) {
        //     if (filePath in this.fileHandles) {
        //         return {
        //             code: FsResultCodes.Ok,
        //             value: this.fileHandles[filePath],
        //         };
        //     }
        // } else {
        //     delete this.fileHandles[filePath];
        // }

        const parentDirResult = path.parent;
        if (parentDirResult.isErr()) {
            return parentDirResult;
        }

        const parentDirHandleResult = await this.resolveDir(
            parentDirResult.inner(),
            // useCache,
        );
        if (parentDirHandleResult.isErr()) {
            return parentDirHandleResult;
        }

        const result = path.name;
        if (result.isErr()) {
            return result;
        }

        try {
            const fileHandle = await parentDirHandleResult
                .inner()
                .getFileHandle(result.inner());
            // this.fileHandles[filePath] = fileHandle;
            return result.makeOk(fileHandle);
        } catch (e) {
            console.error(e);
            return result.makeErr(FsResultCodes.Fail);
        }
    }
}

// const retryIfCacheFailed = async <T>(
//     func: (useCache: boolean) => Promise<FsResult<T>>,
// ): Promise<FsResult<T>> => {
//     const result = await func(true);
//     if (result.code !== FsResultCodes.Fail) {
//         return result;
//     }
//     return await func(false);
// };
//
// const retryIfCacheFailed2 = async (
//     func: (useCache: boolean) => Promise<FsResultCode>,
// ): Promise<FsResultCode> => {
//     const result = await func(true);
//     if (result !== FsResultCodes.Fail) {
//         return result;
//     }
//     return await func(false);
// };