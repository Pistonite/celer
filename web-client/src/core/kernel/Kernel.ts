import reduxWatch from "redux-watch";

import {
    AppState,
    AppStore,
    SettingsState,
    documentActions,
    initStore,
    saveSettings,
    settingsActions,
    settingsSelector,
    viewSelector,
    viewActions,
} from "core/store";
import { isRecompileNeeded } from "core/doc";
import { ExpoDoc, ExportRequest } from "low/celerc";
import { console, Logger, isInDarkMode } from "low/utils";
import type { FileSys, FsResult } from "low/fs";

import type { CompilerKernel } from "./compiler";
import type { EditorKernel } from "./editor";
import { KeyMgr } from "./KeyMgr";
import { WindowMgr } from "./WindowMgr";
import { AlertMgr } from "./AlertMgr";

type InitUiFunction = (
    kernel: Kernel,
    store: AppStore,
    isDarkMode: boolean,
) => () => void;

/// The kernel class
///
/// The kernel owns all global resources like the redux store.
/// It is also responsible for mounting react to the DOM and
/// handles the routing.
export class Kernel {
    /// The logger
    private log: Logger;
    /// The store
    ///
    /// The kernel owns the store. The store is shared
    /// between app boots (i.e. when switching routes)
    private store: AppStore;
    /// The link tag that loads the theme css
    private themeLinkTag: HTMLLinkElement | null = null;
    /// The function to initialize react
    private initReact: InitUiFunction;
    /// The function to unmount react
    private cleanupUi: (() => void) | null = null;

    // Alert API
    private alertMgr: AlertMgr;

    // Editor API
    // The editor is owned by the kernel because the toolbar needs access
    private editor: EditorKernel | null = null;

    // Compiler API
    private compiler: CompilerKernel | null = null;

    constructor(initReact: InitUiFunction) {
        this.log = new Logger("ker");
        this.initReact = initReact;
        this.log.info("starting application");
        this.store = this.initStore();
        this.alertMgr = new AlertMgr(this.store);
    }

    /// Initialize the store
    private initStore(): AppStore {
        this.log.info("initializing store...");
        const store = initStore();
        // switch theme on boot base on store settings
        this.switchTheme(settingsSelector(store.getState()).theme);

        const watchSettings = reduxWatch(() =>
            settingsSelector(store.getState()),
        );

        store.subscribe(
            watchSettings((newVal: SettingsState, oldVal: SettingsState) => {
                // save settings to local storage
                this.log.info("saving settings...");
                saveSettings(newVal);

                // switch theme
                if (newVal.theme !== oldVal.theme) {
                    this.switchTheme(newVal.theme);
                }
            }),
        );

        const watchAll = reduxWatch(() => store.getState());
        store.subscribe(
            watchAll(async (newVal: AppState, oldVal: AppState) => {
                if (await isRecompileNeeded(newVal, oldVal)) {
                    console.info("reloading document due to state change...");
                    if (viewSelector(newVal).stageMode === "edit") {
                        const compiler = await this.getCompiler();
                        compiler.compile();
                    } else {
                        // TODO #26: reload doc from server
                    }
                }
            }),
        );

        return store;
    }

    /// Start the application. Cleans up previous application if needed
    public init() {
        this.initStage();
        this.initUi();

        window.addEventListener("beforeunload", (e) => {
            if (this.editor && this.editor.hasUnsavedChangesSync()) {
                e.preventDefault();
                return (e.returnValue =
                    "There are unsaved changes in the editor which will be lost. Are you sure you want to leave?");
            }
        });
    }

    /// Initialize stage info based on window.location
    private async initStage() {
        this.log.info("initializing stage...");
        const path = window.location.pathname;
        if (path === "/edit") {
            const { initCompiler } = await import("./compiler");
            const compiler = initCompiler(this.store);
            this.compiler = compiler;

            this.store.dispatch(viewActions.setStageMode("edit"));
        } else {
            this.store.dispatch(viewActions.setStageMode("view"));
        }
    }

    /// Initialize UI related stuff
    private initUi() {
        this.log.info("initializing ui...");
        if (this.cleanupUi) {
            this.log.info("unmounting previous ui");
            this.cleanupUi();
        }
        const isDarkMode = isInDarkMode();
        const unmountReact = this.initReact(this, this.store, isDarkMode);

        // key binding handler
        const keyMgr = new KeyMgr(this.store);
        const unlistenKeyMgr = keyMgr.listen();

        // window handlers
        const windowMgr = new WindowMgr(this.store);
        const unlistenWindowMgr = windowMgr.listen();

        this.cleanupUi = () => {
            unmountReact();
            unlistenKeyMgr();
            unlistenWindowMgr();
        };
    }

    /// Switch theme to the given theme
    ///
    /// This replaces the theme css link tag.
    /// The theme files are built by the web-themes project.
    public switchTheme(theme: string) {
        if (!this.themeLinkTag) {
            const e = document.createElement("link");
            e.rel = "stylesheet";
            e.type = "text/css";
            this.themeLinkTag = e;
            const head = document.querySelector("head");
            if (!head) {
                this.log.error("Could not find head tag to attach theme to");
                return;
            }
            head.appendChild(e);
        }
        this.themeLinkTag.href = `/themes/${theme}.min.css`;
    }

    public getAlertMgr(): AlertMgr {
        return this.alertMgr;
    }

    public getEditor(): EditorKernel | null {
        return this.editor;
    }

