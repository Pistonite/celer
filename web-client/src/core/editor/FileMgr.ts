import * as monaco from "monaco-editor";

import { FsErr, FsError, FsFile, FsFileSystem, FsResult, fsErr } from "pure/fs";
import { RwLock } from "pure/utils";

import { AppDispatcher, viewActions } from "core/store";
import { CompilerFileAccess } from "core/compiler";
import {
    Yielder,
    createYielder,
    sleep,
} from "low/utils";

import { EditorContainerDOM } from "./dom";
import { EditorLog, detectLanguageByFileName } from "./utils";

type IStandaloneCodeEditor = monaco.editor.IStandaloneCodeEditor;

/// File manager
///
/// This manages the opened files in the editor
export class FileMgr implements CompilerFileAccess {
    /// Using a RwLock to ensure the tracked files don't change
    /// while being iterated. Generall, write lock is needed
    /// for close() and getFile() for unopened files
    private fs: RwLock<FsFileSystem>;

    private supportsSave: boolean;

    // /// Some operations need to block other operations,
    // /// like saving and loading at the same time is probably bad
    // ///
    // /// Anything that changes files or currentFile or the monaco editor
    // /// should lock the fs
    // private fsLock: ReentrantLock;
    // /// Opened files
    // private files: Record<string, FsFile> = {};

    private currentFile: FsFile | undefined;
    private monacoDom: HTMLDivElement;
    private monacoEditor: IStandaloneCodeEditor;
    /// If the editor is open. Can be false even if currentFile is not undefined, if it's not a text file
    private isEditorOpen = false;

    /// Yielder for file system operations
    private fsYield: Yielder;
    private dispatcher: AppDispatcher;

    constructor(
        fs: FsFileSystem,
        monacoDom: HTMLDivElement,
        monacoEditor: IStandaloneCodeEditor,
        dispatcher: AppDispatcher,
    ) {
        this.supportsSave = fs.capabilities.write;
        this.fs = new RwLock(fs);
        this.dispatcher = dispatcher;
        this.monacoDom = monacoDom;
        this.monacoEditor = monacoEditor;
        this.fsYield = createYielder(64);
        // this.fsLock = new ReentrantLock("file mgr");
    }

    // public setFileSys(fs: FsFileSystem): Promise<void> {
    //     return this.fs.scopedWrite(async (thisFs, setFs) => {
    //         if (thisFs === fs) {
    //             return;
    //         }
    //         thisFs = setFs(fs);
    //         this.updateEditor(undefined, undefined, undefined);
    //         this.dispatcher.dispatch(viewActions.setUnsavedFiles([]));
    //     });
    //     await this.fsLock.lockedScope(undefined, async (token) => {
    //     });
    // }

    public delete() {
        // this.fsLock.lockedScope(undefined, async (token) => {
            this.closeEditor();
            this.monacoEditor.dispose();
        // });
    }

    public async resizeEditor() {
        // do this async for any UI size changes to finish
        await sleep(0);
        // Resize to 0,0 to force monaco to shrink if needed
        this.monacoEditor.layout({ width: 0, height: 0 });
        this.monacoEditor.layout();
    }

    public listDir(path: string): Promise<string[]> {
        return this.fs.scopedRead((fs) => {
            return this.listDirWithFs(fs, path);
        });
    }

    private async listDirWithFs(fs: FsFileSystem, path: string): Promise<string[]> {
        const { val: entries, err } = await fs.listDir(path);
        if (err) {
            const { code, message } = err;
            EditorLog.error(`listDir failed with code ${code}: ${message}`);
            return [];
        }
        return entries;
    }

    public openFile(path: string,): Promise<void> {
        EditorLog.info(`opening ${path}`);
        return this.fs.scopedWrite(async (fs) => {
            return this.openFileWithFs(fs, path);
        });
    }

    private async openFileWithFs(fs: FsFileSystem, path: string): Promise<void> {
        const fsFile = fs.getFile(path);
        const { val, err } = await fsFile.getText();
        if (err) {
            const { code, message } = err;
            EditorLog.error(`openFile failed with code ${code}: ${message}`);
            this.updateEditor(fsFile, undefined);
            return;
        }
        this.updateEditor(fsFile, val);
    }

