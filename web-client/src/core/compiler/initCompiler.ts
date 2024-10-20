import type { AppStore } from "core/store";
import { consoleCompiler as console } from "low/utils";

import type { CompilerKernel } from "./CompilerKernel";
import { CompilerKernelImpl } from "./CompilerKernelImpl";

declare global {
    interface Window {
        __theCompilerKernel: CompilerKernelImpl;
    }
}

export const initCompiler = (store: AppStore): CompilerKernel => {
    if (window.__theCompilerKernel) {
        window.__theCompilerKernel.delete();
    }
    console.info("creating compiler");
    const compiler = new CompilerKernelImpl(store);
    window.__theCompilerKernel = compiler;
    return compiler;
};
