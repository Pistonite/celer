import { AppStore } from "core/store";

import { CompilerKernel } from "./CompilerKernel";
import { CompilerKernelImpl } from "./CompilerKernelImpl";
import { CompilerLog } from "./utils";

declare global {
    interface Window {
        __theCompilerKernel: CompilerKernel;
    }
}

export const initCompiler = (store: AppStore): CompilerKernel => {
    if (window.__theCompilerKernel) {
        window.__theCompilerKernel.delete();
    }
    CompilerLog.info("creating compiler");
    const compiler = new CompilerKernelImpl(store);
    window.__theCompilerKernel = compiler;
    return compiler;
}
