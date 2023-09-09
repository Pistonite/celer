//! Editor logic that wraps monaco editor
import * as monaco from 'monaco-editor'
// eslint-disable-next-line import/no-internal-modules
import editorWorker from 'monaco-editor/esm/vs/editor/editor.worker?worker'
// eslint-disable-next-line import/no-internal-modules
import jsonWorker from 'monaco-editor/esm/vs/language/json/json.worker?worker'
// eslint-disable-next-line import/no-internal-modules
import tsWorker from 'monaco-editor/esm/vs/language/typescript/ts.worker?worker'

import { AppStore, viewActions, viewSelector, settingsSelector } from 'core/store';
import { FileSys, FsFile, FsResultCode, FsResultCodes } from 'low/fs';

import { EditorContainerId, EditorLog, toFsPath } from 'core/editor';
import { isInDarkMode, sleep } from 'low/utils';
import reduxWatch from 'redux-watch';

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

    /// Load changes from the file system for the opened files
    loadChangesFromFs(): Promise<void>;
}

class EditorStateImpl implements EditorState {
    private store: AppStore;
    private fs: FileSys | undefined;

    /// Some operations need to block other operations,
    /// like saving and loading at the same time is probably bad
    ///
    /// Anything that changes files or currentFile or the monaco editor
    /// should lock the fs
    private fsLock: boolean = false;
    /// Opened files
    private files: Record<string, FsFile> = {};
    private currentFile: FsFile | undefined;

    private monacoDom: HTMLDivElement;
    private monacoEditor: monaco.editor.IStandaloneCodeEditor;
    private cleanup: (() => void);

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

        // Subscribe to store updates
        const watchSettings = reduxWatch(() =>
            settingsSelector(store.getState()),
        );
        const unwatchSettings = store.subscribe(
            watchSettings((_newVal, _oldVal) => {
                this.onSettingsUpdate();
            }),
        );
        const watchView = reduxWatch(() =>
            viewSelector(store.getState()),
        );
        const unwatchView = store.subscribe(
            watchView((_newVal, _oldVal) => {
                this.onViewUpdate();
            }),
        );
        this.cleanup = () => {
            window.removeEventListener("resize", resizeHandler);
            unwatchSettings();
            unwatchView();
        };
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
            this.monacoEditor.dispose();
        this.cleanup();
    }

    private onSettingsUpdate() {
            this.resizeEditor();
    }

    private onViewUpdate() {
            this.resizeEditor();
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
        const fsPath = toFsPath(path);
        const idPath = fsPath.path;
      EditorLog.info(`opening ${idPath}`);
        await this.withLockedFs("openFile", async () => {
            if (!this.fs) {
                return;
            }
            if (this.files[idPath]) {
                await this.openFsFile(this.files[idPath]);
                return;
            }
            const fsFile = new FsFile(this.fs, fsPath);
            this.files[idPath] = fsFile;
            await this.openFsFile(fsFile);
        });
    }

    // close the currently opened file
    async closeFile() {
        EditorLog.info("closing file");
        this.ensureLockedFs("closeFile", async () => {
            if (!this.currentFile) {
                return;
            }
            for (const id in this.files) {
                const fsFile = this.files[id];
                if (fsFile === this.currentFile) {
                    EditorLog.info(`closing ${id}`);
                    delete this.files[id];
                    break;
                }
            }
            this.currentFile = undefined;
            this.monacoDom.remove();
            this.store.dispatch(viewActions.updateOpenedFile({
                openedFile: undefined,
                currentFileSupported: true,
            }));
        });
    }

    // WARNING: must only call in locked fs
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
        this.updateEditorValue(path, content.value);
        this.store.dispatch(viewActions.updateOpenedFile({
            openedFile: path,
            currentFileSupported: true,
        }));
        this.currentFile = fsFile;
    }

    // WARNING: must only call in locked fs
    private updateEditorValue(path: string, content: string) {
        this.monacoEditor.setValue(content);
        if (path.endsWith(".js")) {
            const model = this.monacoEditor.getModel();
            if (model) {
                monaco.editor.setModelLanguage(model, "javascript");
            }
        }
        this.attachEditor();
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
        // do this async for any UI size changes to finish
        setTimeout(() => {
            // Resize to 0,0 to force monaco to shrink if needed
            this.monacoEditor.layout({width: 0, height: 0});
            this.monacoEditor.layout();
        }, 0);
    }

    public hasUnsavedChanges(): boolean {
        // TODO: edit not implemented yet so
        return false;
    }

    public async loadChangesFromFs() {
        EditorLog.info("loading changes from filesystem");
        this.store.dispatch(viewActions.startFileSysLoad());
        await sleep(5999);
        const success = await this.withLockedFs("loadChangesFromFs", async () => {
            let success = true;
            for (const id in this.files) {
                const fsFile = this.files[id];
                const result = await this.loadChangesFromFsForFsFile(id, fsFile);
                if (result !== FsResultCodes.Ok) {
                    success = false;
                }
                // sleep some time so the UI doesn't freeze
                await sleep(50);
            }
            return success;
        });
        this.store.dispatch(viewActions.endFileSysLoad(success));
        EditorLog.info("changes loaded from filesystem");
    }

    // WARNING: must only call in locked fs
    async loadChangesFromFsForFsFile(id: string, fsFile: FsFile): Promise<FsResultCode> {
        EditorLog.info(`syncing ${id}`);
        const result = await fsFile.load();
        if (result !== FsResultCodes.Ok) {
            EditorLog.error(`sync failed with code ${result}`);
            if (this.currentFile === fsFile) {
                this.closeFile();
            }
            return result;
        }
        if (this.currentFile === fsFile) {
            const contentResult = await fsFile.getContent();
            if (contentResult.code !== FsResultCodes.Ok) {
                EditorLog.error(`sync failed with code ${contentResult.code}`);
                if (this.currentFile === fsFile) {
                    this.closeFile();
                }
                return contentResult.code;
            }
            this.updateEditorValue(id, contentResult.value);
        }
        return FsResultCodes.Ok;
    }

    /// WARNING: f must not call withLockedFs again - otherwise it will dead lock
    async withLockedFs<T>(reason: string, f: () => Promise<T>): Promise<T> {
        if (this.fsLock) {
            EditorLog.warn(`${reason} waiting for fs to become available...`);
            return await new Promise((resolve) => {
                setTimeout(() => {
                    this.withLockedFs(reason, f).then(resolve);
                }, 1000);
            });
        }
        try {
            this.fsLock = true;
            return await f();
        } catch (e) {
            throw e;
        } finally {
            this.fsLock = false;
        }
    }

    /// Like withLockedFs but will not block if fs is already locked
    async ensureLockedFs<T>(reason: string, f: () => Promise<T>): Promise<T> {
        if (this.fsLock) {
            return await f();
        }
        return await this.withLockedFs(reason, f);
    }

}