    public async loadFromFs(): Promise<void> {
        EditorLog.info("syncing files from file system to editor...");
        const handle = window.setTimeout(() => {
            this.dispatcher.dispatch(viewActions.startFileSysLoad());
        }, 200);
        // ensure editor changes is synced first,
        // so the current file is marked dirty if needed
        this.syncEditorToCurrentFile();
        const success = await this.fs.scopedWrite((fs) => {
            return this.loadFromFsWithFs(fs);
        });
        // const success = await this.fsLock.lockedScope(
        //     lockToken,
        //     async (token) => {
        //         let success = true;
        //         const _yield = createYielder(64);
        //         for (const id in this.files) {
        //             const fsFile = this.files[id];
        //             const result = await this.loadChangesFromFsForFsFile(
        //                 id,
        //                 fsFile,
        //                 token,
        //             );
        //             if (result.isErr()) {
        //                 success = false;
        //             }
        //             await _yield();
        //         }
        //         return success;
        //     },
        // );
        window.clearTimeout(handle);
        this.dispatcher.dispatch(viewActions.endFileSysLoad(success));
        EditorLog.info("sync completed");
        // return success ? allocOk() : allocErr(FsResultCodes.Fail);
    }

    private async loadFromFsWithFs(fs: FsFileSystem): Promise<boolean> {
        const paths = fs.getOpenedPaths();
        let success = true;
        for (let i = 0; i < paths.length; i++) {
            if (!(await this.loadFromFsForPath(fs, paths[i]))) {
                success = false;
            }

            await this.fsYield();
        }
        return success;
                    // const fsFile = this.files[id];
                    // const result = await this.loadChangesFromFsForFsFile(
                    //     id,
                    //     fsFile,
                    //     token,
                    // );
                    // if (result.isErr()) {
                    //     success = false;
                    // }
                    // await _yield();
    }

    private async loadFromFsForPath(fs: FsFileSystem, path: string): Promise<boolean> {
        const fsFile = fs.getFile(path);
        // return await this.fsLock.lockedScope(lockToken, async (token) => {
        const isCurrentFile = this.currentFile === fsFile;

        // let content: string | undefined = undefined;
        let loadError: FsError | undefined = undefined;

        // load the file
        const loadResult = await fsFile.loadIfNotDirty();
        if (loadResult.err) {
            loadError = loadResult.err;
        } else {
            // load is fine, may need to update the content
            if (isCurrentFile) {
                const content = await fsFile.getText();
                if (content.err) {
                    loadError = content.err;
                } else if (!fsFile.isDirty()) {
                    // if the file is not dirty, update the editor's content
                    this.updateEditor(fsFile, content.val);
                }
            }
        }

        if (loadError) {
            const { code, message } = loadError;
            EditorLog.error(`sync failed with code ${code}: ${message}`);
            if (!fsFile.isDirty()) {
                // if the file is not dirty, we close the file
                // in case it doesn't exist on disk anymore
                // and to avoid error on the next save
                EditorLog.info(`closing ${path} due to sync error`);
                if (isCurrentFile) {
                    this.closeEditor();
                }
                fsFile.close();
            }
        }

        return loadError === undefined;
            //
            // let result = await fsFile.loadIfNotDirty();
            //
            // if (result.isOk()) {
            //     if (isCurrentFile) {
            //         const contentResult = await fsFile.getText();
            //         if (contentResult.isOk()) {
            //             content = contentResult.inner();
            //         } else {
            //             result = contentResult;
            //         }
            //     }
            // }
            // if (result.isErr()) {
            //     EditorLog.error(`sync failed with code ${result}`);
            //     if (!fsFile.isNewerThanFs()) {
            //         EditorLog.info(`closing ${idPath}`);
            //         if (isCurrentFile) {
            //             await this.updateEditor(
            //                 undefined,
            //                 undefined,
            //                 undefined,
            //                 token,
            //             );
            //         }
            //         delete this.files[idPath];
            //     }
            // } else {
            //     if (isCurrentFile) {
            //         await this.updateEditor(fsFile, idPath, content, token);
            //     }
            // }
            // return result;
        // });
    }

