import * as monaco from "monaco-editor";
import reduxWatch from "redux-watch";

import {
    AppStore,
    viewSelector,
    settingsSelector,
    viewActions,
    ViewState,
} from "core/store";
import { FileSys, FsResult } from "low/fs";
import { isInDarkMode } from "low/utils";

import { EditorKernel } from "./EditorKernel";
import { EditorLog, toFsPath } from "./utils";
import { IdleMgr } from "./IdleMgr";
import { FileMgr } from "./FileMgr";
import { CompMgr } from "./CompMgr";

export class EditorKernelImpl implements EditorKernel {
    private store: AppStore;

    private idleMgr: IdleMgr;
    private fileMgr: FileMgr;
    private compMgr: CompMgr;

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
        });
        monacoEditor.onMouseDown(() => {
            this.idleMgr.notifyActivity();
        });
        this.fileMgr = new FileMgr(monacoDom, monacoEditor, store);

        this.compMgr = new CompMgr(store);

        const resizeHandler = this.onResize.bind(this);
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

    public async init(): Promise<void> {
        await this.compMgr.init(
            this.fileMgr.getFileAsBytes.bind(this.fileMgr),
            this.fileMgr.wasFileChangedSinceLastCompile.bind(this.fileMgr),
        );
    }

    /// Reset the editor with a new file system. Unsaved changes will be lost
    public async reset(fs?: FileSys): Promise<void> {
        await this.idleMgr.pauseIdleScope(async () => {
            await this.fileMgr.reset(fs);
            this.compile();
        });
    }

    public delete() {
        EditorLog.info("deleting editor");
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
        return await this.idleMgr.pauseIdleScope(async () => {
            return await this.fileMgr.loadChangesFromFs();
        });
    }

    public async saveChangesToFs(isUserAction: boolean): Promise<FsResult<void>> {
        if (isUserAction) {
            this.idleMgr.notifyActivity();
        }
        return await this.idleMgr.pauseIdleScope(async () => {
            return await this.fileMgr.saveChangesToFs();
        });
    }

    public compile(): void {
        if (!this.fileMgr.isFsLoaded()) {
            return;
        }
        this.compMgr.triggerCompile();
    }

    public cancelCompile(): void {
        this.compMgr.cancel();
    }

    private onSettingsUpdate() {
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
        EditorLog.info(
            "idle" + (isLong ? " (long)" : "") + ` duration= ${duration}ms`,
        );
        const { autoLoadActive, unsavedFiles } = viewSelector(
            this.store.getState(),
        );

        // pull changes from monaco editor first to make sure current file is marked dirty if needed
        await this.fileMgr.syncEditorToCurrentFile();

        if (isLong) {
            const {
                autoSaveEnabled,
                autoLoadEnabled,
                deactivateAutoLoadAfterMinutes,
            } = settingsSelector(this.store.getState());

            let shouldRerenderFs = false;

            if (autoLoadActive) {
                if (autoLoadEnabled) {
                    EditorLog.info("auto loading changes...");
                    await this.loadChangesFromFs(false /* isUserAction */);
                    // make sure file system view is rerendered in case there are directory updates
                    shouldRerenderFs = true;
                    // trigger a compile after reloading fs
                    this.compile();
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
                EditorLog.info("auto saving changes...");
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

        if (await this.fileMgr.needsRecompile()) {
            this.compile();
        }
    }
}
