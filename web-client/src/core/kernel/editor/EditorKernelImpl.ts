import * as monaco from "monaco-editor";
import reduxWatch from "redux-watch";

import {
    AppStore,
    viewSelector,
    settingsSelector,
    viewActions,
    ViewState,
    SettingsState,
    settingsActions,
} from "core/store";
import { EntryPointsSorted, get_entry_points } from "low/celerc";
import { fetchAsBytes } from "low/fetch";
import { FileSys, FsResult } from "low/fs";
import { Result, allocOk, isInDarkMode, sleep, wrapAsync } from "low/utils";

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

    private shouldRecompile = false;

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

        this.compMgr = new CompMgr(store);

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

    public async init(): Promise<void> {
        await this.compMgr.init(
            this.fileMgr.getFileAsBytes.bind(this.fileMgr),
            fetchAsBytes,
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

    public async compile(): Promise<void> {
        this.shouldRecompile = false;
        if (!this.fileMgr.isFsLoaded()) {
            return;
        }
        const entryPath = settingsSelector(this.store.getState()).compilerEntryPath;
        // check if entry path is a valid file
        if (entryPath && !await this.fileMgr.exists(toFsPath(entryPath.split("/")))) {
            EditorLog.warn("entry path is invalid, attempting correction...");
            const newEntryPath = await this.correctEntryPath(entryPath);
            if (newEntryPath !== entryPath) {
                // update asynchronously to avoid infinite blocking loop
                await sleep(0);
                this.store.dispatch(settingsActions.setCompilerEntryPath(newEntryPath));
                return;
            }
        }
        this.compMgr.triggerCompile(entryPath);
    }

    public getEntryPoints(): Promise<Result<EntryPointsSorted, unknown>> {
        if (!this.fileMgr.isFsLoaded()) {
            return Promise.resolve(allocOk([]));
        }
        return wrapAsync(get_entry_points);
    }

    /// Try to correct an invalid entry path
    ///
    /// The invalid entry path may be saved from a previous project.
    /// The function will try to find a valid entry path from the current project.
    /// However, if the same entry path is found in the current project, that will be returned
    private async correctEntryPath(entryPath: string): Promise<string> {
        const entryPointsResult = await this.getEntryPoints();
        if (entryPointsResult.isErr()) {
            return "";
        }
        const newEntryPoints = entryPointsResult.inner();
        if (newEntryPoints.length === 0) {
            return "";
        }
        // if entry point with the same path exists, don't correct it
        for (const [_, path] of newEntryPoints) {
            if (path === entryPath) {
                return path;
            }
        }
        // if entry point with "default" name exists, try that first
        for (const [name, path] of newEntryPoints) {
            if (name === "default") {
                if (path && await this.fileMgr.exists(toFsPath(path.split("/")))) {
                    return path;
                } 
                break;
            }
        }
        // otherwise find the first valid entry point
        for (const [_, path] of newEntryPoints) {
            if (path && await this.fileMgr.exists(toFsPath(path.split("/")))) {
                return path;
            } 
        }
        return "";
    }

    private onSettingsUpdate(oldVal: SettingsState, newVal: SettingsState) {
        this.onResize();
        if (this.fileMgr.isFsLoaded()) {
            if (oldVal.compilerEntryPath !== newVal.compilerEntryPath) {
                this.compile();
            }
        }
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
