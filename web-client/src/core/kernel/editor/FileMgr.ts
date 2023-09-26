import * as monaco from "monaco-editor";

import { AppDispatcher, viewActions } from "core/store";
import { FileSys, FsFile, FsPath, FsResult, FsResultCodes } from "low/fs";
import { allocErr, allocOk, sleep } from "low/utils";

import { EditorContainerId, EditorLog, toFsPath } from "./utils";

type IStandaloneCodeEditor = monaco.editor.IStandaloneCodeEditor;

/// File manager
///
/// This manages the opened files in the editor
export class FileMgr {
    private fs: FileSys | undefined;

    /// Some operations need to block other operations,
    /// like saving and loading at the same time is probably bad
    ///
    /// Anything that changes files or currentFile or the monaco editor
    /// should lock the fs
    private fsLock = false;
    /// Opened files
    private files: Record<string, FsFile> = {};

    private currentFile: FsFile | undefined;
    private monacoDom: HTMLDivElement;
    private monacoEditor: IStandaloneCodeEditor;
    /// If the editor is open. Can be false even if currentFile is not undefined, if it's not a text file
    private isEditorOpen = false;

    private dispatcher: AppDispatcher;

    constructor(
        monacoDom: HTMLDivElement,
        monacoEditor: IStandaloneCodeEditor,
        dispatcher: AppDispatcher,
    ) {
        this.dispatcher = dispatcher;
        this.monacoDom = monacoDom;
        this.monacoEditor = monacoEditor;
    }

    public isFsLoaded(): boolean {
        return this.fs !== undefined;
    }

    public async reset(fs?: FileSys) {
        await this.lockedFsScope("reset", async () => {
            if (this.fs === fs) {
                return;
            }
            this.fs = fs;
            this.files = {};
            await this.updateEditor(undefined, undefined, undefined);
            this.dispatcher.dispatch(viewActions.setUnsavedFiles([]));
            if (fs) {
                EditorLog.info("resetting file system...");
                this.dispatcher.dispatch(
                    viewActions.updateFileSys({
                        rootPath: fs.getRootName(),
                        supportsSave: fs.isWritable(),
                    }),
                );
            } else {
                EditorLog.info("closing file system...");
                this.dispatcher.dispatch(
                    viewActions.updateFileSys({
                        rootPath: undefined,
                        supportsSave: true,
                    }),
                );
            }
        });
    }

    public delete() {
        this.lockedFsScope("delete", async () => {
            this.monacoEditor.dispose();
        });
    }

    public resizeEditor() {
        // do this async for any UI size changes to finish
        setTimeout(() => {
            // Resize to 0,0 to force monaco to shrink if needed
            this.monacoEditor.layout({ width: 0, height: 0 });
            this.monacoEditor.layout();
        }, 0);
    }

    public async listDir(path: string[]): Promise<string[]> {
        return await this.ensureLockedFs("listDir", async () => {
            if (!this.fs) {
                return [];
            }
            const fsPath = toFsPath(path);
            const result = await this.fs.listDir(fsPath);
            if (result.isErr()) {
                EditorLog.error(`listDir failed with code ${result.inner()}`);
                return [];
            }
            return result.inner();
        });
    }

    public async openFile(path: FsPath): Promise<FsResult<void>> {
        const idPath = path.path;
        EditorLog.info(`opening ${idPath}`);
        return await this.lockedFsScope("openFile", async () => {
            if (!this.fs) {
                EditorLog.error("openFile failed: fs is not initialized");
                return allocErr(FsResultCodes.Fail);
            }
            let fsFile = this.files[idPath];
            if (!fsFile) {
                fsFile = new FsFile(this.fs, path);
                this.files[idPath] = fsFile;
            }
            const result = await fsFile.getText();
            if (result.isErr()) {
                EditorLog.error(`openFile failed with code ${result.inner()}`);
                await this.updateEditor(fsFile, idPath, undefined);
            } else {
                await this.updateEditor(fsFile, idPath, result.inner());
            }
            return result.makeOk(undefined);
        });
    }

    public async loadChangesFromFs(): Promise<FsResult<void>> {
        EditorLog.info("loading changes from file system...");
        this.dispatcher.dispatch(viewActions.setAutoLoadActive(true));
        const handle = window.setTimeout(() => {
            this.dispatcher.dispatch(viewActions.startFileSysLoad());
        }, 200);
        const success = await this.lockedFsScope(
            "loadChangesFromFs",
            async () => {
                // ensure editor changes is synced first,
                // so the current file is marked dirty
                await this.syncEditorToCurrentFile();
                let success = true;
                for (const id in this.files) {
                    const fsFile = this.files[id];
                    const result = await this.loadChangesFromFsForFsFile(
                        id,
                        fsFile,
                    );
                    if (result.isErr()) {
                        success = false;
                    }
                    // yield to UI thread
                    await sleep(0);
                }
                return success;
            },
        );
        window.clearTimeout(handle);
        this.dispatcher.dispatch(viewActions.endFileSysLoad(success));
        EditorLog.info("changes loaded from file system");
        return success ? allocOk() : allocErr(FsResultCodes.Fail);
    }

