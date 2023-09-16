
import { AppDispatcher, viewActions } from "core/store";
import { sleep } from "low/utils";
import { initCompiler } from "low/celerc";

export type RequestFileFunction = (path: string) => Promise<Uint8Array>;
export type CheckChangedFunction = (path: string) => boolean;

/// The compilation manager
///
/// Responsible for triggering compilation of the document.
/// This ensures compilation is done for the latest version of the document.
/// and no 2 compilations are running at the same time.
export class CompMgr {
    private dispatcher: AppDispatcher;

    private needCompile: boolean;
    private compiling: boolean;

    constructor(dispatcher: AppDispatcher) {
        this.dispatcher = dispatcher;
        this.needCompile = false;
        this.compiling = false;

    }

    public async init(loadFile: RequestFileFunction, checkChanged: CheckChangedFunction) {
        initCompiler(loadFile, checkChanged);
    }

    /// Trigger compilation of the document
    ///
    /// This will batch multiple compiler calls. There will be guaranteed to be at least one call to the compiler
    /// after this function is called.
    ///
    /// After compilation is done, the document will automatically be updated
    public triggerCompile() {
        if (this.needCompile) {
            return;
        }
        this.needCompile = true;
        this.compile();
    }

    /// Cancel the current compilation if it is running (do nothing if not)
    public cancel() {
        // wasm api cancel
        // wasm.wasmCall(cancelCompile)
        console.log("test compile cancel");
    }

    private async compile() {
        // check if another compilation is running
        // this is safe because there's no await between checking and setting (no other code can run)
        if (this.compiling) {
            return;
        }
        console.log("test compile function start");
        const handle = window.setTimeout(() => {
            this.dispatcher.dispatch(viewActions.setCompileInProgress(true));
        }, 200);
        this.compiling = true;
        while (this.needCompile) {
            console.log("test compile start");
            // turn off the flag before compiling.
            // if anyone calls triggerCompile during compilation, it will be turned on again
            // to trigger another compile
            this.needCompile = false;
            await sleep(5000);
            console.log("test compile end");
        }

        window.clearTimeout(handle);
        this.dispatcher.dispatch(viewActions.setCompileInProgress(false));
        this.compiling = false;
        console.log("function end");
        //wasm api should be something like:
        //compile(requestfunction) -> Promise<result>
    }

}
