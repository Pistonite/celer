import { Ok, tryAsync } from "pure/result";
import { errstr } from "pure/utils";

import { FsErr, FsResult, FsVoid, fsErr, fsFail } from "./FsError.ts";
import {
    FsFileSystem,
    FsFileSystemUninit,
    FsCapabilities,
} from "./FsFileSystem.ts";
import { FsFile } from "./FsFile.ts";
import { fsIsRoot, fsNormalize } from "./FsPath.ts";
import { FsFileMgr } from "./FsFileMgr.ts";
import { FsFileSystemInternal } from "./FsFileSystemInternal.ts";

/// FsFileSystem implementation that uses FileEntry API
export class FsImplEntryAPI
    implements FsFileSystemUninit, FsFileSystem, FsFileSystemInternal
{
    public root: string;
    public capabilities: FsCapabilities;

    private rootEntry: FileSystemDirectoryEntry;

    private mgr: FsFileMgr;

    constructor(root: string, rootEntry: FileSystemDirectoryEntry) {
        this.root = root;
        this.rootEntry = rootEntry;
        this.capabilities = {
            write: false,
            live: true,
        };
        this.mgr = new FsFileMgr();
    }

    public init(): Promise<FsResult<FsFileSystem>> {
        // no init needed
        return Promise.resolve({ val: this });
    }

    public async listDir(path: string): Promise<FsResult<string[]>> {
        const normalized = fsNormalize(path);
        if (normalized.err) {
            return normalized;
        }
        path = normalized.val;

        const entry = await this.resolveDir(path);
        if (entry.err) {
            return entry;
        }

        const entries = await tryAsync(
            () =>
                new Promise<FileSystemEntry[]>((resolve, reject) => {
                    entry.val.createReader().readEntries(resolve, reject);
                }),
        );
        if ("err" in entries) {
            const err = fsFail(
                "Failed to list directory `" +
                    path +
                    "`: " +
                    errstr(entries.err),
            );
            return { err };
        }

        const names = entries.val.map(({ isDirectory, name }) => {
            if (isDirectory) {
                return name + "/";
            }
            return name;
        });

        return { val: names };
    }

    public async read(path: string): Promise<FsResult<File>> {
        const normalized = fsNormalize(path);
        if (normalized.err) {
            return normalized;
        }
        path = normalized.val;

        const entry = await this.resolveFile(path);
        if (entry.err) {
            return entry;
        }

        const file = await tryAsync(
            () =>
                new Promise<File>((resolve, reject) => {
                    entry.val.file(resolve, reject);
                }),
        );
        if ("err" in file) {
            const err = fsFail(
                "Failed to read file `" + path + "`: " + errstr(file.err),
            );
            return { err };
        }

        return file;
    }

    public write(): Promise<FsVoid> {
        const err = fsErr(
            FsErr.NotSupported,
            "Write not supported in FileEntry API",
        );
        return Promise.resolve({ err });
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

    /// Resolve a directory entry. Path must be normalized
    private async resolveDir(
        path: string,
    ): Promise<FsResult<FileSystemDirectoryEntry>> {
        if (fsIsRoot(path)) {
            return { val: this.rootEntry };
        }
        const entry = await tryAsync(
            () =>
                new Promise<FileSystemEntry>((resolve, reject) => {
                    this.rootEntry.getDirectory(path, {}, resolve, reject);
                }),
        );
        if ("err" in entry) {
            const err = fsFail(
                "Failed to resolve directory `" +
                    path +
                    "`: " +
                    errstr(entry.err),
            );
            return { err };
        }
        if (!entry.val.isDirectory) {
            const err = fsErr(
                FsErr.IsFile,
                "Path `" + path + "` is not a directory",
            );
            return { err };
        }
        return entry as Ok<FileSystemDirectoryEntry>;
    }

    /// Resolve a file entry. Path must be normalized
    private async resolveFile(
        path: string,
    ): Promise<FsResult<FileSystemFileEntry>> {
        if (fsIsRoot(path)) {
            const err = fsErr(
                FsErr.IsDirectory,
                "Path `" + path + "` is not a file",
            );
            return { err };
        }
        const entry = await tryAsync(
            () =>
                new Promise<FileSystemEntry>((resolve, reject) => {
                    this.rootEntry.getFile(path, {}, resolve, reject);
                }),
        );
        if ("err" in entry) {
            const err = fsFail(
                "Failed to resolve file `" + path + "`: " + errstr(entry.err),
            );
            return { err };
        }
        if (!entry.val.isFile) {
            const err = fsErr(
                FsErr.IsDirectory,
                "Path `" + path + "` is not a file",
            );
            return { err };
        }
        return entry as Ok<FileSystemFileEntry>;
    }
}
