import reduxWatch from "redux-watch";

import {
    AppStore,
    viewSelector,
    settingsSelector,
    viewActions,
    ViewState,
} from "core/store";
import { FileSys, FsResultCode } from "low/fs";

import { EditorKernel } from "./EditorKernel";
import { EditorLog, toFsPath } from "./utils";
import { IdleMgr } from "./IdleMgr";
import { FileMgr } from "./FileMgr";

export class EditorKernelImpl implements EditorKernel {
    private store: AppStore;

    private idleMgr: IdleMgr;
    private fileMgr: FileMgr;

    private cleanup: () => void;

    constructor(store: AppStore) {
        this.store = store;
        this.fileMgr = new FileMgr(store);

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

        this.idleMgr = new IdleMgr(this.onIdle.bind(this));

        this.cleanup = () => {
            window.removeEventListener("resize", resizeHandler);
            unwatchSettings();
            unwatchView();
            this.idleMgr.stop();
            this.fileMgr.delete();
        };

        this.idleMgr.start();
    }

    /// Reset the editor with a new file system. Unsaved changes will be lost
    public async reset(fs?: FileSys): Promise<void> {
        EditorLog.info("resetting editor");
        await this.idleMgr.pauseIdleScope(async () => {
            await this.fileMgr.reset(fs);
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
    ): Promise<FsResultCode> {
        const fsPath = toFsPath(path);
        const result = await this.idleMgr.pauseIdleScope(async () => {
            return await this.fileMgr.openFile(fsPath);
        });
        if (isUserAction) {
            this.idleMgr.notifyActivity();
        }
        return result;
    }

    public hasUnsavedChanges(): boolean {
        // TODO: edit not implemented yet so
        return false;
    }

    public async loadChangesFromFs(
        isUserAction: boolean,
    ): Promise<FsResultCode> {
        if (isUserAction) {
            this.idleMgr.notifyActivity();
        }
        return await this.idleMgr.pauseIdleScope(async () => {
            return await this.fileMgr.loadChangesFromFs();
        });
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

    private async onIdle(isLong: boolean, duration: number) {
        if (!this.fileMgr.isFsLoaded()) {
            return;
        }
        EditorLog.info(
            "idle" + (isLong ? " (long)" : "") + ` duration= ${duration}ms`,
        );
        if (isLong) {
            const { autoLoadActive } = viewSelector(this.store.getState());
            if (autoLoadActive) {
                const { autoLoadEnabled, deactivateAutoLoadAfterMinutes } =
                    settingsSelector(this.store.getState());
                if (autoLoadEnabled) {
                    EditorLog.info("auto loading changes...");
                    await this.loadChangesFromFs(false /* isUserAction */);
                }
                if (deactivateAutoLoadAfterMinutes > 0) {
                    if (duration > deactivateAutoLoadAfterMinutes * 60 * 1000) {
                        EditorLog.info("deactivating auto load...");
                        this.store.dispatch(
                            viewActions.setAutoLoadActive(false),
                        );
                    }
                }
            }
        }
    }
}