    private async loadChangesFromFsForFsFile(
        idPath: string,
        fsFile: FsFile,
    ): Promise<FsResult<void>> {
        return await this.ensureLockedFs(
            "loadChangesFromFsForFsFile",
            async () => {
                const isCurrentFile = this.currentFile === fsFile;
                let content: string | undefined = undefined;

                let result = await fsFile.loadIfNotDirty();

                if (result.isOk()) {
                    if (isCurrentFile) {
                        const contentResult = await fsFile.getText();
                        if (contentResult.isOk()) {
                            content = contentResult.inner();
                        } else {
                            result = contentResult;
                        }
                    }
                }
                if (result.isErr()) {
                    EditorLog.error(`sync failed with code ${result}`);
                    if (!fsFile.isNewerThanFs()) {
                        EditorLog.info(`closing ${idPath}`);
                        if (isCurrentFile) {
                            await this.updateEditor(
                                undefined,
                                undefined,
                                undefined,
                            );
                        }
                        delete this.files[idPath];
                    }
                } else {
                    if (isCurrentFile) {
                        await this.updateEditor(fsFile, idPath, content);
                    }
                }
                return result;
            },
        );
    }

    public async hasUnsavedChanges(): Promise<boolean> {
        return await this.ensureLockedFs("hasUnsavedChanges", async () => {
            if (!this.isFsLoaded()) {
                return false;
            }
            await this.syncEditorToCurrentFile();
            for (const id in this.files) {
                const fsFile = this.files[id];
                if (fsFile.isNewerThanFs()) {
                    return true;
                }
            }
            return false;
        });
    }

    public hasUnsavedChangesSync(): boolean {
        if (!this.isFsLoaded()) {
            return false;
        }
        if (this.currentFile) {
            this.currentFile.setContent(this.monacoEditor.getValue());
        }
        for (const id in this.files) {
            const fsFile = this.files[id];
            if (fsFile.isNewerThanFs()) {
                return true;
            }
        }
        return false;
    }

    public async saveChangesToFs(): Promise<FsResult<void>> {
        if (!this.fs?.isWritable()) {
            return allocErr(FsResultCodes.NotSupported);
        }
        EditorLog.info("saving changes to file system...");
        const handle = window.setTimeout(() => {
            this.dispatcher.dispatch(viewActions.startFileSysSave());
        }, 200);
        const success = await this.lockedFsScope(
            "saveChangesToFs",
            async () => {
                // ensure editor changes is synced first,
                // so the current file is marked dirty
                await this.syncEditorToCurrentFile();
                let success = true;
                for (const id in this.files) {
                    const fsFile = this.files[id];
                    const result = await this.saveChangesToFsForFsFile(
                        id,
                        fsFile,
                    );
                    if (result.isErr()) {
                        success = false;
                    }
                    // yield to UI thread
                    await sleep(0);
                }
                return success;
            },
        );
        window.clearTimeout(handle);
        this.dispatcher.dispatch(viewActions.endFileSysSave(success));
        EditorLog.info("changes saved to file system");
        return success ? allocOk() : allocErr(FsResultCodes.Fail);
    }

    private async saveChangesToFsForFsFile(
        idPath: string,
        fsFile: FsFile,
    ): Promise<FsResult<void>> {
        return await this.ensureLockedFs(
            "saveChangesToFsForFsFile",
            async () => {
                const result = await fsFile.writeIfNewer();
                if (result.isErr()) {
                    EditorLog.error(`save ${idPath} failed with code ${result.inner()}`);
                }
                return result;
            },
        );
    }

    private async updateEditor(
        file: FsFile | undefined,
        path: string | undefined,
        content: string | undefined,
    ) {
        await this.ensureLockedFs("updateEditor", async () => {
            // in case we are switching files, sync the current file first
            if (this.currentFile !== file) {
                await this.syncEditorToCurrentFile();
                this.currentFile = file;
            }
            const success = content !== undefined;
            this.dispatcher.dispatch(
                viewActions.updateOpenedFile({
                    openedFile: path,
                    currentFileSupported: success,
                }),
            );

            if (success && path !== undefined) {
                // this check is necessary because
                // some browsers rerenders the editor even if the content is the same (Firefox)
                // which causes annoying flickering
                if (this.monacoEditor.getValue() !== content) {
                    this.monacoEditor.setValue(content);
                }

                // TODO: actually have language detection
                const model = this.monacoEditor.getModel();
                if (model) {
                    if (path.endsWith(".js")) {
                        if (model.getLanguageId() !== "javascript") {
                            monaco.editor.setModelLanguage(model, "javascript");
                        }
                    } else {
                        if (model.getLanguageId() !== "text") {
                            monaco.editor.setModelLanguage(model, "text");
                        }
                    }
                }

                await this.attachEditor();
                this.isEditorOpen = true;
            } else {
                this.monacoDom.remove();
                this.isEditorOpen = false;
            }
        });
    }

