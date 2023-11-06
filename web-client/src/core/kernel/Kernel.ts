import reduxWatch from "redux-watch";

import {
    AppStore,
    SettingsState,
    initStore,
    saveSettings,
    settingsSelector,
    viewActions,
} from "core/store";
import { console, Logger, isInDarkMode, sleep } from "low/utils";
import type { FileAccess, FileSys } from "low/fs";

import type { CompilerKernel } from "./compiler";
import type { EditorKernel } from "./editor";
import { KeyMgr } from "./KeyMgr";

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
    private store: AppStore | null = null;
    /// The store cleanup function
    private cleanupStore: (() => void) | null = null;
    /// The link tag that loads the theme css
    private themeLinkTag: HTMLLinkElement | null = null;
    /// The function to initialize react
    private initReact: InitUiFunction;
    /// The function to unmount react
    private cleanupUi: (() => void) | null = null;

    // Alert API
    private alertCallback: ((ok: boolean) => void) | undefined = undefined;

    // Editor API
    // The editor is owned by the kernel because the toolbar needs access
    private editor: EditorKernel | null = null;

    // Compiler API
    private compiler: CompilerKernel | null = null;

    constructor(initReact: InitUiFunction) {
        this.log = new Logger("ker");
        this.initReact = initReact;
    }

    /// Start the application. Cleans up previous application if needed
    public init() {
        this.log.info("starting application");
        let store = this.store;
        if (!store) {
            store = this.initStore();
            this.store = store;
        }

        this.initStage(store);
        this.initUi(store);

        window.addEventListener("beforeunload", (e) => {
            if (this.editor && this.editor.hasUnsavedChangesSync()) {
                e.preventDefault();
                return (e.returnValue =
                    "There are unsaved changes in the editor which will be lost. Are you sure you want to leave?");
            }
        });
    }

    /// Initialize stage info based on window.location
    private async initStage(store: AppStore) {
        this.log.info("initializing stage...");
        const path = window.location.pathname;
        if (path === "/edit") {
            const { initCompiler } = await import("./compiler");
            const compiler = initCompiler(store);
            this.compiler = compiler;

            store.dispatch(viewActions.setStageMode("edit"));
        } else {
            store.dispatch(viewActions.setStageMode("view"));
        }
    }

    /// Initialize the store
    private initStore(): AppStore {
        this.log.info("initializing store...");
        if (this.cleanupStore) {
            this.log.info("cleaning up previous store");
            this.cleanupStore();
        }
        const store = initStore();
        // switch theme base on store settings
        this.switchTheme(settingsSelector(store.getState()).theme);

        const watchSettings = reduxWatch(() =>
            settingsSelector(store.getState()),
        );
        // persist settings to local storage TODO
        const unwatchSettings = store.subscribe(
            watchSettings((newVal: SettingsState, oldVal: SettingsState) => {
                // save settings to local storage
                saveSettings(newVal);

                // switch theme
                if (newVal.theme !== oldVal.theme) {
                    this.switchTheme(newVal.theme);
                }
            }),
        );

        this.cleanupStore = () => {
            unwatchSettings();
        };
        return store;
    }

    /// Initialize UI related stuff
    private initUi(store: AppStore) {
        this.log.info("initializing ui...");
        if (this.cleanupUi) {
            this.log.info("unmounting previous ui");
            this.cleanupUi();
        }
        const isDarkMode = isInDarkMode();
        const unmountReact = this.initReact(this, store, isDarkMode);

        // key binding handler
        const keyMgr = new KeyMgr(store);
        const unlistenKeyMgr = keyMgr.listen();

        this.cleanupUi = () => {
            unmountReact();
            unlistenKeyMgr();
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

    /// Show an alert dialog
    ///
    /// Returns a promise that resolves to true if the user
    /// clicked ok and false if the user clicked cancel.
    public showAlert(
        title: string,
        message: string,
        okButton: string,
        cancelButton: string,
    ): Promise<boolean> {
        return new Promise((resolve) => {
            this.alertCallback = (ok) => {
                this.store?.dispatch(viewActions.clearAlert());
                resolve(ok);
                this.alertCallback = undefined;
            };
            const store = this.store;
            if (!store) {
                console.error("store not initialized");
                resolve(false);
                return;
            }
            store.dispatch(
                viewActions.setAlert({
                    title,
                    text: message,
                    okButton,
                    cancelButton,
                }),
            );
        });
    }

    public onAlertAction(ok: boolean) {
        if (this.alertCallback) {
            this.alertCallback(ok);
        }
    }

    public getEditor(): EditorKernel | null {
        return this.editor;
    }

    public getCompiler(): CompilerKernel | null {
        return this.compiler;
    }

    public async setFileSys(fs: FileSys) {
        // TODO #122: only need to init editor if not using external editor
        const webEditor = true;
        if (webEditor) {
            if (!this.editor) {
                if (!this.store) {
                    throw new Error("store not initialized");
                }
                const { initEditor } = await import("./editor");
                const editor = initEditor(this.store);
                editor.init(() => {
                    this.compiler?.compile();
                });
                this.editor = editor;
            }
            this.editor.reset(fs);
            this.initCompilerWithRetry(this.editor.getFileAccess());
        } else {
            // TODO #122: bind FileSys directly to compiler
        }
    }
    async initCompilerWithRetry(fileAccess: FileAccess) {
        const MAX_TRIES = 10;
        for (let i = 0; i < MAX_TRIES; i++) {
            if (this.compiler) {
                await this.compiler.init(fileAccess);
                return;
            }
            console.warn("compiler not ready, retrying...");
            await sleep(500);
        }
        console.error("compiler not ready after max retries");
    }
}
