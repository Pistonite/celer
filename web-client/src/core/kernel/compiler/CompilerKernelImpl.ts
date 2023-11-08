import reduxWatch from "redux-watch";

import {
    AppStore,
    SettingsState,
    documentActions,
    settingsActions,
    settingsSelector,
    viewActions,
    viewSelector,
} from "core/store";
import {
    EntryPointsSorted,
    compile_document,
    get_entry_points,
} from "low/celerc";
import {
    Debouncer,
    wrapAsync,
    setWorker,
    registerWorkerHandler,
    allocOk,
    Result,
    sleep,
} from "low/utils";
import { FileAccess, FsResultCodes } from "low/fs";

import { CompilerKernel } from "./CompilerKernel";
import { CompilerLog } from "./utils";

async function checkFileExists(fileAccess: FileAccess, path: string): Promise<boolean> {
    const result = await fileAccess.getFileContent(path, true);
    if (result.isOk()) {
        return true;
    }
    if (result.inner() === FsResultCodes.NotModified) {
        return true;
    }
    return false;
}

/// The compilation kernel
///
/// Owns the compiler worker and handles compilation requests.
/// It uses FileAccess interface to send files to the worker.
export class CompilerKernelImpl implements CompilerKernel {
    private store: AppStore;
    private fileAccess: FileAccess | undefined = undefined;

    private compilerDebouncer: Debouncer;
    private needCompile: boolean;
    private compiling: boolean;

    private validatedEntryPath: string | undefined = undefined;

    private cleanup: () => void;

    constructor(store: AppStore) {
        this.store = store;
        this.compilerDebouncer = new Debouncer(
            100,
            this.compileInternal.bind(this),
        );
        this.needCompile = false;
        this.compiling = false;

        const watchSettings = reduxWatch(() =>
            settingsSelector(store.getState()),
        );
        const unwatchSettings = store.subscribe(
            watchSettings((newVal, oldVal) => {
                this.onSettingsUpdate(newVal, oldVal);
            }),
        );

        this.cleanup = () => {
            unwatchSettings();
        };
    }

    public delete() {
        CompilerLog.info("deleting compiler");
        this.uninit();
        this.cleanup();
    }

    public uninit() {
        CompilerLog.info("uninitializing compiler...");
        this.fileAccess = undefined;
        this.store.dispatch(viewActions.setCompilerReady(false));
    }

    public async init(fileAccess: FileAccess) {
        this.store.dispatch(viewActions.setCompilerReady(false));
        CompilerLog.info("initializing compiler worker...");
        this.fileAccess = fileAccess;
        const worker = new Worker("/celerc/worker.js");
        registerWorkerHandler(
            "load_file",
            async ([path, checkChanged]: [string, boolean]) => {
                if (!this.fileAccess) {
                    worker.postMessage([
                        "file",
                        1,
                        path,
                        "file access not available",
                    ]);
                    return;
                }
                const result = await this.fileAccess.getFileContent(
                    path,
                    checkChanged,
                );
                if (result.isOk()) {
                    worker.postMessage([
                        "file",
                        0,
                        path,
                        [true, result.inner()],
                    ]);
                } else {
                    const err = result.inner();
                    if (err === FsResultCodes.NotModified) {
                        worker.postMessage(["file", 0, path, [false]]);
                    } else {
                        worker.postMessage(["file", 1, path, err]);
                    }
                }
            },
        );

        await setWorker(worker, CompilerLog);
        this.store.dispatch(viewActions.setCompilerReady(true));
    }

    public async getEntryPoints(): Promise<Result<EntryPointsSorted, unknown>> {
        await this.ensureReady();
        if (!this.fileAccess) {
            return allocOk([]);
        }
        return await wrapAsync(get_entry_points);
    }

