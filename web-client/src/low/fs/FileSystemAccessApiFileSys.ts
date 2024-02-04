import { console, allocErr, allocOk, wrapAsync } from "low/utils";
import { FileSys } from "./FileSys";
import { FsPath } from "./FsPath";
import { FsResult, FsResultCodes } from "./FsResult";

export const isFileSystemAccessApiSupported = (): boolean => {
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

    if (!window.FileSystemFileHandle.prototype.createWritable) {
        return false;
    }

    // @ts-expect-error window should have showDirectoryPicker
    if (!window.showDirectoryPicker) {
        return false;
    }

    return true;
};

type PermissionStatus = "granted" | "denied" | "prompt";

/// FileSys implementation that uses FileSystem Access API
/// This is only supported in Chrome/Edge
export class FileSystemAccessApiFileSys implements FileSys {
    private rootPath: string;
    private rootHandle: FileSystemDirectoryHandle;
    private permissionStatus: PermissionStatus;

    constructor(rootPath: string, rootHandle: FileSystemDirectoryHandle) {
        this.rootPath = rootPath;
        this.rootHandle = rootHandle;
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
            isFileSystemAccessApiSupported() &&
            this.permissionStatus === "granted"
        );
    }

    public isStale(): boolean {
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
    }

    async resolveDir(
        path: FsPath,
    ): Promise<FsResult<FileSystemDirectoryHandle>> {
        if (path.isRoot) {
            return allocOk(this.rootHandle);
        }

        const parentPathResult = path.parent;
        if (parentPathResult.isErr()) {
            return parentPathResult;
        }

        const parentDirResult = await this.resolveDir(parentPathResult.inner());
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

    public async readFile(path: FsPath): Promise<FsResult<File>> {
        const result = await this.resolveFile(path);
        if (result.isErr()) {
            return result;
        }
        try {
            const file = await result.inner().getFile();
            return result.makeOk(file);
        } catch (e) {
            console.error(e);
            return result.makeErr(FsResultCodes.Fail);
        }
    }

    public async writeFile(
        path: FsPath,
        content: string | Uint8Array,
    ): Promise<FsResult<void>> {
        const result = await this.resolveFile(path);

        if (result.isErr()) {
            return result;
        }
        try {
            const file = await result.inner().createWritable();
            await file.write(content);
            await file.close();
            return result.makeOk(undefined);
        } catch (e) {
            console.error(e);
            return result.makeErr(FsResultCodes.Fail);
        }
    }

    async resolveFile(path: FsPath): Promise<FsResult<FileSystemFileHandle>> {
        const parentDirResult = path.parent;
        if (parentDirResult.isErr()) {
            return parentDirResult;
        }

        const parentDirHandleResult = await this.resolveDir(
            parentDirResult.inner(),
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
            return result.makeOk(fileHandle);
        } catch (e) {
            console.error(e);
            return result.makeErr(FsResultCodes.Fail);
        }
    }
}
