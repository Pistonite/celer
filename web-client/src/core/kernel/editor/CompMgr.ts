import { AppDispatcher, documentActions, viewActions } from "core/store";
import { init, compile_document } from "low/celerc";
import { Debouncer, Logger, wrapAsync, console } from "low/utils";

const CompilerLog = new Logger("com");

export type RequestFileFunction = (path: string) => Promise<Uint8Array>;

/// The compilation manager
///
/// Responsible for triggering compilation of the document.
/// This ensures compilation is done for the latest version of the document.
/// and no 2 compilations are running at the same time.
export class CompMgr {
    private compilerDebouncer: Debouncer;
    private dispatcher: AppDispatcher;

    private needCompile: boolean;
    private compiling: boolean;

    constructor(dispatcher: AppDispatcher) {
        this.dispatcher = dispatcher;
        this.compilerDebouncer = new Debouncer(100, this.compile.bind(this));
        this.needCompile = false;
        this.compiling = false;
    }

    public async init(
        loadFile: RequestFileFunction,
        loadUrl: RequestFileFunction,
    ) {
        init(
            window.location.origin,
            (x: string) => CompilerLog.info(x),
            (x: string) => CompilerLog.warn(x),
            (x: string) => CompilerLog.error(x),
            loadFile,
            loadUrl,
        );
    }

    /// Trigger compilation of the document
    ///
    /// This will batch multiple compiler calls. There will be guaranteed to be at least one call to the compiler
    /// after this function is called.
    ///
    /// After compilation is done, the document will automatically be updated
    public triggerCompile() {
        this.needCompile = true;
        this.compilerDebouncer.dispatch();
    }

    private async compile() {
        // check if another compilation is running
        // this is safe because there's no await between checking and setting (no other code can run)
        if (this.compiling) {
            CompilerLog.warn("compilation already in progress, skipping");
            return;
        }
        const handle = window.setTimeout(() => {
            this.dispatcher.dispatch(viewActions.setCompileInProgress(true));
        }, 200);
        this.compiling = true;
        while (this.needCompile) {
            // turn off the flag before compiling.
            // if anyone calls triggerCompile during compilation, it will be turned on again
            // to trigger another compile
            this.needCompile = false;
            CompilerLog.info("invoking compiler...");
            const result = await wrapAsync(compile_document);
            if (result.isErr()) {
                console.error(result.inner());
            } else {
                const doc = result.inner();
                if (doc !== undefined) {
                    this.dispatcher.dispatch(documentActions.setDocument(doc));
                }
            }
        }
        CompilerLog.info("finished compiling");

        window.clearTimeout(handle);
        this.dispatcher.dispatch(viewActions.setCompileInProgress(false));
        this.compiling = false;
    }
}
