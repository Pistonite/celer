import * as monaco from "monaco-editor";

import { AppDispatcher, viewActions } from "core/store";
import { EditorContainerDOM } from "core/editor";
import {
    FileAccess,
    FileSys,
    FsFile,
    FsPath,
    FsResult,
    FsResultCodes,
} from "low/fs";
import {
    ReentrantLock,
    allocErr,
    allocOk,
    createYielder,
    sleep,
} from "low/utils";

import { EditorLog, detectLanguageByFileName, toFsPath } from "./utils";

type IStandaloneCodeEditor = monaco.editor.IStandaloneCodeEditor;

/// File manager
///
/// This manages the opened files in the editor
export class FileMgr implements FileAccess {
    private fs: FileSys;

    /// Some operations need to block other operations,
    /// like saving and loading at the same time is probably bad
    ///
    /// Anything that changes files or currentFile or the monaco editor
    /// should lock the fs
    private fsLock: ReentrantLock;
    /// Opened files
    private files: Record<string, FsFile> = {};

    private currentFile: FsFile | undefined;
    private monacoDom: HTMLDivElement;
    private monacoEditor: IStandaloneCodeEditor;
    /// If the editor is open. Can be false even if currentFile is not undefined, if it's not a text file
    private isEditorOpen = false;

    private dispatcher: AppDispatcher;

    constructor(
        fileSys: FileSys,
        monacoDom: HTMLDivElement,
        monacoEditor: IStandaloneCodeEditor,
        dispatcher: AppDispatcher,
    ) {
        this.fs = fileSys;
        this.dispatcher = dispatcher;
        this.monacoDom = monacoDom;
        this.monacoEditor = monacoEditor;
        this.fsLock = new ReentrantLock("file mgr");
    }

    public async setFileSys(fs: FileSys) {
        await this.fsLock.lockedScope(undefined, async (token) => {
            if (this.fs === fs) {
                return;
            }
            this.fs = fs;
            this.files = {};
            await this.updateEditor(undefined, undefined, undefined, token);
            this.dispatcher.dispatch(viewActions.setUnsavedFiles([]));
        });
    }

