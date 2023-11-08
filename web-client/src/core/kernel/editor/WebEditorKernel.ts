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
import { isInDarkMode } from "low/utils";

import { EditorKernel } from "./EditorKernel";
import { EditorLog, toFsPath } from "./utils";
import { IdleMgr } from "./IdleMgr";
import { FileMgr } from "./FileMgr";

EditorLog.info("loading web editor kernel");

export const initWebEditor = (store: AppStore): EditorKernel => {
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
    return new WebEditorKernel(store);
}

class WebEditorKernel implements EditorKernel, FileAccess {
    private store: AppStore;

    private idleMgr: IdleMgr;
    private fileMgr: FileMgr;

    private shouldRecompile = false;
    private compile: () => void = () => {
        EditorLog.warn("compiler not binded");
    };

    private cleanup: () => void;

    constructor(store: AppStore) {
        this.store = store;

        this.idleMgr = new IdleMgr(this.onIdle.bind(this));

        const monacoDom = document.createElement("div");
        monacoDom.id = "monaco-editor";
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
        this.fileMgr = new FileMgr(monacoDom, monacoEditor, store);

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

    public init(compile: () => void): void {
        this.compile = compile;
    }

    /// Reset the editor with a new file system. Unsaved changes will be lost
    public async reset(fs?: FileSys): Promise<void> {
        await this.idleMgr.pauseIdleScope(async () => {
            await this.fileMgr.reset(fs);
        });
    }

    public delete() {
        EditorLog.info("deleting web editor");
        // @ts-expect-error setting to undefined to make sure the editor is not double-deleted
        window.__theEditorKernel = undefined;
        this.reset();
        this.cleanup();
    }

    public async listDir(
        path: string[],
        isUserAction: boolean,
    ): Promise<string[]> {
        if (isUserAction) {
            this.idleMgr.notifyActivity();
        }
        // probably fine with not locking idle mgr here
        return await this.fileMgr.listDir(path);
    }

    /// Open a file in the editor
    public async openFile(
        path: string[],
        isUserAction: boolean,
    ): Promise<FsResult<void>> {
        const fsPath = toFsPath(path);
        const result = await this.idleMgr.pauseIdleScope(async () => {
            return await this.fileMgr.openFile(fsPath);
        });
        if (isUserAction) {
            this.idleMgr.notifyActivity();
        }
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

    public async loadChangesFromFs(
        isUserAction: boolean,
    ): Promise<FsResult<void>> {
        if (isUserAction) {
            this.idleMgr.notifyActivity();
        }
        const result = await this.idleMgr.pauseIdleScope(async () => {
            return await this.fileMgr.loadChangesFromFs();
        });
        this.compile();
        return result;
    }

    public async saveChangesToFs(
        isUserAction: boolean,
    ): Promise<FsResult<void>> {
        if (isUserAction) {
            this.idleMgr.notifyActivity();
        }
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
        return this;
    }

    public async getFileContent(
        path: string,
        checkChanged: boolean,
    ): Promise<FsResult<Uint8Array>> {
        return await this.fileMgr.getFileAsBytes(path, checkChanged);
    }

    public isAvailable(): boolean {
        return this.fileMgr.isFsLoaded();
    }

    public async exists(path: string): Promise<boolean> {
        return await this.fileMgr.exists(toFsPath(path.split("/")));
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
    private async onIdle(isLong: boolean, duration: number) {
        if (!this.fileMgr.isFsLoaded()) {
            return;
        }
        const { autoLoadActive, unsavedFiles } = viewSelector(
            this.store.getState(),
        );

        // pull changes from monaco editor first to make sure current file is marked dirty if needed
        await this.fileMgr.syncEditorToCurrentFile();

        let shouldRecompile = this.shouldRecompile;

        if (isLong) {
            const {
                autoSaveEnabled,
                autoLoadEnabled,
                deactivateAutoLoadAfterMinutes,
            } = settingsSelector(this.store.getState());

            let shouldRerenderFs = false;

            if (autoLoadActive) {
                if (autoLoadEnabled) {
                    await this.loadChangesFromFs(false /* isUserAction */);
                    // make sure file system view is rerendered in case there are directory updates
                    shouldRerenderFs = true;
                    // trigger a compile after reloading fs
                    shouldRecompile = true;
                }
                if (deactivateAutoLoadAfterMinutes > 0) {
                    if (duration > deactivateAutoLoadAfterMinutes * 60 * 1000) {
                        EditorLog.info(
                            "auto load deactivated due to inactivity",
                        );
                        this.store.dispatch(
                            viewActions.setAutoLoadActive(false),
                        );
                    }
                }
            }

            if (autoSaveEnabled) {
                await this.saveChangesToFs(false /* isUserAction */);
                // make sure file system view is rerendered in case there are directory updates
                shouldRerenderFs = true;
            }
            if (shouldRerenderFs) {
                this.store.dispatch(viewActions.incFileSysSerial());
            }
        }

        // do this last so we can get the latest save status after auto-save
        this.fileMgr.updateDirtyFileList(unsavedFiles);
        if (shouldRecompile) {
            this.compile();
        }
    }
}
