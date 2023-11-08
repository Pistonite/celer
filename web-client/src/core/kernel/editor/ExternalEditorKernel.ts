import { FileAccess, FileSys, FsResult, FsResultCodes } from "low/fs";
import { EditorKernel } from "./EditorKernel";
import { EditorLog, KernelAccess, toFsPath } from "./utils";
import { AppStore } from "core/store";
import { IdleMgr, ReentrantLock, allocErr, allocOk } from "low/utils";

EditorLog.info("loading external editor kernel");

export const initExternalEditor = (): EditorKernel => {
}

class ExternalEditorKernel implements EditorKernel, FileAccess {
    private store: AppStore;

    private idleMgr: IdleMgr;
    private fs: FileSys;
    private fsLock: ReentrantLock;

    private shouldRecompile = false;
    private kernelAccess: KernelAccess;

    constructor(store: AppStore, kernelAccess: KernelAccess, fileSys: FileSys) {
        this.store = store;
        this.kernelAccess = kernelAccess;
        this.fs = fileSys;
        this.idleMgr = new IdleMgr(0, 1000, 2, 20, 8000, this.recompileIfChanged.bind(this));
        this.fsLock = new ReentrantLock("fs");
    }

    public delete(): void {
        EditorLog.info("deleting external editor");
        // @ts-expect-error setting to undefined to make sure the editor is not double-deleted
        window.__theEditorKernel = undefined;
        this.idleMgr.stop();
    }

    public nofityActivity(): void {
        this.idleMgr.notifyActivity();
    }

    private async recompileIfChanged() {
    }
    
    // === FileAccess ===
    public getFileAccess(): FileAccess {
        return this;
    }
    private cachedFileContent: { [path: string]: Uint8Array } = {};
    private modifiedTimeWhenLastAccessed: { [path: string]: number } = {};
    public async getFileContent(path: string, checkChanged: boolean): Promise<FsResult<Uint8Array>> {
        const fsPath = toFsPath(path.split("/"));
        const fsFileResult = await this.fs.readFile(fsPath);
        if (fsFileResult.isErr()) {
            return fsFileResult;
        }
        const file = fsFileResult.inner();
        const modifiedTimeLast =
            this.modifiedTimeWhenLastAccessed[path];
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

    public async exists(path: string): Promise<boolean> {
        const result = await this.getFileContent(path, true);
        if (result.isOk()) {
            return true;
        }
        if (result.inner() === FsResultCodes.NotModified) {
            return true;
        }
        return false;
    }

    // === Stub implementations ===
    async listDir(): Promise<string[]> {
        return []
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
