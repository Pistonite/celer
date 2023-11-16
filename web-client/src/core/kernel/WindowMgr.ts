import { AppStore, viewActions } from "core/store";

/// Manager for various global window events
export class WindowMgr {
    private store: AppStore;

    private resizeHandle: number | undefined = undefined;

    constructor(store: AppStore) {
        this.store = store;
    }

    /// Register the window handlers
    ///
    /// Returns a function to unregister
    public listen(): () => void {
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
        window.addEventListener("resize", onResize);
        return () => {
            window.removeEventListener("resize", onResize);
        };
    }
}