    public hasUnsavedChanges(): Promise<boolean> {
        this.syncEditorToCurrentFile();
        return this.fs.scopedRead(async (fs) => {
            const paths = fs.getOpenedPaths();
            for (let i = 0; i < paths.length; i++) {
                const fsFile = fs.getFile(paths[i]);
                if (fsFile.isDirty()) {
                    return true;
                }
                await this.fsYield();
            }
            return false;
        });
        // return await this.fsLock.lockedScope(lockToken, async (token) => {
        //     await this.syncEditorToCurrentFile(token);
        //     const yielder = createYielder(64);
        //     for (const id in this.files) {
        //         const fsFile = this.files[id];
        //         if (fsFile.isNewerThanFs()) {
        //             return true;
        //         }
        //         await yielder();
        //     }
        //     return false;
        // });
    }

    public hasUnsavedChangesSync(): boolean {
        this.syncEditorToCurrentFile();
        const fs = this.fs.inner;
        const paths = fs.getOpenedPaths();
        for (let i = 0; i < paths.length; i++) {
            const fsFile = fs.getFile(paths[i]);
            if (fsFile.isDirty()) {
                return true;
            }
        }
        return false;
    }

    public async saveToFs(): Promise<boolean> {
        if (!this.supportsSave) {
            EditorLog.error("save not supported!");
            EditorLog.warn("saveToFs should only be called if save is supported");
            return false;
        }

        EditorLog.info("saving changes...");
        const handle = window.setTimeout(() => {
            this.dispatcher.dispatch(viewActions.startFileSysSave());
        }, 200);
        // ensure editor changes is synced first,
        // so the current file is marked dirty
        this.syncEditorToCurrentFile();

        const success = await this.fs.scopedWrite((fs) => {
            return this.saveToFsWithFs(fs);
                //            
                // let success = true;
                // const _yield = createYielder(64);
                // for (const id in this.files) {
                //     const fsFile = this.files[id];
                //     const result = await this.saveChangesToFsForFsFile(
                //         id,
                //         fsFile,
                //         token,
                //     );
                //     if (result.isErr()) {
                //         success = false;
                //     }
                //     await _yield();
                // }
                // return success;
        });

        window.clearTimeout(handle);
        this.dispatcher.dispatch(viewActions.endFileSysSave(success));
        EditorLog.info("save completed");
        return success;
    }

    private async saveToFsWithFs(fs: FsFileSystem): Promise<boolean> {
        let success = true;
        const paths = fs.getOpenedPaths();
        for (let i = 0; i < paths.length; i++) {
            if (!(await this.saveToFsForPath(fs, paths[i]))) {
                success = false;
            }
            await this.fsYield();
        }
        return success;
    }

    private async saveToFsForPath(fs: FsFileSystem, path: string): Promise<boolean> {
        // return await this.fsLock.lockedScope(lockToken, async () => {
        const { err } = await fs.getFile(path).writeIfNewer();
        if (err) {
            const { code, message } = err;
            EditorLog.error(`save failed with code ${code}: ${message}`);
            return false;
        }
        return true;
            //
            // const result = await fsFile.writeIfNewer();
            // if (result.isErr()) {
            // }
            // return result;
        // });
    }

    // private async updateEditorLegacy(
    //     file: FsFile | undefined,
    //     path: string | undefined,
    //     content: string | undefined,
    //     lockToken?: number,
    // ) {
    //     await this.fsLock.lockedScope(lockToken, async (token) => {
    //         // in case we are switching files, sync the current file first
    //         if (this.currentFile !== file) {
    //             await this.syncEditorToCurrentFile(token);
    //             this.currentFile = file;
    //         }
    //         const success = content !== undefined;
    //         this.dispatcher.dispatch(
    //             viewActions.updateOpenedFile({
    //                 openedFile: path,
    //                 currentFileSupported: success,
    //             }),
    //         );
    //
    //         if (success && path !== undefined) {
    //             // this check is necessary because
    //             // some browsers rerenders the editor even if the content is the same (Firefox)
    //             // which causes annoying flickering
    //             if (this.monacoEditor.getValue() !== content) {
    //                 this.monacoEditor.setValue(content);
    //             }
    //
    //             // TODO #20: language feature support
    //             this.switchLanguage(detectLanguageByFileName(path));
    //
    //             await this.attachEditor();
    //             this.isEditorOpen = true;
    //         } else {
    //             this.monacoDom.remove();
    //             this.isEditorOpen = false;
    //         }
    //     });
    // }
    //
    private closeEditor() {
        if (this.currentFile) {
            this.syncEditorToCurrentFile();
            // note: don't close the file in memory,
            // as it may have unsaved content.
            this.currentFile = undefined;
        }
        this.dispatcher.dispatch(
            viewActions.updateOpenedFile({
                openedFile: undefined,
                currentFileSupported: false,
            }),
        );
        this.detachEditor();
    }

