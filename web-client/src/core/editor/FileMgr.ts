import * as monaco from "monaco-editor";

import type {
    FsError,
    FsFile,
    FsFileSystem,
    FsResult,
} from "@pistonite/pure/fs";
import { RwLock } from "@pistonite/pure/sync";

import type { AppDispatcher } from "core/store";
import { viewActions } from "core/store";
import type { CompilerFileAccess } from "core/compiler";
import type { Yielder } from "low/utils";
import { createYielder, sleep, consoleEditor as console } from "low/utils";

import { EditorContainerDOM } from "./dom";
import type { ChangeTracker } from "./ChangeTracker";
import { newModifyTimeBasedTracker } from "./ChangeTracker";

type IStandaloneCodeEditor = monaco.editor.IStandaloneCodeEditor;

/// File manager
///
/// This manages the opened files in the web editor
export class FileMgr implements CompilerFileAccess {
    /// Using a RwLock to ensure the tracked files don't change
    /// while being iterated. Generall, write lock is needed
    /// for close() and getFile() for unopened files
    private fs: RwLock<FsFileSystem>;

    private supportsSave: boolean;

    private currentFile: FsFile | undefined;
    private monacoDom: HTMLDivElement;
    private monacoEditor: IStandaloneCodeEditor;
    /// If the editor is open. Can be false even if currentFile is not undefined, if it's not a text file
    private isEditorOpen = false;

    /// Yielder for file system operations
    private fsYield: Yielder;
    private dispatcher: AppDispatcher;

    private tracker: ChangeTracker;

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
        this.tracker = newModifyTimeBasedTracker();
    }

    public delete() {
        this.closeEditor();
        this.monacoEditor.dispose();
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

    private async listDirWithFs(
        fs: FsFileSystem,
        path: string,
    ): Promise<string[]> {
        const { val: entries, err } = await fs.listDir(path);
        if (err) {
            const { code, message } = err;
            console.error(`listDir failed with code ${code}: ${message}`);
            return [];
        }
        return entries;
    }

    public openFile(path: string): Promise<void> {
        console.info(`opening ${path}`);
        return this.fs.scopedWrite(async (fs) => {
            return this.openFileWithFs(fs, path);
        });
    }

    private async openFileWithFs(
        fs: FsFileSystem,
        path: string,
    ): Promise<void> {
        const fsFile = fs.getFile(path);
        const { val, err } = await fsFile.getText();
        if (err) {
            const { code, message } = err;
            console.error(`openFile failed with code ${code}: ${message}`);
            this.updateEditor(fsFile, undefined);
            return;
        }
        this.updateEditor(fsFile, val);
    }

    public async loadFromFs(): Promise<void> {
        console.info("syncing files from file system to editor...");
        const handle = window.setTimeout(() => {
            this.dispatcher.dispatch(viewActions.startFileSysLoad());
        }, 200);
        // ensure editor changes is synced first,
        // so the current file is marked dirty if needed
        this.syncEditorToCurrentFile();
        let success = await this.fs.scopedWrite((fs) => {
            return this.loadFromFsWithFs(fs);
        });
        if (!success) {
            // failure could be due to project structure change. try again
            console.warn("sync failed, retrying...");
            success = await this.fs.scopedWrite((fs) => {
                return this.loadFromFsWithFs(fs);
            });
        }
        window.clearTimeout(handle);
        this.dispatcher.dispatch(viewActions.endFileSysLoad(success));
        this.dispatcher.dispatch(viewActions.incFileSysSerial());
        console.info("sync completed");
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
    }

    private async loadFromFsForPath(
        fs: FsFileSystem,
        path: string,
    ): Promise<boolean> {
        const fsFile = fs.getFile(path);
        const isCurrentFile = this.currentFile === fsFile;

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
            console.error(`sync failed with code ${code}: ${message}`);
            if (!fsFile.isDirty()) {
                // if the file is not dirty, we close the file
                // in case it doesn't exist on disk anymore
                // and to avoid error on the next save
                console.info(`closing ${path} due to sync error`);
                if (isCurrentFile) {
                    this.closeEditor();
                }
                fsFile.close();
            }
        }

        return loadError === undefined;
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
            console.error("save not supported!");
            console.warn("saveToFs should only be called if save is supported");
            return false;
        }

        console.info("saving changes...");
        const handle = window.setTimeout(() => {
            this.dispatcher.dispatch(viewActions.startFileSysSave());
        }, 200);
        // ensure editor changes is synced first,
        // so the current file is marked dirty
        this.syncEditorToCurrentFile();

        const success = await this.fs.scopedWrite((fs) => {
            return this.saveToFsWithFs(fs);
        });

        window.clearTimeout(handle);
        this.dispatcher.dispatch(viewActions.endFileSysSave(success));
        console.info("save completed");
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

    private async saveToFsForPath(
        fs: FsFileSystem,
        path: string,
    ): Promise<boolean> {
        // return await this.fsLock.lockedScope(lockToken, async () => {
        const { err } = await fs.getFile(path).writeIfNewer();
        if (err) {
            const { code, message } = err;
            console.error(`save failed with code ${code}: ${message}`);
            return false;
        }
        return true;
    }

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

    public getFileContent(
        path: string,
        checkChanged: boolean,
    ): Promise<FsResult<Uint8Array>> {
        return this.fs.scopedWrite((fs) => {
            return this.getFileContentWithFs(fs, path, checkChanged);
        });
    }

    private async getFileContentWithFs(
        fs: FsFileSystem,
        path: string,
        checkChanged: boolean,
    ): Promise<FsResult<Uint8Array>> {
        const fsFile = fs.getFile(path);
        if (checkChanged) {
            const notModified =
                await this.tracker.checkModifiedSinceLastAccess(fsFile);
            if (notModified.err) {
                return notModified;
            }
        }
        return await fsFile.getBytes();
    }

    private async attachEditor() {
        let div = EditorContainerDOM.get();
        while (!div) {
            console.warn("editor container not found. Will try again.");
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
            console.info("editor attached");
        }
        this.isEditorOpen = true;
    }

    private detachEditor() {
        this.monacoDom.remove();
        this.isEditorOpen = false;
    }
}

function detectLanguageByFileName(fileName: string): string {
    if (fileName.match(/\.(j|t)s$/i)) {
        return "typescript";
    }
    if (fileName.match(/\.ya?ml/i)) {
        return "yaml";
    }
    if (fileName.match(/\.json/i)) {
        return "json";
    }
    return "text";
}
