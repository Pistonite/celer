//! Logic for external editor workflow

import { FsFileSystem, FsResult, fsJoin, fsRoot } from "pure/fs";

import { CompilerFileAccess } from "core/compiler";
import { IdleMgr, Yielder, createYielder, consoleEditor as console } from "low/utils";

import { EditorKernel } from "./EditorKernel";
import { EditorKernelAccess } from "./EditorKernelAccess";
import { ModifyTimeTracker } from "./ModifyTimeTracker";

console.info("loading external editor kernel");

export const initExternalEditor = (kernel: EditorKernelAccess, fs: FsFileSystem): EditorKernel => {
    console.info("creating external editor");
    return new ExternalEditorKernel(kernel, fs);
};

class ExternalEditorKernel implements EditorKernel, CompilerFileAccess{
    private deleted = false;
    private idleMgr: IdleMgr;
    private fs: FsFileSystem;
    private lastCompiledTime = 0;

    private kernel: EditorKernelAccess;
    private fsYield: Yielder;
    private modifyTracker: ModifyTimeTracker;

    constructor(kernel: EditorKernelAccess, fs: FsFileSystem) {
        this.kernel = kernel;
        this.fs = fs;
        this.idleMgr = new IdleMgr(
            0,
            1000,
            2,
            20,
            8000,
            this.recompileIfChanged.bind(this),
        );
        this.fsYield = createYielder(64);
        this.modifyTracker = new ModifyTimeTracker();
        this.idleMgr.start();
    }

    public delete(): void {
        console.info("deleting external editor");
        if (this.deleted) {
            console.warn("editor already deleted");
            return;
        }
        this.deleted = true;
        this.idleMgr.stop();
    }

    public notifyActivity(): void {
        this.idleMgr.notifyActivity();
    }

    private async recompileIfChanged() {
        // locking is not needed because idle will be paused
        // when an idle cycle is running
        const changed = await this.checkDirectoryChanged(fsRoot());
        if (changed) {
            this.lastCompiledTime = Date.now();
            this.notifyActivity();
            this.kernel.reloadDocument();
        }
    }

    private async checkDirectoryChanged(path: string): Promise<boolean> {
        const entries = await this.fs.listDir(path);
        if (entries.err) {
            // error reading entry, something probably happened?
            return true;
        }
        for (const entry of entries.val) {
            const subPath = fsJoin(path, entry);
            if (entry.endsWith("/")) {
                const subDirChanged = await this.checkDirectoryChanged(subPath);
                if (subDirChanged) {
                    return true;
                }
            } else {
                const fileChanged = await this.checkFileChanged(subPath);
                if (fileChanged) {
                    return true;
                }
            }
            await this.fsYield();
        }
        return false;
    }

    private async checkFileChanged(path: string): Promise<boolean> {
        const fsFile = this.fs.getFile(path);
        const lastModified = await fsFile.getLastModified();
        if (lastModified.err) {
            return true;
        }
        return lastModified.val > this.lastCompiledTime;
    }

    // === CompilerFileAccess ===
    public getFileAccess(): CompilerFileAccess {
        return this;
    }

    public async getFileContent(
        path: string,
        checkChanged: boolean,
    ): Promise<FsResult<Uint8Array>> {
        const fsFile = this.fs.getFile(path);
        if (checkChanged) {
            const notModified = await this.modifyTracker.checkModifiedSinceLastAccess(fsFile);
            if (notModified.err) {
                return notModified;
            }
        }

        return await fsFile.getBytes();
    }

    // === Stub implementations ===
    public async listDir(): Promise<string[]> { return []; }
    public async openFile(): Promise<void> { }
    public async hasUnsavedChanges(): Promise<boolean> {
        return false;
    }
    public hasUnsavedChangesSync(): boolean {
        return false;
    }
    public async loadFromFs(): Promise<void> { }
    public async saveToFs(): Promise<void> { }
}
