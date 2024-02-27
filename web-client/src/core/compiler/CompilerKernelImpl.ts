import { FsErr, FsError } from "pure/fs";
import { Result, tryAsync } from "pure/result";
import { errstr } from "pure/utils";

import { getRawPluginOptions } from "core/doc";
import {
    AppStore,
    documentActions,
    settingsActions,
    settingsSelector,
    viewActions,
    viewSelector,
} from "core/store";
import {
    EntryPointsSorted,
    ExpoDoc,
    ExportRequest,
    PluginOptionsRaw,
    compile_document,
    export_document,
    get_entry_points,
    set_plugin_options,
} from "low/celerc";
import {
    setWorker,
    registerWorkerHandler,
    sleep,
    ReentrantLock,
    consoleCompiler as console,
} from "low/utils";

import { CompilerKernel } from "./CompilerKernel";
import { CompilerFileAccess } from "./CompilerFileAccess";

async function checkFileExists(
    fileAccess: CompilerFileAccess,
    path: string,
): Promise<boolean> {
    const content = await fileAccess.getFileContent(path, true);
    if (content.val) {
        return true;
    }
    if (content.err.code === FsErr.NotModified) {
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
    private fileAccess: CompilerFileAccess | undefined = undefined;

    private needCompile: boolean;
    /// Flag used to prevent multiple compilation to run at the same time
    private compiling: boolean;
    /// Lock to prevent compilation and other operations from running at the same time
    private compilerLock: ReentrantLock;
    private lastPluginOptions: PluginOptionsRaw | undefined;

    private cleanup: () => void;
    private waiters: Array<(x: unknown) => void>;

    constructor(store: AppStore) {
        this.store = store;
        this.needCompile = false;
        this.compiling = false;
        this.compilerLock = new ReentrantLock("compiler");
        this.waiters = [];

        this.cleanup = () => {
            // no cleanup needed for now
        };
    }

    public delete() {
        console.info("deleting compiler");
        this.uninit();
        this.cleanup();
    }

    public uninit() {
        console.info("uninitializing compiler...");
        this.fileAccess = undefined;
        this.store.dispatch(viewActions.setCompilerReady(false));
        this.store.dispatch(viewActions.setCompileInProgress(false));
        this.compiling = false;
    }

    public async init(fileAccess: CompilerFileAccess) {
        this.store.dispatch(viewActions.setCompilerReady(false));
        console.info("initializing compiler worker...");
        this.fileAccess = fileAccess;
        this.lastPluginOptions = undefined;
        const worker = new Worker("/celerc/worker.js");
        registerWorkerHandler(
            "load_file",
            async ([path, checkChanged]: [string, boolean]) => {
                if (!this.fileAccess) {
                    worker.postMessage([
                        "file",
                        1,
                        path,
                        {
                            code: FsErr.Fail,
                            message: "file access not available",
                        } satisfies FsError,
                    ]);
                    return;
                }
                const bytes = await this.fileAccess.getFileContent(
                    path,
                    checkChanged,
                );
                if (bytes.err) {
                    if (bytes.err.code === FsErr.NotModified) {
                        worker.postMessage(["file", 0, path, [false]]);
                    } else {
                        worker.postMessage(["file", 1, path, bytes.err]);
                    }
                    return;
                }
                worker.postMessage(["file", 0, path, [true, bytes.val]]);
            },
        );

        await setWorker(worker, console);
        this.store.dispatch(viewActions.setCompilerReady(true));
    }

    public async getEntryPoints(): Promise<Result<EntryPointsSorted, unknown>> {
        if (!(await this.ensureReady())) {
            console.error("worker not ready after max waiting");
            return { val: [] };
        }
        if (!this.fileAccess) {
            return { val: [] };
        }
        return await tryAsync(get_entry_points);
    }

    /// Trigger compilation of the document
    ///
    /// This will batch multiple compiler calls. There will be guaranteed to be at least one call to the compiler
    /// after this function is called.
    ///
    /// After compilation is done, the document will automatically be updated
    public async compile() {
        // setting the needCompile flag to ensure this request is handled eventually
        this.needCompile = true;
        if (!this.fileAccess) {
            console.warn("file access not available, skipping compile");
            return;
        }

        if (!(await this.ensureReady())) {
            console.warn(
                "worker not ready after max waiting, skipping compile",
            );
            return;
        }

        const validatedEntryPath = await this.validateEntryPath();
        if (validatedEntryPath.err) {
            console.warn("entry path is invalid, skipping compile");
            return;
        }

        this.store.dispatch(viewActions.setCompileInProgress(true));

        // lock the compiler so other operations can't run
        await this.compilerLock.lockedScope(undefined, async () => {
            // wait to let the UI update first
            await sleep(0);
            // check if another compilation is running
            // this is safe because there's no await between checking and setting (no other code can run)
            if (this.compiling) {
                return await new Promise((resolve) => {
                    if (!this.compiling) {
                        resolve(undefined);
                    }
                    console.warn("compilation already in progress, skipping");
                    this.waiters.push(resolve);
                });
            }
            this.compiling = true;
            while (this.needCompile) {
                // turn off the flag before compiling.
                // if anyone calls triggerCompile during compilation, it will be turned on again
                // to trigger another compile
                this.needCompile = false;
                const state = this.store.getState();
                const { compilerUseCachedPrepPhase } = settingsSelector(state);

                await this.updatePluginOptions();

                console.info("invoking compiler...");
                const result = await tryAsync(() => {
                    return compile_document(
                        validatedEntryPath.val,
                        compilerUseCachedPrepPhase,
                    );
                });
                // yielding just in case other things need to update
                await sleep(0);
                if ("err" in result) {
                    console.error(result.err);
                } else {
                    const doc = result.val;
                    if (this.fileAccess && doc !== undefined) {
                        this.store.dispatch(documentActions.setDocument(doc));
                    }
                }
            }
            this.store.dispatch(viewActions.setCompileInProgress(false));
            this.compiling = false;
            console.info("finished compiling");
        });
        const waiters = this.waiters;
        this.waiters = [];
        waiters.forEach((resolve) => resolve(undefined));
    }

    public async export(request: ExportRequest): Promise<ExpoDoc> {
        if (!this.fileAccess) {
            return {
                error: "Compiler not available. Please make sure a project is loaded.",
            };
        }

        if (!(await this.ensureReady())) {
            return {
                error: "Compiler is not ready. Please try again later.",
            };
        }

        const validatedEntryPath = await this.validateEntryPath();
        if ("err" in validatedEntryPath) {
            return {
                error: "Compiler entry path is invalid. Please check your settings.",
            };
        }

        return await this.compilerLock.lockedScope(undefined, async () => {
            const { compilerUseCachedPrepPhase } = settingsSelector(
                this.store.getState(),
            );

            await this.updatePluginOptions();

            const result = await tryAsync(() => {
                return export_document(
                    validatedEntryPath.val,
                    compilerUseCachedPrepPhase,
                    request,
                );
            });

            if ("err" in result) {
                console.error(result.err);
                return { error: errstr(result.err) };
            }
            return result.val;
        });
    }

    /// Try to wait for the compiler to be ready. Returns true if it becomes ready eventually.
    ///
    /// A timeout of 1 minute is implemented to prevent infinite wait.
    private async ensureReady(): Promise<boolean> {
        const INTERVAL = 500;
        const MAX_WAIT = 60000;
        let acc = 0;
        while (acc < MAX_WAIT) {
            if (viewSelector(this.store.getState()).compilerReady) {
                return true;
            }
            console.info("worker not ready, waiting...");
            await sleep(INTERVAL);
            acc += INTERVAL;
        }
        return false;
    }

    /// Validate the entry path
    ///
    /// Returns OK with the entry path if it is valid (or empty). Otherwise,
    /// attempts to fix the entry path and returns Err to skip the compilation
    private async validateEntryPath(): Promise<
        Result<string | undefined, undefined>
    > {
        if (!this.fileAccess) {
            return { err: undefined };
        }
        // check if entry path is a valid file
        const { compilerEntryPath } = settingsSelector(this.store.getState());
        if (compilerEntryPath) {
            const filePath = compilerEntryPath.startsWith("/")
                ? compilerEntryPath.substring(1)
                : compilerEntryPath;
            if (!(await checkFileExists(this.fileAccess, filePath))) {
                console.warn("entry path is invalid, attempting correction...");
            }

            const newEntryPath = await this.correctEntryPath(compilerEntryPath);
            if (newEntryPath !== compilerEntryPath) {
                // update asynchronously to avoid infinite blocking loop
                // updating the entry path will trigger another compile
                await sleep(0);
                console.info(`set entry path to ${newEntryPath}`);
                this.store.dispatch(
                    settingsActions.setCompilerEntryPath(newEntryPath),
                );
                return { err: undefined };
            }
        }

        // if entryPath is empty string, change it to undefined
        const validatedEntryPath = compilerEntryPath || undefined;
        return { val: validatedEntryPath };
    }

    /// Try to correct an invalid entry path
    ///
    /// The invalid entry path may be saved from a previous project.
    /// The function will try to find a valid entry path from the current project.
    /// However, if the same entry path is found in the current project, that will be returned
    private async correctEntryPath(entryPath: string): Promise<string> {
        const entryPoints = await this.getEntryPoints();
        if ("err" in entryPoints) {
            return "";
        }
        const newEntryPoints = entryPoints.val;
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

    private async updatePluginOptions() {
        const pluginOptions = getRawPluginOptions(this.store.getState());
        if (pluginOptions !== this.lastPluginOptions) {
            this.lastPluginOptions = pluginOptions;
            console.info("updating plugin options...");
            const result = await tryAsync(() =>
                set_plugin_options(pluginOptions),
            );
            if ("err" in result) {
                console.error(result.err);
                console.warn(
                    "failed to set plugin options. The output may be wrong.",
                );
            } else {
                console.info("plugin options updated");
            }
        }
    }
}
