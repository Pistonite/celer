import reduxWatch from "redux-watch";

import { FsFileSystem } from "pure/fs";

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
    documentSelector,
} from "core/store";
import {
    getDefaultSplitTypes,
    getSplitExportPluginConfigs,
    isRecompileNeeded,
    loadDocumentFromCurrentUrl,
} from "core/doc";
import type { CompilerKernel } from "core/compiler";
import type { EditorKernel, EditorKernelAccess } from "core/editor";
import { ExpoDoc, ExportRequest } from "low/celerc";
import {
    consoleKernel as console,
    isInDarkMode,
    sleep,
    AlertMgr,
} from "low/utils";

import { KeyMgr } from "./KeyMgr";
import { WindowMgr } from "./WindowMgr";
import { AlertMgrImpl } from "./AlertMgr";

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
export class Kernel implements EditorKernelAccess {
    /// The store
    ///
    /// The kernel owns the store. The store is shared
    /// between app boots (i.e. when switching routes)
    private store: AppStore;
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
        this.initReact = initReact;
        console.info("starting application");
        this.store = this.initStore();
        this.alertMgr = new AlertMgrImpl(this.store);
    }

    /// Initialize the store
    private initStore(): AppStore {
        console.info("initializing store...");
        const store = initStore();

        const watchSettings = reduxWatch(() =>
            settingsSelector(store.getState()),
        );

        store.subscribe(
            watchSettings((newVal: SettingsState, _oldVal: SettingsState) => {
                // save settings to local storage
                console.info("saving settings...");
                saveSettings(newVal);
            }),
        );

        const watchAll = reduxWatch(() => store.getState());
        store.subscribe(
            watchAll(async (newVal: AppState, oldVal: AppState) => {
                if (await isRecompileNeeded(newVal, oldVal)) {
                    console.info("reloading document due to state change...");
                    await this.reloadDocument();
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
        console.info("initializing stage...");
        const path = window.location.pathname;
        if (path === "/edit") {
            document.title = "Celer Editor";
            this.store.dispatch(viewActions.setStageMode("edit"));
        } else {
            setTimeout(() => {
                this.reloadDocument();
            }, 0);
            this.store.dispatch(viewActions.setStageMode("view"));
        }
    }

    /// Initialize UI related stuff
    private initUi() {
        console.info("initializing ui...");
        if (this.cleanupUi) {
            console.info("unmounting previous ui");
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
            console.error(
                "compiler is not available in view mode. This is a bug!",
            );
            throw new Error("compiler is not available in view mode");
        }
        if (!this.compiler) {
            const { initCompiler } = await import("core/compiler");
            const compiler = initCompiler(this.store);
            this.compiler = compiler;
        }
        return this.compiler;
    }

    /// Open a project file system
    ///
    /// This function eats the error because alerts will be shown to the user
    public async openProjectFileSystem(fs: FsFileSystem): Promise<void> {
        console.info("opening file system...");

        const { editorMode } = settingsSelector(this.store.getState());
        const { write, live } = fs.capabilities;
        if (editorMode === "web") {
            // must be able to save to use web editor
            if (!write) {
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
            }
        }

        if (!live) {
            const yes = await this.getAlertMgr().show({
                title: "Heads up!",
                message:
                    "Your browser has limited support for file system access when opening a project from a dialog. Celer will not be able to detect new, renamed or deleted files! Please see the learn more link below for more information.",
                okButton: "Continue anyway",
                cancelButton: "Cancel",
                learnMoreLink: "/docs/route/editor/external#open-a-project",
            });
            if (!yes) {
                return;
            }
        }

        const { initEditor } = await import("core/editor");
        const editor = await initEditor(this, fs, this.store);
        this.editor = editor;
        this.updateRootPathInStore(fs);
        const compiler = await this.getCompiler();
        await compiler.init(editor.getFileAccess());

        // trigger a first run when loading new project
        compiler.compile();
        console.info("project opened.");
    }

    public async closeProjectFileSystem() {
        console.info("closing file system...");
        this.store.dispatch(documentActions.setDocument(undefined));
        this.updateRootPathInStore(undefined);
        this.editor = null;
        const { deleteEditor } = await import("core/editor");
        deleteEditor();
        const compiler = await this.getCompiler();
        compiler.uninit();
    }

    private updateRootPathInStore(fs: FsFileSystem | undefined) {
        this.store.dispatch(viewActions.updateFileSys(fs?.root ?? undefined));
    }

    public async reloadDocument() {
        if (viewSelector(this.store.getState()).stageMode === "edit") {
            const compiler = await this.getCompiler();
            await compiler.compile();
            return;
        }
        await this.reloadDocumentFromServer();
    }

    public async export(request: ExportRequest): Promise<ExpoDoc> {
        const splitExportConfigs = getSplitExportPluginConfigs();
        if (splitExportConfigs.find((c) => c.use === request.pluginId)) {
            if (request.payload && typeof request.payload === "object") {
                const payload = request.payload as Record<string, unknown>;
                if (!payload["split-types"]) {
                    const { splitTypes } = settingsSelector(
                        this.store.getState(),
                    );
                    let injected: string[];
                    if (splitTypes) {
                        injected = splitTypes;
                    } else {
                        const { document } = documentSelector(
                            this.store.getState(),
                        );
                        if (document) {
                            injected = getDefaultSplitTypes(document);
                        } else {
                            injected = [];
                        }
                    }
                    payload["split-types"] = injected;
                    console.info(
                        `injected ${injected.length} split types into export request payload.`,
                    );
                }
            }
        }
        const { stageMode } = viewSelector(this.store.getState());
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

    /// Reload the document from the server based on the current URL
    private async reloadDocumentFromServer() {
        this.store.dispatch(documentActions.setDocument(undefined));
        // let UI update
        await sleep(0);
        // show progress spinner if reload takes longer than 200ms
        const handle = setTimeout(() => {
            this.store.dispatch(viewActions.setCompileInProgress(true));
        }, 200);

        let retry = true;
        while (retry) {
            console.info("reloading document from server");
            const result = await loadDocumentFromCurrentUrl();
            if (result.type === "failure") {
                this.store.dispatch(documentActions.setDocument(undefined));
                console.info("failed to load document from server");
                console.error(result.data);
                retry = await this.getAlertMgr().show({
                    title: "Failed to load route",
                    message: result.data,
                    learnMoreLink: result.help,
                    okButton: "Retry",
                    cancelButton: "Cancel",
                });
                if (!retry) {
                    await this.alertMgr.show({
                        title: "Load cancelled",
                        message:
                            'You can retry at any time by refreshing the page, or by clicking "Reload Document" from the toolbar.',
                        okButton: "Got it",
                        cancelButton: "",
                    });
                    break;
                }
                console.warn("retrying in 1s...");
                await sleep(1000);
                continue;
            }
            console.info("received document from server");
            const doc = result.data;
            try {
                const { title, version } = doc.execDoc.project;
                if (!title) {
                    document.title = "Celer Viewer";
                } else if (!version) {
                    document.title = title;
                } else {
                    document.title = `${title} - ${version}`;
                }
            } catch (e) {
                console.warn("failed to set document title");
                console.error(e);
                document.title = "Celer Viewer";
            }
            this.store.dispatch(documentActions.setDocument(doc));
            break;
        }
        clearTimeout(handle);
        this.store.dispatch(viewActions.setCompileInProgress(false));
    }
}
