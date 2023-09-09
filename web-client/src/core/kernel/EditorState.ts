//! Editor logic that wraps monaco editor
import * as monaco from 'monaco-editor'
// eslint-disable-next-line import/no-internal-modules
import editorWorker from 'monaco-editor/esm/vs/editor/editor.worker?worker'
// eslint-disable-next-line import/no-internal-modules
import jsonWorker from 'monaco-editor/esm/vs/language/json/json.worker?worker'
// eslint-disable-next-line import/no-internal-modules
import tsWorker from 'monaco-editor/esm/vs/language/typescript/ts.worker?worker'

import { AppStore, viewActions } from 'core/store';
import { FileSys, FsFile, FsResultCode, FsResultCodes, fsRootPath } from 'low/fs';

import { EditorContainerId, EditorLog, toFsPath } from 'core/editor';
import { isInDarkMode } from 'low/utils';

EditorLog.info("loading editor module");

declare global {
    interface Window {
        __theEditorState: EditorState;
    }
}

export const initEditor = (store: AppStore): EditorState => {
    if (window.__theEditorState) {
        window.__theEditorState.delete();
    }
    EditorLog.info("creating editor");
    window.MonacoEnvironment = {
        getWorker(_, label) {
            if (label === 'json') {
                return new jsonWorker()
            }
            if (label === 'typescript' || label === 'javascript') {
                return new tsWorker()
            }
            return new editorWorker()
        }
    };

    const editor = new EditorStateImpl(store);
    window.__theEditorState = editor;
    return editor;
}

export interface EditorState {
    /// Reset the editor with a new file system. Unsaved changes will be lost
    reset(fs?: FileSys): void;

    /// Delete the editor state and free resources
    delete(): void;

    /// Request directory listing
    ///
    /// See EditorTree for input/output format
    /// On failure this returns empty array. This function will not throw
    listDir(path: string[]): Promise<string[]>;

    /// Open a file in the editor
    openFile(path: string[]): Promise<void>;

    /// Check if there are unsaved changes
    hasUnsavedChanges(): boolean;

    /// Load changes from the file system
    loadChangesFromFs(): Promise<void>;
}

class EditorStateImpl implements EditorState {
    private store: AppStore;
    private fs: FileSys | undefined;
    /// Opened files
    private files: Record<string, FsFile> = {};
    private currentFile: FsFile | undefined;

    private monacoDom: HTMLDivElement;
    private monacoEditor: monaco.editor.IStandaloneCodeEditor;
    private cleanupMonaco: (() => void);

    constructor(store: AppStore) {
        this.store = store;
        this.monacoDom = document.createElement("div");
        this.monacoDom.id = "monaco-editor";
        this.monacoEditor = monaco.editor.create(this.monacoDom, {
            theme: isInDarkMode() ? "vs-dark" : "vs",
            });

        const resizeHandler = () => {
            this.resizeEditor();
        };
        window.addEventListener("resize", resizeHandler);
        this.cleanupMonaco = () => {
            window.removeEventListener("resize", resizeHandler);
            this.monacoEditor.dispose();
        }
    }

    /// Reset the editor with a new file system. Unsaved changes will be lost
    public reset(fs?: FileSys) {
        EditorLog.info("resetting editor");
        this.fs = fs;
        this.store.dispatch(viewActions.updateOpenedFile({
            openedFile: undefined,
            currentFileSupported: true,
        }));
        if (fs) {
            this.store.dispatch(viewActions.updateFileSys({
                rootPath: fs.getRootName(),
                supportsSave: fs.isWritable(),
            }));
        } else {
            this.store.dispatch(viewActions.updateFileSys({
                rootPath: undefined,
                supportsSave: true,
            }));
        }
        this.monacoDom.remove();
    }

    public delete() {
        EditorLog.info("deleting editor");
        this.reset();
        this.cleanupMonaco();
    }

    /// Request directory listing
    ///
    /// See EditorTree for input/output format
    /// On failure this returns empty array. This function will not throw
    public async listDir(path: string[]): Promise<string[]> {
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
    }

    /// Open a file in the editor
    public async openFile(path: string[]) {
        if (!this.fs) {
            return;
        }
        const fsPath = toFsPath(path);
        const idPath = fsPath.path;
      EditorLog.info(`opening ${idPath}`);
        if (this.files[idPath]) {
            await this.openFsFile(this.files[idPath]);
            return;
        }
        const fsFile = new FsFile(this.fs, fsPath);
        this.files[idPath] = fsFile;
        await this.openFsFile(fsFile);
    }

    private async openFsFile(fsFile: FsFile) {
        const path = fsFile.getDisplayPath();
        const content = await fsFile.getContent();
        if (content.code !== FsResultCodes.Ok) {
            EditorLog.error(`openFsFile failed with code ${content.code}`);
            this.store.dispatch(viewActions.updateOpenedFile({
                openedFile: path,
                currentFileSupported: false,
            }));
            return;
        }
        this.monacoEditor.setValue(content.value);
        if (path.endsWith(".js")) {
            const model = this.monacoEditor.getModel();
            if (model) {
                monaco.editor.setModelLanguage(model, "javascript");
            }
        }
        this.attachEditor();
        this.store.dispatch(viewActions.updateOpenedFile({
            openedFile: path,
            currentFileSupported: true,
        }));
        this.currentFile = fsFile;
    }

    private attachEditor() {
        const div = document.getElementById(EditorContainerId);
        if (!div) {
            EditorLog.warn("editor container not found. Will try again.");
            setTimeout(() => {
                this.attachEditor();
            }, 100);
            return;
        }
        div.childNodes.forEach((node) => {
            node.remove();
        });
        div.appendChild(this.monacoDom);
        this.resizeEditor();
        EditorLog.info("editor attached");
    }

    private resizeEditor() {
        // Resize to 0,0 to force monaco to shrink if needed
        this.monacoEditor.layout({width: 0, height: 0});
        this.monacoEditor.layout();
    }

    public hasUnsavedChanges(): boolean {
        // TODO: edit not implemented yet so
        return false;
    }

    public async loadChangesFromFs() {
        console.log("TODO");
    }

}

