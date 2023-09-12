
import { AppDispatcher } from "core/store";
import { Debouncer } from "low/utils";

export type RequestFileFunction = (path: string) => Promise<Uint8Array>;

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

    private async compile() {
        // check if another compilation is running
        // this is safe because there's no await between checking and setting (no other code can run)
        if (this.compiling) {
            return;
        }
        this.compiling = true;
        // turn off the flag before compiling.
        // if anyone calls triggerCompile during compilation, it will be turned on again
        // to trigger another compile
        this.needCompile = false;

        //wasm api should be something like:
        //compile(requestfunction) -> Promise<result>
    }

}