    private updateEditor(file: FsFile, content: string | undefined) {
        // in case we are switching files, sync the current file first
        if (this.currentFile !== file) {
            this.syncEditorToCurrentFile();
            this.currentFile = file;
        }
        const currentFileSupported = content !== undefined;
        this.dispatcher.dispatch(
            viewActions.updateOpenedFile({
                openedFile: file.path,
                currentFileSupported,
            }),
        );

        if (!currentFileSupported) {
            return;
        }

        // this check is necessary because
        // some browsers rerenders the editor even if the content is the same (Firefox)
        // which causes annoying flickering
        if (this.monacoEditor.getValue() !== content) {
            this.monacoEditor.setValue(content);
        }

        // TODO #20: language feature support
        this.switchLanguage(detectLanguageByFileName(file.path));

        setTimeout(() => this.attachEditor(), 0);
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

    // public async syncEditorToCurrentFileLegacy(lockToken?: number) {
    //     await this.fsLock.lockedScope(lockToken, async () => {
    //         if (this.currentFile && this.isEditorOpen) {
    //             this.currentFile.setContent(this.monacoEditor.getValue());
    //         }
    //     });
    // }

    /// Sync the text from editor to the in memory file storage
    public syncEditorToCurrentFile(): void {
        if (this.currentFile && this.isEditorOpen) {
            this.currentFile.setText(this.monacoEditor.getValue());
        }
    }

    public async updateDirtyFileList(currentList: string[]) {
        const unsavedFiles = await this.fs.scopedRead(async (fs) => {
            const unsavedFiles = new Set<string>();
            const paths = fs.getOpenedPaths();
            for (let i = 0; i < paths.length; i++) {
                const fsFile = fs.getFile(paths[i]);
                if (fsFile.isDirty()) {
                    unsavedFiles.add(paths[i]);
                }
                await this.fsYield();
            }
            return unsavedFiles;
        });

        // don't update if the list is the same
        // to prevent unnecessary rerenders
        if (unsavedFiles.size === currentList.length) {
            // new API in the future:
            // const needsUpdate = currentSet.symmetricDifference(unsavedFiles).size;
            let needsUpdate = false;
            for (let i = 0; i < currentList.length; i++) {
                if (!unsavedFiles.has(currentList[i])) {
                    needsUpdate = true;
                    break;
                }
            }
            if (!needsUpdate) {
                const currentSet = new Set(currentList);
                for (const path of unsavedFiles) {
                    if (!currentSet.has(path)) {
                        needsUpdate = true;
                        break;
                    }
                }
            }
            if (!needsUpdate) {
                return;
            }
        }
        const newList = Array.from(unsavedFiles);
        this.dispatcher.dispatch(viewActions.setUnsavedFiles(newList));
    }

    private modifiedTimeWhenLastAccessed: { [path: string]: number } = {};
    public getFileContent(
        path: string,
        checkChanged: boolean,
    ): Promise<FsResult<Uint8Array>> {
        return this.fs.scopedWrite((fs) => {
            return this.getFileContentWithFs(fs, path, checkChanged);
        });
    }

    private async getFileContentWithFs(fs: FsFileSystem, path: string, checkChanged: boolean): Promise<FsResult<Uint8Array>> {
        const fsFile = fs.getFile(path);
        if (checkChanged) {
            const modifiedTimeCurrent = await fsFile.getLastModified();
            if (modifiedTimeCurrent.err) {
                return modifiedTimeCurrent;
            }
            const modifiedTimeLast = this.modifiedTimeWhenLastAccessed[path];
            this.modifiedTimeWhenLastAccessed[path] = modifiedTimeCurrent.val;
            if (
                modifiedTimeLast &&
                    modifiedTimeLast >= modifiedTimeCurrent.val
            ) {
                // 1. file was accessed before
                // 2. file was not modified since last access
                return { err: fsErr(FsErr.NotModified, "Not modified") };
            }
        }
        return await fsFile.getBytes();
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
            await this.resizeEditor();
            EditorLog.info("editor attached");
        }
        this.isEditorOpen = true;
    }

    private detachEditor() {
        this.monacoDom.remove();
        this.isEditorOpen = false;
    }
}
