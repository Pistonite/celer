//! Logic for external editor workflow

import {
    FileAccess,
    FileSys,
    FsPath,
    FsResult,
    FsResultCodes,
    fsRootPath,
} from "low/fs";
import { IdleMgr, Yielder, createYielder, allocOk } from "low/utils";

import { EditorKernel } from "./EditorKernel";
import { EditorLog, KernelAccess, toFsPath } from "./utils";

EditorLog.info("loading external editor kernel");

export const initExternalEditor = (
    kernelAccess: KernelAccess,
    fileSys: FileSys,
): EditorKernel => {
    EditorLog.info("creating external editor");
    return new ExternalEditorKernel(kernelAccess, fileSys);
};

class ExternalEditorKernel implements EditorKernel, FileAccess {
    private idleMgr: IdleMgr;
    private fs: FileSys;
    private lastCompiledTime = 0;

    private kernelAccess: KernelAccess;

    constructor(kernelAccess: KernelAccess, fileSys: FileSys) {
        this.kernelAccess = kernelAccess;
        this.fs = fileSys;
        this.idleMgr = new IdleMgr(
            0,
            1000,
            2,
            20,
            8000,
            this.recompileIfChanged.bind(this),
        );
        this.idleMgr.start();
    }

    public delete(): void {
        EditorLog.info("deleting external editor");
        // @ts-expect-error setting to undefined to make sure the editor is not double-deleted
        window.__theEditorKernel = undefined;
        this.idleMgr.stop();
    }

    public notifyActivity(): void {
        this.idleMgr.notifyActivity();
    }

    private async recompileIfChanged() {
        const yielder = createYielder(64);
        const changed = await this.checkDirectoryChanged(yielder, fsRootPath);
        if (changed) {
            this.lastCompiledTime = Date.now();
            this.notifyActivity();
            this.kernelAccess.compile();
        }
    }

    private async checkDirectoryChanged(
        yielder: Yielder,
        fsPath: FsPath,
    ): Promise<boolean> {
        const fsDirResult = await this.fs.listDir(fsPath);
        if (fsDirResult.isErr()) {
            return false;
        }
        const dirContent = fsDirResult.inner();
        for (const entry of dirContent) {
            const subPath = fsPath.resolve(entry);
            if (entry.endsWith("/")) {
                const subDirChanged = await this.checkDirectoryChanged(
                    yielder,
                    subPath,
                );
                if (subDirChanged) {
                    return true;
                }
            } else {
                const fileChanged = await this.checkFileChanged(subPath);
                if (fileChanged) {
                    return true;
                }
            }
            await yielder();
        }
        return false;
    }

    private async checkFileChanged(fsPath: FsPath): Promise<boolean> {
        const fsFileResult = await this.fs.readFile(fsPath);
        if (fsFileResult.isErr()) {
            return false;
        }
        const modifiedTime = fsFileResult.inner().lastModified;
        return modifiedTime > this.lastCompiledTime;
    }

    // === FileAccess ===
    public getFileAccess(): FileAccess {
        return this;
    }
    private cachedFileContent: { [path: string]: Uint8Array } = {};
    private modifiedTimeWhenLastAccessed: { [path: string]: number } = {};
    public async getFileContent(
        path: string,
        checkChanged: boolean,
    ): Promise<FsResult<Uint8Array>> {
        const fsPath = toFsPath(path.split("/"));
        const fsFileResult = await this.fs.readFile(fsPath);
        if (fsFileResult.isErr()) {
            return fsFileResult;
        }
        const file = fsFileResult.inner();
        const modifiedTimeLast = this.modifiedTimeWhenLastAccessed[path];
        const modifiedTimeCurrent = file.lastModified;
        this.modifiedTimeWhenLastAccessed[path] = modifiedTimeCurrent;
        if (
            path in this.cachedFileContent &&
            modifiedTimeLast &&
            modifiedTimeLast >= modifiedTimeCurrent
        ) {
            // 1. file was accessed before (and cached)
            // 2. file was not modified since last access
            if (checkChanged) {
                return fsFileResult.makeErr(FsResultCodes.NotModified);
            }
            return fsFileResult.makeOk(this.cachedFileContent[path]);
        }
        // file was not accessed before or was modified since last access
        const bytes = new Uint8Array(await file.arrayBuffer());
        this.cachedFileContent[path] = bytes;
        return fsFileResult.makeOk(bytes);
    }

    // === Stub implementations ===
    async listDir(): Promise<string[]> {
        return [];
    }
    async openFile(): Promise<FsResult<void>> {
        return allocOk(undefined);
    }
    async hasUnsavedChanges(): Promise<boolean> {
        return false;
    }
    hasUnsavedChangesSync(): boolean {
        return false;
    }
    async loadChangesFromFs(): Promise<FsResult<void>> {
        return allocOk(undefined);
    }
    async saveChangesToFs(): Promise<FsResult<void>> {
        return allocOk(undefined);
    }
}
