import * as monaco from "monaco-editor";
// eslint-disable-next-line import/no-internal-modules
import editorWorker from "monaco-editor/esm/vs/editor/editor.worker?worker";
// eslint-disable-next-line import/no-internal-modules
import jsonWorker from "monaco-editor/esm/vs/language/json/json.worker?worker";
// eslint-disable-next-line import/no-internal-modules
import tsWorker from "monaco-editor/esm/vs/language/typescript/ts.worker?worker";
import reduxWatch from "redux-watch";

import {
    AppStore,
    viewSelector,
    settingsSelector,
    viewActions,
    ViewState,
    SettingsState,
} from "core/store";
import { FileAccess, FileSys, FsResult } from "low/fs";
import { isInDarkMode, IdleMgr, DOMId } from "low/utils";

import { EditorKernel } from "./EditorKernel";
import { EditorLog, toFsPath } from "./utils";
import { FileMgr } from "./FileMgr";
import { KernelAccess } from "./KernelAccess";

EditorLog.info("loading web editor kernel");

export const initWebEditor = (
    kernelAccess: KernelAccess,
    fileSys: FileSys,
    store: AppStore,
): EditorKernel => {
    EditorLog.info("creating web editor");
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
    return new WebEditorKernel(kernelAccess, fileSys, store);
};

const MonacoEditorDOM = new DOMId("monaco-editor");
MonacoEditorDOM.style({
    height: "100%",
});

class WebEditorKernel implements EditorKernel {
    private deleted = false;
    private store: AppStore;

    private idleMgr: IdleMgr;
    private fileMgr: FileMgr;

    private shouldRecompile = false;
    private kernelAccess: KernelAccess;

    private cleanup: () => void;

    constructor(kernelAccess: KernelAccess, fileSys: FileSys, store: AppStore) {
        this.store = store;
        this.kernelAccess = kernelAccess;

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
        this.fileMgr = new FileMgr(fileSys, monacoDom, monacoEditor, store);

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
        EditorLog.info("deleting web editor");
        if (this.deleted) {
            EditorLog.warn("editor already deleted");
            return;
        }
        this.cleanup();
        this.deleted = true;
    }

    public notifyActivity(): void {
        this.idleMgr.notifyActivity();
    }

    public async listDir(path: string[]): Promise<string[]> {
        // probably fine with not locking idle mgr here
        return await this.fileMgr.listDir(path);
    }

    /// Open a file in the editor
    public async openFile(path: string[]): Promise<FsResult<void>> {
        const fsPath = toFsPath(path);
        const result = await this.idleMgr.pauseIdleScope(async () => {
            return await this.fileMgr.openFile(fsPath);
        });
        return result;
    }

    public async hasUnsavedChanges(): Promise<boolean> {
        return await this.idleMgr.pauseIdleScope(async () => {
            return await this.fileMgr.hasUnsavedChanges();
        });
    }

    public hasUnsavedChangesSync(): boolean {
        return this.fileMgr.hasUnsavedChangesSync();
    }

    public async loadChangesFromFs(): Promise<FsResult<void>> {
        const result = await this.idleMgr.pauseIdleScope(async () => {
            return await this.fileMgr.loadChangesFromFs();
        });
        this.kernelAccess.reloadDocument();
        return result;
    }

    public async saveChangesToFs(): Promise<FsResult<void>> {
        const result = await this.idleMgr.pauseIdleScope(async () => {
            return await this.fileMgr.saveChangesToFs();
        });
        if (result.isErr()) {
            return result;
        }
        const { unsavedFiles } = viewSelector(this.store.getState());
        this.fileMgr.updateDirtyFileList(unsavedFiles);
        return result.makeOk(undefined);
    }

    // === FileAccess ===
    public getFileAccess(): FileAccess {
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
        await this.fileMgr.syncEditorToCurrentFile();
        if (isLong) {
            const { autoSaveEnabled } = settingsSelector(this.store.getState());

            let shouldRerenderFs = false;

            if (autoSaveEnabled) {
                await this.saveChangesToFs();
                // make sure file system view is rerendered in case there are directory updates
                shouldRerenderFs = true;
            }
            if (shouldRerenderFs) {
                this.store.dispatch(viewActions.incFileSysSerial());
            }
        }

        // do this last so we can get the latest save status after auto-save
        this.fileMgr.updateDirtyFileList(unsavedFiles);
        if (this.shouldRecompile) {
            this.kernelAccess.reloadDocument();
            this.shouldRecompile = false;
        }
    }
}
