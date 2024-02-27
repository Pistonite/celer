//! FsFileSystem implementation for FileSystemAccess API

import { tryAsync } from "pure/result";
import { errstr } from "pure/utils";

import {
    FsFileSystem,
    FsFileSystemUninit,
    FsCapabilities,
} from "./FsFileSystem.ts";
import { FsErr, FsResult, FsVoid, fsErr, fsFail } from "./FsError.ts";
import { FsFile } from "./FsFile.ts";
import {
    fsComponents,
    fsGetBase,
    fsGetName,
    fsIsRoot,
    fsNormalize,
} from "./FsPath.ts";
import { FsFileMgr } from "./FsFileMgr.ts";
import { FsFileSystemInternal } from "./FsFileSystemInternal.ts";

type PermissionStatus = "granted" | "denied" | "prompt";

/// FsFileSystem implementation that uses FileSystem Access API
/// This is only supported in Chrome/Edge
export class FsImplHandleAPI
    implements FsFileSystemUninit, FsFileSystem, FsFileSystemInternal
{
    public root: string;
    public capabilities: FsCapabilities;
    /// If app requested write access
    private writeMode: boolean;
    private rootHandle: FileSystemDirectoryHandle;
    private permissionStatus: PermissionStatus;

    private mgr: FsFileMgr;

    constructor(
        rootPath: string,
        rootHandle: FileSystemDirectoryHandle,
        write: boolean,
    ) {
        this.root = rootPath;
        this.rootHandle = rootHandle;
        this.writeMode = write;
        this.permissionStatus = "prompt";
        this.capabilities = {
            write,
            live: true,
        };
        this.mgr = new FsFileMgr();
    }

    public async init(): Promise<FsResult<FsFileSystem>> {
        // @ts-expect-error ts lib does not have requestPermission
        this.permissionStatus = await this.rootHandle.requestPermission({
            mode: this.writeMode ? "readwrite" : "read",
        });
        if (this.permissionStatus !== "granted") {
            const err = fsErr(FsErr.PermissionDenied, "User denied permission");
            return { err };
        }
        return { val: this };
    }

    public async listDir(path: string): Promise<FsResult<string[]>> {
        const normalized = fsNormalize(path);
        if (normalized.err) {
            return normalized;
        }
        path = normalized.val;

        const handle = await this.resolveDir(path);
        if (handle.err) {
            return handle;
        }

        const entries = await tryAsync(async () => {
            const entries: string[] = [];
            // @ts-expect-error ts lib does not have values()
            for await (const entry of handle.val.values()) {
                const { kind, name } = entry;
                if (kind === "directory") {
                    entries.push(name + "/");
                } else {
                    entries.push(name);
                }
            }
            return entries;
        });
        if ("err" in entries) {
            const err = fsFail(
                "Error reading entries from directory `" +
                    path +
                    "`: " +
                    errstr(entries.err),
            );
            return { err };
        }
        return entries;
    }

    public async read(path: string): Promise<FsResult<File>> {
        const normalized = fsNormalize(path);
        if (normalized.err) {
            return normalized;
        }
        path = normalized.val;

        const handle = await this.resolveFile(path);
        if (handle.err) {
            return handle;
        }

        const file = await tryAsync(() => handle.val.getFile());
        if ("err" in file) {
            const err = fsFail(
                "Failed to read file `" + path + "`: " + errstr(file.err),
            );
            return { err };
        }
        return file;
    }

    public async write(path: string, content: Uint8Array): Promise<FsVoid> {
        if (!this.writeMode) {
            const err = fsErr(
                FsErr.PermissionDenied,
                "Write mode not requested",
            );
            return { err };
        }
        const normalized = fsNormalize(path);
        if (normalized.err) {
            return normalized;
        }
        path = normalized.val;

        const handle = await this.resolveFile(path);
        if (handle.err) {
            return handle;
        }

        const result = await tryAsync(async () => {
            const file = await handle.val.createWritable();
            await file.write(content);
            await file.close();
            return {};
        });
        if ("err" in result) {
            const err = fsFail(
                "Failed to write file `" + path + "`: " + errstr(result.err),
            );
            return { err };
        }
        return {};
    }

    public getFile(path: string): FsFile {
        return this.mgr.get(this, path);
    }

    public getOpenedPaths(): string[] {
        return this.mgr.getOpenedPaths();
    }
    public closeFile(path: string): void {
        this.mgr.close(path);
    }

    /// Resolve the FileSystemDirectoryHandle for a directory.
    /// The path must be normalized
    private async resolveDir(
        path: string,
    ): Promise<FsResult<FileSystemDirectoryHandle>> {
        if (fsIsRoot(path)) {
            return { val: this.rootHandle };
        }
        let handle: FileSystemDirectoryHandle = this.rootHandle;
        const parts: string[] = [];
        for (const part of fsComponents(path)) {
            parts.push(part);
            const next = await tryAsync(() => handle.getDirectoryHandle(part));
            if ("err" in next) {
                const dir = parts.join("/");
                const err = fsFail(
                    "Failed to resolve directory `" +
                        dir +
                        "`: " +
                        errstr(next.err),
                );
                return { err };
            }
            handle = next.val;
        }

        return { val: handle };
    }

    /// Resolve the FileSystemFileHandle for a file.
    /// The path must be normalized
    private async resolveFile(
        path: string,
    ): Promise<FsResult<FileSystemFileHandle>> {
        const parent = fsGetBase(path);
        if (parent.err) {
            return parent;
        }

        const name = fsGetName(path);
        if (name.err) {
            return name;
        }

        const handle = await this.resolveDir(parent.val);
        if (handle.err) {
            return handle;
        }

        const file = await tryAsync(() => handle.val.getFileHandle(name.val));
        if ("err" in file) {
            const err = fsFail(
                "Failed to resolve file `" + path + "`: " + errstr(file.err),
            );
            return { err };
        }
        return file;
    }
}