    public async syncEditorToCurrentFile() {
        await this.ensureLockedFs("syncEditorToCurrentFile", async () => {
            if (this.currentFile && this.isEditorOpen) {
                this.currentFile.setContent(this.monacoEditor.getValue());
            }
        });
    }

    public updateDirtyFileList(currentList: string[]) {
        const unsavedFiles: string[] = [];
        const ids = Object.keys(this.files);
        ids.sort();
        ids.forEach((id) => {
            if (this.files[id].isNewerThanFs()) {
                unsavedFiles.push(id);
            }
        });
        // don't update if the list is the same
        // to prevent unnecessary rerenders
        if (unsavedFiles.length === currentList.length) {
            let needsUpdate = false;
            for (let i = 0; i < unsavedFiles.length; i++) {
                if (unsavedFiles[i] !== currentList[i]) {
                    needsUpdate = true;
                    break;
                }
            }
            if (!needsUpdate) {
                return;
            }
        }
        this.dispatcher.dispatch(viewActions.setUnsavedFiles(unsavedFiles));
    }

    // public checkIfFileChangedSinceLastCompiled(path: string): boolean {
    //     const fsFile = this.files[path];
    //     if (!fsFile) {
    //         // if file is not loaded in the editor then
    //         // return true so it can be loaded
    //         return true;
    //     }
    //     return fsFile.wasChangedSinceLastCompile();
    // }

    public async getFileAsBytes(path: string): Promise<Uint8Array> {
        return await this.ensureLockedFs("getFileAsBytes", async () => {
            if (!this.fs) {
                throw new Error("fs is not initialized");
            }
            let fsFile = this.files[path];
            if (!fsFile) {
                const fsPath = toFsPath(path.split("/"));
                fsFile = new FsFile(this.fs, fsPath);
                this.files[fsPath.path] = fsFile;
            }
            const result = await fsFile.getBytes();
            if (result.isErr()) {
                throw new Error(`getFileAsBytes failed with code ${result.inner()}`);
            }
            return result.inner();
        });
    }

    // public async needsRecompile(): Promise<boolean> {
    //     return await this.ensureLockedFs("needsRecompile", async () => {
    //         if (!this.fs) {
    //             return false;
    //         }
    //         for (const id in this.files) {
    //             const fsFile = this.files[id];
    //             if (fsFile.wasChangedSinceLastCompile()) {
    //                 return true;
    //             }
    //             await sleep(0);
    //         }
    //         return false;
    //     });
    // }
    //
    // public wasFileChangedSinceLastCompile(path: string): boolean {
    //     if (!this.fs) {
    //         return false;
    //     }
    //     let fsFile = this.files[path];
    //     if (!fsFile) {
    //         const fsPath = toFsPath(path.split("/"));
    //         fsFile = new FsFile(this.fs, fsPath);
    //         this.files[fsPath.path] = fsFile;
    //     }
    //     return fsFile.wasChangedSinceLastCompile();
    // }

    private async attachEditor() {
        let div = document.getElementById(EditorContainerId);
        while (!div) {
            EditorLog.warn("editor container not found. Will try again.");
            await sleep(100);
            div = document.getElementById(EditorContainerId);
        }
        let alreadyAttached = false;
        div.childNodes.forEach((node) => {
            if (node === this.monacoDom) {
                alreadyAttached = true;
            } else {
                node.remove();
            }
        });
        if (!alreadyAttached) {
            div.appendChild(this.monacoDom);
            this.resizeEditor();
            EditorLog.info("editor attached");
        }
    }

    /// WARNING: f must not call lockedFsScope again - otherwise it will dead lock
    private async lockedFsScope<T>(
        reason: string,
        f: () => Promise<T>,
    ): Promise<T> {
        let cycles = 0;
        while (this.fsLock) {
            cycles++;
            if (cycles % 100 === 0) {
                EditorLog.warn(
                    `${reason} has been waiting for fs lock for ${
                        cycles / 10
                    } seconds!`,
                );
                cycles = 0;
            }
            await sleep(100);
        }
        try {
            this.fsLock = true;
            return await f();
        } finally {
            this.fsLock = false;
        }
    }

    /// Like withLockedFs but will not block if fs is already locked
    private async ensureLockedFs<T>(
        reason: string,
        f: () => Promise<T>,
    ): Promise<T> {
        if (this.fsLock) {
            return await f();
        }
        return await this.lockedFsScope(reason, f);
    }
}