    /// Trigger compilation of the document
    ///
    /// This will batch multiple compiler calls. There will be guaranteed to be at least one call to the compiler
    /// after this function is called.
    ///
    /// After compilation is done, the document will automatically be updated
    public async compile() {
        if (!this.fileAccess) {
            CompilerLog.warn("file access not available, skipping compile");
            return;
        }
        await this.ensureReady();
        // check if entry path is a valid file
        const { compilerEntryPath } = settingsSelector(this.store.getState());
        if (compilerEntryPath) {
            const filePath = compilerEntryPath.startsWith("/")
                ? compilerEntryPath.substring(1)
                : compilerEntryPath;
            if (!(await checkFileExists(this.fileAccess, filePath))) {
                CompilerLog.warn(
                    "entry path is invalid, attempting correction...",
                );
            }

            const newEntryPath = await this.correctEntryPath(compilerEntryPath);
            if (newEntryPath !== compilerEntryPath) {
                // update asynchronously to avoid infinite blocking loop
                // updating the entry path will trigger another compile
                await sleep(0);
                this.store.dispatch(
                    settingsActions.setCompilerEntryPath(newEntryPath),
                );
                return;
            }
        }

        // if entryPath is empty string, change it to undefined
        this.validatedEntryPath = compilerEntryPath || undefined;
        this.needCompile = true;
        this.compilerDebouncer.dispatch();
    }

    private async ensureReady() {
        while (!viewSelector(this.store.getState()).compilerReady) {
            CompilerLog.info("worker not ready, waiting...");
            await sleep(500);
        }
    }

    private async compileInternal() {
        // check if another compilation is running
        // this is safe because there's no await between checking and setting (no other code can run)
        if (this.compiling) {
            CompilerLog.warn("compilation already in progress, skipping");
            return;
        }
        const handle = window.setTimeout(() => {
            this.store.dispatch(viewActions.setCompileInProgress(true));
        }, 200);
        this.compiling = true;
        while (this.needCompile) {
            // turn off the flag before compiling.
            // if anyone calls triggerCompile during compilation, it will be turned on again
            // to trigger another compile
            this.needCompile = false;
            CompilerLog.info("invoking compiler...");
            const { compilerUseCachePack0 } = settingsSelector(
                this.store.getState(),
            );
            const result = await wrapAsync(() => {
                return compile_document(
                    this.validatedEntryPath,
                    compilerUseCachePack0,
                );
            });
            if (result.isErr()) {
                CompilerLog.error(result.inner());
            } else {
                const doc = result.inner();
                if (doc !== undefined) {
                    this.store.dispatch(documentActions.setDocument(doc));
                }
            }
        }
        CompilerLog.info("finished compiling");

        window.clearTimeout(handle);
        this.store.dispatch(viewActions.setCompileInProgress(false));
        this.compiling = false;
    }

    /// Try to correct an invalid entry path
    ///
    /// The invalid entry path may be saved from a previous project.
    /// The function will try to find a valid entry path from the current project.
    /// However, if the same entry path is found in the current project, that will be returned
    private async correctEntryPath(entryPath: string): Promise<string> {
        const entryPointsResult = await this.getEntryPoints();
        if (entryPointsResult.isErr()) {
            return "";
        }
        const newEntryPoints = entryPointsResult.inner();
        if (newEntryPoints.length === 0) {
            return "";
        }
        // if entry point with the same path exists, don't correct it
        // i.e. the entry point config is intended, but the file is missing
        for (const [_, path] of newEntryPoints) {
            if (path === entryPath) {
                return path;
            }
        }
        // if entry point with "default" name exists, try that first
        for (const [name, path] of newEntryPoints) {
            if (name === "default" && path) {
                if (!this.fileAccess) {
                    return "";
                }
                const filePath = path.startsWith("/")
                    ? path.substring(1)
                    : path;
                if (await checkFileExists(this.fileAccess, filePath)) {
                    return path;
                }
                break;
            }
        }
        // otherwise find the first valid entry point
        for (const [_, path] of newEntryPoints) {
            if (path) {
                if (!this.fileAccess) {
                    return "";
                }
                const filePath = path.startsWith("/")
                    ? path.substring(1)
                    : path;
                if (await checkFileExists(this.fileAccess, filePath)) {
                    return path;
                }
            }
        }
        return "";
    }

    private onSettingsUpdate(oldVal: SettingsState, newVal: SettingsState) {
        if (this.fileAccess) {
            if (oldVal.compilerEntryPath !== newVal.compilerEntryPath) {
                CompilerLog.info("entry path changed, triggering compile");
                this.compile();
            }
        }
    }
}