    public delete() {
        this.fsLock.lockedScope(undefined, async (token) => {
            this.updateEditor(undefined, undefined, undefined, token);
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

    public async listDir(
        path: string[],
        lockToken?: number,
    ): Promise<string[]> {
        return await this.fsLock.lockedScope(lockToken, async () => {
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

    public async openFile(
        path: FsPath,
        lockToken?: number,
    ): Promise<FsResult<void>> {
        const idPath = path.path;
        EditorLog.info(`opening ${idPath}`);
        return await this.fsLock.lockedScope(lockToken, async (token) => {
            let fsFile = this.files[idPath];
            if (!fsFile) {
                fsFile = new FsFile(this.fs, path);
                this.files[idPath] = fsFile;
            }
            const result = await fsFile.getText();
            if (result.isErr()) {
                EditorLog.error(`openFile failed with code ${result.inner()}`);
                await this.updateEditor(fsFile, idPath, undefined, token);
            } else {
                await this.updateEditor(fsFile, idPath, result.inner(), token);
            }
            return result.makeOk(undefined);
        });
    }

    public async loadChangesFromFs(
        lockToken?: number,
    ): Promise<FsResult<void>> {
        EditorLog.info("loading changes from file system...");
        const handle = window.setTimeout(() => {
            this.dispatcher.dispatch(viewActions.startFileSysLoad());
        }, 200);
        const success = await this.fsLock.lockedScope(
            lockToken,
            async (token) => {
                // ensure editor changes is synced first,
                // so the current file is marked dirty
                await this.syncEditorToCurrentFile(token);
                let success = true;
                const _yield = createYielder(64);
                for (const id in this.files) {
                    const fsFile = this.files[id];
                    const result = await this.loadChangesFromFsForFsFile(
                        id,
                        fsFile,
                        token,
                    );
                    if (result.isErr()) {
                        success = false;
                    }
                    await _yield();
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
        lockToken?: number,
    ): Promise<FsResult<void>> {
        return await this.fsLock.lockedScope(lockToken, async (token) => {
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
                            token,
                        );
                    }
                    delete this.files[idPath];
                }
            } else {
                if (isCurrentFile) {
                    await this.updateEditor(fsFile, idPath, content, token);
                }
            }
            return result;
        });
    }

    public async hasUnsavedChanges(lockToken?: number): Promise<boolean> {
        return await this.fsLock.lockedScope(lockToken, async (token) => {
            await this.syncEditorToCurrentFile(token);
            const yielder = createYielder(64);
            for (const id in this.files) {
                const fsFile = this.files[id];
                if (fsFile.isNewerThanFs()) {
                    return true;
                }
                await yielder();
            }
            return false;
        });
    }

    public hasUnsavedChangesSync(): boolean {
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

    public async saveChangesToFs(lockToken?: number): Promise<FsResult<void>> {
        if (!this.fs.isWritable()) {
            return allocErr(FsResultCodes.NotSupported);
        }
        EditorLog.info("saving changes to file system...");
        const handle = window.setTimeout(() => {
            this.dispatcher.dispatch(viewActions.startFileSysSave());
        }, 200);
        const success = await this.fsLock.lockedScope(
            lockToken,
            async (token) => {
                // ensure editor changes is synced first,
                // so the current file is marked dirty
                await this.syncEditorToCurrentFile(token);
                let success = true;
                const _yield = createYielder(64);
                for (const id in this.files) {
                    const fsFile = this.files[id];
                    const result = await this.saveChangesToFsForFsFile(
                        id,
                        fsFile,
                        token,
                    );
                    if (result.isErr()) {
                        success = false;
                    }
                    await _yield();
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
        lockToken?: number,
    ): Promise<FsResult<void>> {
        return await this.fsLock.lockedScope(lockToken, async () => {
            const result = await fsFile.writeIfNewer();
            if (result.isErr()) {
                EditorLog.error(
                    `save ${idPath} failed with code ${result.inner()}`,
                );
            }
            return result;
        });
    }

    private async updateEditor(
        file: FsFile | undefined,
        path: string | undefined,
        content: string | undefined,
        lockToken?: number,
    ) {
        await this.fsLock.lockedScope(lockToken, async (token) => {
            // in case we are switching files, sync the current file first
            if (this.currentFile !== file) {
                await this.syncEditorToCurrentFile(token);
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

                // TODO #20: language feature support
                this.switchLanguage(detectLanguageByFileName(path));

                await this.attachEditor();
                this.isEditorOpen = true;
            } else {
                this.monacoDom.remove();
                this.isEditorOpen = false;
            }
        });
    }

    private switchLanguage(languageId: string) {
        const model = this.monacoEditor.getModel();
        if (!model) {
            return;
        }
        if (model.getLanguageId() !== languageId) {
            monaco.editor.setModelLanguage(model, languageId);
        }
    }

    public async syncEditorToCurrentFile(lockToken?: number) {
        await this.fsLock.lockedScope(lockToken, async () => {
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

    private modifiedTimeWhenLastAccessed: { [path: string]: number } = {};
    public async getFileContent(
        path: string,
        checkChanged: boolean,
    ): Promise<FsResult<Uint8Array>> {
        return await this.fsLock.lockedScope(undefined, async () => {
            if (!this.fs) {
                return allocErr(FsResultCodes.Fail);
            }
            let fsFile = this.files[path];
            if (!fsFile) {
                const fsPath = toFsPath(path.split("/"));
                fsFile = new FsFile(this.fs, fsPath);
                this.files[fsPath.path] = fsFile;
            }
            if (checkChanged) {
                const modifiedTimeLast =
                    this.modifiedTimeWhenLastAccessed[path];
                const modifiedTimeCurrent = await fsFile.getLastModified();
                this.modifiedTimeWhenLastAccessed[path] = modifiedTimeCurrent;
                if (
                    modifiedTimeLast &&
                    modifiedTimeLast >= modifiedTimeCurrent
                ) {
                    // 1. file was accessed before
                    // 2. file was not modified since last access
                    return allocErr(FsResultCodes.NotModified);
                }
            }
            return await fsFile.getBytes();
        });
    }

    private async attachEditor() {
        let div = EditorContainerDOM.get();
        while (!div) {
            EditorLog.warn("editor container not found. Will try again.");
            await sleep(100);
            div = EditorContainerDOM.get();
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
}
