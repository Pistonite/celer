//! Logic for external editor workflow

import { FsErr, FsFileSystem, FsResult, fsJoin, fsRoot } from "pure/fs";

import { CompilerFileAccess } from "core/compiler";
import {
    IdleMgr,
    Yielder,
    createYielder,
    consoleEditor as console,
} from "low/utils";

import { EditorKernel } from "./EditorKernel";
import { EditorKernelAccess } from "./EditorKernelAccess";
import { ChangeTracker, StaticTimeTracker, newHashBasedTracker, newModifyTimeBasedTracker } from "./ChangeTracker";

console.info("loading external editor kernel");

export const initExternalEditor = (
    kernel: EditorKernelAccess,
    fs: FsFileSystem,
): EditorKernel => {
    console.info("creating external editor");
    return new ExternalEditorKernel(kernel, fs);
};

class ExternalEditorKernel implements EditorKernel, CompilerFileAccess {
    private deleted = false;
    private idleMgr: IdleMgr;
    private fs: FsFileSystem;

    private kernel: EditorKernelAccess;
    private fsYield: Yielder;
    /// Tracker used to track if file changed since last compile
    private staticTimeTracker: StaticTimeTracker; // used when fs is live
    private staticHashTracker: ChangeTracker; // used when fs is not live
    private tracker: ChangeTracker;

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
        const { live } = this.fs.capabilities;
        if (live) {
            console.info("using modify time based change tracker");
            this.tracker = newModifyTimeBasedTracker();
        } else {
            console.info("using hash based change tracker");
            this.tracker = newHashBasedTracker();
        }
        this.staticTimeTracker = new StaticTimeTracker();
        this.staticHashTracker = newHashBasedTracker();
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
        const changed = await this.checkDirectoryChanged(fsRoot(), this.fs.capabilities.live);

        if (changed) {
            this.staticTimeTracker.setLastTime(Date.now());
            this.notifyActivity();
            await this.kernel.reloadDocument();
        }
    }

    private async checkDirectoryChanged(path: string, live: boolean): Promise<boolean> {
        const entries = await this.fs.listDir(path);
        if (entries.err) {
            // error reading entry, something probably happened?
            return true;
        }

        // in non-live mode, we want to always iterate all files
        // to make sure we track all changes at once
        let changed = false;
        for (const entry of entries.val) {
            const subPath = fsJoin(path, entry);
            if (entry.endsWith("/")) {
                const dirChanged = await this.checkDirectoryChanged(subPath, live);
                if (dirChanged) {
                    changed = true;
                    if (live) {
                        return true;
                    }
                }
            } else {
                const fileChanged = await this.checkFileChanged(subPath, live);
                if (fileChanged) {
                    changed = true;
                    if (live) {
                        return true;
                    }
                }
            }
            await this.fsYield();
        }
        return changed;
    }

    private async checkFileChanged(path: string, live: boolean): Promise<boolean> {
        // close the file so we always get the latest modified time
        // note that in web editor flow, we don't need to do this
        // because the file system content always needs to be
        // manually synced to the web editor, which updates the modified time
        this.fs.getFile(path).close();
        const fsFile = this.fs.getFile(path);
        let result;
        if (live) {
            result = await this.staticTimeTracker.checkModifiedSinceLastAccess(fsFile);
        } else {
            result = await this.staticHashTracker.checkModifiedSinceLastAccess(fsFile);
        }
        if (result.err) {
            if (result.err.code === FsErr.NotModified) {
                return false;
            }
        }
        return true;
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
            const notModified =
                await this.tracker.checkModifiedSinceLastAccess(fsFile);
            if (notModified.err) {
                return notModified;
            }
        }

        const bytes = await fsFile.getBytes();
        const { live } = this.fs.capabilities;
        if (!live) {
            // close the file so we always get the latest content from disk
            // directly
            fsFile.close();
        }
        return bytes;
    }

    // === Stub implementations ===
    public async listDir(): Promise<string[]> {
        return [];
    }
    public async openFile(): Promise<void> {}
    public async hasUnsavedChanges(): Promise<boolean> {
        return false;
    }
    public hasUnsavedChangesSync(): boolean {
        return false;
    }
    public async loadFromFs(): Promise<void> {}
    public async saveToFs(): Promise<void> {}
}
