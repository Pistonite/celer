import * as monaco from "monaco-editor";
// eslint-disable-next-line import/no-internal-modules
import editorWorker from "monaco-editor/esm/vs/editor/editor.worker?worker";
// eslint-disable-next-line import/no-internal-modules
import jsonWorker from "monaco-editor/esm/vs/language/json/json.worker?worker";
// eslint-disable-next-line import/no-internal-modules
import tsWorker from "monaco-editor/esm/vs/language/typescript/ts.worker?worker";
import reduxWatch from "redux-watch";

import { FsFileSystem } from "pure/fs";

import {
    AppStore,
    viewSelector,
    settingsSelector,
    viewActions,
    ViewState,
    SettingsState,
} from "core/store";
import { CompilerFileAccess } from "core/compiler";
import { isInDarkMode, IdleMgr, DOMId, consoleEditor as console } from "low/utils";

import { FileMgr } from "./FileMgr";

import { EditorKernel } from "./EditorKernel";
import { EditorKernelAccess } from "./EditorKernelAccess";

console.info("loading web editor kernel");

export const initWebEditor = (
    kernel: EditorKernelAccess,
    fs: FsFileSystem,
    store: AppStore,
): EditorKernel => {
    console.info("creating web editor");
    window.MonacoEnvironment = {
        getWorker(_, label) {
            if (label === "json") {
                return new jsonWorker();
            }
            if (label === "typescript" || label === "javascript") {
                return new tsWorker();
            }
            return new editorWorker();
        },
    };
    return new WebEditorKernel(kernel, fs, store);
};

const MonacoEditorDOM = new DOMId("monaco-editor");

class WebEditorKernel implements EditorKernel {
    private deleted = false;
    private store: AppStore;

    private idleMgr: IdleMgr;
    private fileMgr: FileMgr;

    private shouldRecompile = false;
    private kernel: EditorKernelAccess;

    private cleanup: () => void;

    constructor(kernel: EditorKernelAccess, fs: FsFileSystem, store: AppStore) {
        this.store = store;
        this.kernel = kernel;

        this.idleMgr = new IdleMgr(
            5000,
            1000,
            2,
            5,
            20000,
            this.onIdle.bind(this),
        );

        const monacoDom = document.createElement("div");
        monacoDom.id = MonacoEditorDOM.id;
        monacoDom.style.height = "100%";
        const monacoEditor = monaco.editor.create(monacoDom, {
            theme: isInDarkMode() ? "vs-dark" : "vs",
            tabSize: 2,
        });
        monacoEditor.onKeyDown(() => {
            this.idleMgr.notifyActivity();
            this.shouldRecompile = true;
        });
        monacoEditor.onMouseDown(() => {
            this.idleMgr.notifyActivity();
        });
        this.fileMgr = new FileMgr(fs, monacoDom, monacoEditor, store);

        const resizeHandler = this.onResize.bind(this);
        window.addEventListener("resize", resizeHandler);

        // Subscribe to store updates
        const watchSettings = reduxWatch(() =>
            settingsSelector(store.getState()),
        );
        const unwatchSettings = store.subscribe(
            watchSettings((newVal, oldVal) => {
                this.onSettingsUpdate(newVal, oldVal);
            }),
        );
        const watchView = reduxWatch(() => viewSelector(store.getState()));
        const unwatchView = store.subscribe(
            watchView((newVal, oldVal) => {
                this.onViewUpdate(oldVal, newVal);
            }),
        );

        this.cleanup = () => {
            window.removeEventListener("resize", resizeHandler);
            unwatchSettings();
            unwatchView();
            this.idleMgr.stop();
            this.fileMgr.delete();
        };

        this.idleMgr.start();
    }

    public delete() {
        console.info("deleting web editor");
        if (this.deleted) {
            console.warn("editor already deleted");
            return;
        }
        this.cleanup();
        this.deleted = true;
    }

    public notifyActivity(): void {
        this.idleMgr.notifyActivity();
    }

    public listDir(path: string): Promise<string[]> {
        // probably fine with not locking idle mgr here
        // since it's read-only access
        return this.fileMgr.listDir(path);
    }

    /// Open a file in the editor
    public openFile(path: string): Promise<void> {
        return this.idleMgr.pauseIdleScope(() => {
            return this.fileMgr.openFile(path);
        });
    }

    public hasUnsavedChanges(): Promise<boolean> {
        return this.idleMgr.pauseIdleScope(() => {
            return this.fileMgr.hasUnsavedChanges();
        });
    }

    public hasUnsavedChangesSync(): boolean {
        return this.fileMgr.hasUnsavedChangesSync();
    }

    public async loadFromFs(): Promise<void> {
        await this.idleMgr.pauseIdleScope(() => {
            return this.fileMgr.loadFromFs();
        });
        this.kernel.reloadDocument();
    }

    public async saveToFs(): Promise<void> {
        await this.idleMgr.pauseIdleScope(() => {
            return this.fileMgr.saveToFs();
        });

        const { unsavedFiles } = viewSelector(this.store.getState());
        await this.fileMgr.updateDirtyFileList(unsavedFiles);
    }

    // === CompilerFileAccess ===
    public getFileAccess(): CompilerFileAccess {
        return this.fileMgr;
    }

    private onSettingsUpdate(_oldVal: SettingsState, _newVal: SettingsState) {
        this.onResize();
    }

    private onViewUpdate(oldVal: ViewState, newVal: ViewState) {
        // view state can change often
        // so we only want to react to changes that affect the editor
        if (oldVal.isEditingLayout !== newVal.isEditingLayout) {
            this.onResize();
        }
    }

    private onResize() {
        this.fileMgr.resizeEditor();
    }

    /// The editor does the following things on idle:
    /// 1. If there's an opened file, pull changes from monaco editor
    /// 2. If any file has been touched, run compiler
    /// 3. Update unsaved file list in view store
    ///
    /// Long idle only:
    /// - If auto save is enabled, save changes to fs
    /// - If auto load is enabled, load changes from fs
    private async onIdle(isLong: boolean) {
        const { unsavedFiles } = viewSelector(this.store.getState());

        // pull changes from monaco editor first to make sure current file is marked dirty if needed
        this.fileMgr.syncEditorToCurrentFile();
        if (isLong) {
            const { autoSaveEnabled } = settingsSelector(this.store.getState());

            let shouldRerenderFs = false;

            if (autoSaveEnabled) {
                await this.saveToFs();
                // make sure file system view is rerendered in case there are directory updates
                shouldRerenderFs = true;
            }
            if (shouldRerenderFs) {
                this.store.dispatch(viewActions.incFileSysSerial());
            }
        }

        // do this last so we can get the latest save status after auto-save
        await this.fileMgr.updateDirtyFileList(unsavedFiles);
        if (this.shouldRecompile) {
            this.kernel.reloadDocument();
            this.shouldRecompile = false;
        }
    }
}
