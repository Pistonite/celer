import reduxWatch from "redux-watch";

import {
    AppStore,
    SettingsState,
    documentActions,
    initStore,
    settingsSelector,
} from "core/store";
import { Logger } from "low/utils";

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
    // private store: AppStore | null = null;
    /// The store cleanup function
    private cleanupStore: (() => void) | null = null;
    /// The link tag that loads the theme css
    private themeLinkTag: HTMLLinkElement | null = null;
    /// The function to initialize react
    private initReact: InitUiFunction;
    /// The function to unmount react
    private cleanupUi: (() => void) | null = null;

    constructor(initReact: InitUiFunction) {
        this.log = new Logger("ker");
        this.initReact = initReact;
    }

    /// Start the application. Cleans up previous application if needed
    public init() {
        this.log.info("starting application");
        const store = this.initStore();
        this.initUi(store);

        this.test(store);
    }

    private async test(store: AppStore) {
        const wasm = await import("low/celerc");
        const testFn = async (test: any) => {
            const result = await wasm.tryCompileFromCompDoc(test);
            store.dispatch(
                documentActions.setDocument(result)
            );
        }
        (window as any).testFn = testFn;
        console.log("window api ready");
    }

    /// Initialize the store
    private initStore(): AppStore {
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
                console.log({
                    message: "settings changed",
                    new: newVal,
                    old: oldVal,
                });

                // switch theme
                if (newVal.theme !== oldVal.theme) {
                    this.switchTheme(newVal.theme);
                }
            }),
        );

        // this.store = store;
        this.cleanupStore = () => {
            unwatchSettings();
        };
        return store;
    }

    /// Initialize UI related stuff
    private initUi(store: AppStore) {
        if (this.cleanupUi) {
            this.log.info("unmounting previous ui");
            this.cleanupUi();
        }
        const isDarkMode = !!(
            window.matchMedia &&
            window.matchMedia("(prefers-color-scheme: dark)").matches
        );
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
}
