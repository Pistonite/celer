import { AppStore, viewActions } from "core/store";
import { consoleKernel as console } from "low/utils";

import { Kernel } from "./Kernel";

export type UiMgrInitFn = (
    kernel: Kernel,
    store: AppStore
) => () => void;

/// Manager for React and various global window events
export class UiMgr {
    private kernel: Kernel;
    private store: AppStore;

    private initFn: UiMgrInitFn;
    private cleanupFn: (() => void) | undefined = undefined;

    private resizeHandle: number | undefined = undefined;

    constructor(kernel: Kernel, store: AppStore, initFn: UiMgrInitFn) {
        this.kernel = kernel;
        this.store = store;
        this.initFn = initFn;
    }

    /// Register the window handlers
    ///
    /// Returns a function to unregister
    public init() {
        console.info("initializing ui...");
        const onResize = () => {
            if (this.resizeHandle) {
                // already resizing
                window.clearTimeout(this.resizeHandle);
            } else {
                this.store.dispatch(viewActions.setIsResizingWindow(true));
            }
            this.resizeHandle = window.setTimeout(() => {
                this.resizeHandle = undefined;
                this.store.dispatch(viewActions.setIsResizingWindow(false));
            }, 200);
        };
        const cleanUpUi = this.initFn(this.kernel, this.store);

        window.addEventListener("resize", onResize);
        this.cleanupFn = () => {
            cleanUpUi();
            window.removeEventListener("resize", onResize);
        };
    }

    public delete() {
        console.info("deleting ui...");
        this.cleanupFn?.();
        window.clearTimeout(this.resizeHandle);
    }
}
