import { consoleKernel as console } from "low/utils";

import { UiMgrInitFn } from "./UiMgr";
import { Kernel } from "./Kernel";

declare global {
    interface Window {
        __theKernel: Kernel;
    }
}

export async function initKernel(initUi: UiMgrInitFn): Promise<void> {
    if (window.__theKernel) {
        console.warn("deleting old kernel...");
        window.__theKernel.delete();
    }
    console.info("initializing kernel...");
    let kernel;
    if (window.location.pathname === "/edit") {
        const { KernelEditImpl } = await import("./KernelEditImpl");
        kernel = new KernelEditImpl(initUi);
    } else {
        const { KernelViewImpl } = await import("./KernelViewImpl");
        kernel = new KernelViewImpl(initUi);
    }

    window.__theKernel = kernel;
    kernel.init();
    console.info("kernel initialized");
}