    /// Get or load the compiler
    public async getCompiler(): Promise<CompilerKernel> {
        const state = this.store.getState();
        const stageMode = viewSelector(state).stageMode;
        if (stageMode !== "edit") {
            this.log.error(
                "compiler is not available in view mode. This is a bug!",
            );
            throw new Error("compiler is not available in view mode");
        }
        if (!this.compiler) {
            const { initCompiler } = await import("./compiler");
            const compiler = initCompiler(this.store);
            this.compiler = compiler;
        }
        return this.compiler;
    }

    /// Handle the result of opening a project
    ///
    /// This will show error message accordingly if the result is failure.
    /// On success, it will initialize the file system and the editor.
    ///
    /// This function eats the error because alerts will be shown to the user
    public async handleOpenFileSysResult(
        fileSysResult: FsResult<FileSys>,
    ): Promise<void> {
        console.info("opening file system...");
        const { FsResultCodes } = await import("low/fs");
        if (fileSysResult.isErr()) {
            const code = fileSysResult.inner();
            if (code === FsResultCodes.UserAbort) {
                console.info("opening file system aborted.");
                return;
            }
            if (code === FsResultCodes.NotSupported) {
                await this.getAlertMgr().show({
                    title: "Not Supported",
                    message: "Your browser does not support this feature.",
                    okButton: "Close",
                    learnMoreLink: "/docs/route/editor/web#browser-os-support",
                });
            } else if (code === FsResultCodes.IsFile) {
                await this.getAlertMgr().show({
                    title: "Error",
                    message:
                        "You dropped a file. Make sure you are dropping the project folder and not individual files.",
                    okButton: "Close",
                });
            } else {
                await this.getAlertMgr().show({
                    title: "Error",
                    message: `Cannot open the project. Make sure you have access to the folder or contact support. (Error code ${code}}`,
                    okButton: "Close",
                });
            }
            return;
        }
        console.info("initializing new file system...");
        const fileSys = fileSysResult.inner();
        let result = await fileSys.init();
        while (result.isErr()) {
            let retry = false;
            const code = result.inner();
            if (code === FsResultCodes.PermissionDenied) {
                retry = await this.getAlertMgr().show({
                    title: "Permission Denied",
                    message:
                        "You must given file system access permission to the app to use this feature. Please try again and grant the permission when prompted.",
                    okButton: "Grant Permission",
                    cancelButton: "Cancel",
                });
            } else {
                retry = await this.getAlertMgr().show({
                    title: "Error",
                    message: `Failed to initialize the project. Please try again. (Error code ${code})`,
                    okButton: "Retry",
                    cancelButton: "Cancel",
                });
            }
            if (!retry) {
                return;
            }
            result = await fileSys.init();
        }

        const { editorMode } = settingsSelector(this.store.getState());
        if (editorMode === "web") {
            // must be able to save to use web editor
            if (!fileSys.isWritable()) {
                const yes = await this.getAlertMgr().show({
                    title: "Save not supported",
                    message:
                        "The web editor cannot be used because your browser does not support saving changes to the file system. If you wish to edit the project, you can use the External Editor workflow and have Celer load changes directly from your file system.",
                    okButton: "Use external editor",
                    cancelButton: "Cancel",
                    learnMoreLink: "/docs/route/editor/web#browser-os-support",
                });
                if (!yes) {
                    return;
                }
                this.store.dispatch(settingsActions.setEditorMode("external"));
                // make sure store is updated for the next check
            }
        }

        if (fileSys.isStale()) {
            const yes = await this.getAlertMgr().show({
                title: "Heads up!",
                message:
                    "Your browser has limited support for file system access when opening a project from a dialog. Certain operations may not work! Please see the learn more link below for more information.",
                okButton: "Continue anyway",
                cancelButton: "Cancel",
                learnMoreLink: "/docs/route/editor/external#open-a-project",
            });
            if (!yes) {
                return;
            }
        }

        const { initEditor } = await import("./editor");
        const editor = await initEditor(this, fileSys, this.store);
        this.editor = editor;
        this.updateRootPathInStore(fileSys);
        const compiler = await this.getCompiler();
        await compiler.init(this.editor.getFileAccess());

        // trigger a first run when loading new project
        compiler.compile();
        console.info("project opened.");
    }

    public async compile() {
        const compiler = await this.getCompiler();
        compiler.compile();
    }

    public async closeFileSys() {
        console.info("closing file system...");
        this.store.dispatch(documentActions.setDocument(undefined));
        this.updateRootPathInStore(undefined);
        this.editor = null;
        const { deleteEditor } = await import("./editor");
        deleteEditor();
        const compiler = await this.getCompiler();
        compiler.uninit();
    }

    private updateRootPathInStore(fileSys: FileSys | undefined) {
        if (fileSys) {
            this.store.dispatch(
                viewActions.updateFileSys(fileSys.getRootName()),
            );
        } else {
            this.store.dispatch(viewActions.updateFileSys(undefined));
        }
    }

    public async export(request: ExportRequest): Promise<ExpoDoc> {
        // TODO #33: inject split types
        const stageMode = viewSelector(this.store.getState()).stageMode;
        if (stageMode === "edit") {
            const compiler = await this.getCompiler();
            return await compiler.export(request);
        } else {
            // TODO #184: export from server
            return {
                error: "Export from server is not available yet. This is tracked by issue 184 on GitHub",
            };
        }
    }
}
