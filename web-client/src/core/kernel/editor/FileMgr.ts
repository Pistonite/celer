import * as monaco from "monaco-editor";

import { AppDispatcher, viewActions } from "core/store";
import { FileSys, FsFile, FsPath, FsResultCode, FsResultCodes } from "low/fs";
import { isInDarkMode, sleep } from "low/utils";

import { EditorContainerId, EditorLog, toFsPath } from "./utils";
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

    /// Opened editor
    private currentFile: FsFile | undefined;
    private monacoDom: HTMLDivElement;
    private monacoEditor: monaco.editor.IStandaloneCodeEditor;

    private dispatcher: AppDispatcher;

    constructor(dispatcher: AppDispatcher) {
        this.dispatcher = dispatcher;
        this.monacoDom = document.createElement("div");
        this.monacoDom.id = "monaco-editor";
        this.monacoEditor = monaco.editor.create(this.monacoDom, {
            theme: isInDarkMode() ? "vs-dark" : "vs",
        });
    }

    public isFsLoaded(): boolean {
        return this.fs !== undefined;
    }

    public async reset(fs?: FileSys) {
        await this.lockedFsScope("reset", async () => {
            this.fs = fs;
            this.files = {};
            await this.updateEditor(undefined, undefined, undefined);
            if (fs) {
                this.dispatcher.dispatch(
                    viewActions.updateFileSys({
                        rootPath: fs.getRootName(),
                        supportsSave: fs.isWritable(),
                    }),
                );
            } else {
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
            if (result.code !== FsResultCodes.Ok) {
                EditorLog.error(`listDir failed with code ${result.code}`);
                return [];
            }
            return result.value;
        });
    }

    public async openFile(path: FsPath): Promise<FsResultCode> {
        const idPath = path.path;
        EditorLog.info(`opening ${idPath}`);
        return await this.lockedFsScope("openFile", async () => {
            if (!this.fs) {
                EditorLog.error("openFile failed: fs is not initialized");
                return FsResultCodes.Fail;
            }
            let fsFile = this.files[idPath];
            if (!fsFile) {
                fsFile = new FsFile(this.fs, path);
                this.files[idPath] = fsFile;
            }
            const content = await fsFile.getContent();
            if (content.code !== FsResultCodes.Ok) {
                EditorLog.error(`openFile failed with code ${content.code}`);
                await this.updateEditor(fsFile, idPath, undefined);
            } else {
                await this.updateEditor(fsFile, idPath, content.value);
            }
            return content.code;
        });
    }

    public async loadChangesFromFs(): Promise<FsResultCode> {
        EditorLog.info("loading changes from filesystem");
        this.dispatcher.dispatch(viewActions.setAutoLoadActive(true));
        const handle = window.setTimeout(() => {
            this.dispatcher.dispatch(viewActions.startFileSysLoad());
        }, 200);
        const success = await this.lockedFsScope(
            "loadChangesFromFs",
            async () => {
                let success = true;
                for (const id in this.files) {
                    const fsFile = this.files[id];
                    const result = await this.loadChangesFromFsForFsFile(
                        id,
                        fsFile,
                    );
                    if (result !== FsResultCodes.Ok) {
                        success = false;
                    }
                    // sleep some time so the UI doesn't freeze
                    await sleep(50);
                }
                return success;
            },
        );
        window.clearTimeout(handle);
        this.dispatcher.dispatch(viewActions.endFileSysLoad(success));
        EditorLog.info("changes loaded from filesystem");
        return success ? FsResultCodes.Ok : FsResultCodes.Fail;
    }

    async loadChangesFromFsForFsFile(
        idPath: string,
        fsFile: FsFile,
    ): Promise<FsResultCode> {
        EditorLog.info(`syncing ${idPath}`);
        return await this.ensureLockedFs(
            "loadChangesFromFsForFsFile",
            async () => {
                const isCurrentFile = this.currentFile === fsFile;
                let content: string | undefined = undefined;

                let result = await fsFile.load();

                if (result === FsResultCodes.Ok) {
                    if (isCurrentFile) {
                        const contentResult = await fsFile.getContent();
                        result = contentResult.code;
                        if (contentResult.code === FsResultCodes.Ok) {
                            content = contentResult.value;
                        }
                    }
                }
                if (result !== FsResultCodes.Ok) {
                    EditorLog.error(`sync failed with code ${result}`);
                    if (!fsFile.isDirty()) {
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

    private async updateEditor(
        file: FsFile | undefined,
        path: string | undefined,
        content: string | undefined,
    ) {
        this.ensureLockedFs("updateEditor", async () => {
            this.currentFile = file;
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
                // TODO: actually have good language detection
                if (path.endsWith(".js")) {
                    const model = this.monacoEditor.getModel();
                    if (model) {
                        if (model.getLanguageId() !== "javascript") {
                            monaco.editor.setModelLanguage(model, "javascript");
                        }
                    }
                }
                await this.attachEditor();
            } else {
                this.monacoDom.remove();
            }
        });
    }

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
        while (this.fsLock) {
            EditorLog.info(`${reason} waiting for fs to become available...`);
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
