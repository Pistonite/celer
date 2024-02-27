import { tryCatch, tryAsync } from "pure/result";
import { errstr } from "pure/utils";

import { FsFileSystem, FsFileSystemUninit } from "./FsFileSystem.ts";
import { FsErr, FsError, FsResult, fsErr, fsFail } from "./FsError.ts";
import { fsGetSupportStatus } from "./FsSupportStatus.ts";
import { FsImplFileAPI } from "./FsImplFileAPI.ts";
import { FsImplEntryAPI } from "./FsImplEntryAPI.ts";
import { FsImplHandleAPI } from "./FsImplHandleAPI.ts";

/// Handle for handling top level open errors, and decide if the operation should be retried
export type FsOpenRetryHandler = (
    error: FsError,
    attempt: number,
) => Promise<FsResult<boolean>>;

const MAX_RETRY = 10;

/// Open a file system for read-only access with a directory picker dialog
export async function fsOpenRead(
    retryHandler?: FsOpenRetryHandler,
): Promise<FsResult<FsFileSystem>> {
    const fs = await createWithPicker(false, retryHandler);
    if (fs.err) {
        return fs;
    }
    return await init(fs.val, retryHandler);
}

/// Open a file system for read-write access with a directory picker dialog
export async function fsOpenReadWrite(
    retryHandler?: FsOpenRetryHandler,
): Promise<FsResult<FsFileSystem>> {
    const fs = await createWithPicker(true, retryHandler);
    if (fs.err) {
        return fs;
    }
    return await init(fs.val, retryHandler);
}

/// Open a file system for read-only access from a DataTransferItem from a drag and drop event
export async function fsOpenReadFrom(
    item: DataTransferItem,
    retryHandler?: FsOpenRetryHandler,
): Promise<FsResult<FsFileSystem>> {
    const fs = await createFromDataTransferItem(item, false, retryHandler);
    if (fs.err) {
        return fs;
    }
    return await init(fs.val, retryHandler);
}

/// Open a file system for read-write access from a DataTransferItem from a drag and drop event
export async function fsOpenReadWriteFrom(
    item: DataTransferItem,
    retryHandler?: FsOpenRetryHandler,
): Promise<FsResult<FsFileSystem>> {
    const fs = await createFromDataTransferItem(item, true, retryHandler);
    if (fs.err) {
        return fs;
    }
    return await init(fs.val, retryHandler);
}

async function createWithPicker(
    write: boolean,
    retryHandler: FsOpenRetryHandler | undefined,
): Promise<FsResult<FsFileSystemUninit>> {
    for (let attempt = 1; attempt <= MAX_RETRY; attempt++) {
        const { implementation } = fsGetSupportStatus();
        if (implementation === "FileSystemAccess") {
            const handle = await tryAsync(() => showDirectoryPicker(write));
            if (handle.val) {
                return createFromFileSystemHandle(handle.val, write);
            }
            if (retryHandler) {
                // eslint-disable-next-line @typescript-eslint/no-explicit-any
                const isAbort =
                    handle.err && (handle.err as any).name === "AbortError";
                const error = isAbort
                    ? fsErr(FsErr.UserAbort, "User cancelled the operation")
                    : fsFail(errstr(handle.err));
                const shouldRetry = await retryHandler(error, attempt);
                if (shouldRetry.err) {
                    // retry handler failed
                    return shouldRetry;
                }
                if (!shouldRetry.val) {
                    // don't retry
                    return { err: error };
                }
            }
            // Retry with FileSystemAccess API
            continue;
        }

        // FileEntry API only supported through drag and drop
        // so fallback to File API
        const inputElement = document.createElement("input");
        inputElement.id = "temp";
        inputElement.style.display = "none";
        document.body.appendChild(inputElement);
        inputElement.type = "file";
        inputElement.webkitdirectory = true;

        const fsUninit = await new Promise<FsResult<FsFileSystemUninit>>(
            (resolve) => {
                inputElement.addEventListener("change", (event) => {
                    const files = (event.target as HTMLInputElement).files;
                    if (!files) {
                        const err = fsFail(
                            "Failed to get files from input element",
                        );
                        return resolve({ err });
                    }
                    resolve(createFromFileList(files));
                });
                inputElement.click();
            },
        );
        inputElement.remove();

        if (fsUninit.val) {
            return fsUninit;
        }

        if (retryHandler) {
            const shouldRetry = await retryHandler(fsUninit.err, attempt);
            if (shouldRetry.err) {
                // retry handler failed
                return shouldRetry;
            }
            if (!shouldRetry.val) {
                // don't retry
                return fsUninit;
            }
            // fall through to retry
        }
    }
    return { err: fsFail("Max retry count reached") };
}

async function createFromDataTransferItem(
    item: DataTransferItem,
    write: boolean,
    retryHandler: FsOpenRetryHandler | undefined,
): Promise<FsResult<FsFileSystemUninit>> {
    for (let attempt = 1; attempt <= MAX_RETRY; attempt++) {
        let error: FsError | undefined = undefined;
        const { implementation } = fsGetSupportStatus();
        // Prefer File System Access API since it supports writing
        if (
            "getAsFileSystemHandle" in item &&
            implementation === "FileSystemAccess"
        ) {
            const handle = await tryAsync(() => getAsFileSystemHandle(item));
            if (handle.val) {
                return createFromFileSystemHandle(handle.val, write);
            }
            error = fsFail(
                "Failed to get handle from DataTransferItem: " +
                    errstr(handle.err),
            );
        } else if (
            "webkitGetAsEntry" in item &&
            implementation === "FileEntry"
        ) {
            const entry = tryCatch(() => webkitGetAsEntry(item));
            if (entry.val) {
                return createFromFileSystemEntry(entry.val);
            }
            error = fsFail(
                "Failed to get entry from DataTransferItem: " +
                    errstr(entry.err),
            );
        }
        if (!error) {
            const err = fsErr(
                FsErr.NotSupported,
                "No supported API found on the DataTransferItem",
            );
            return { err };
        }
        // handle error
        if (retryHandler) {
            const shouldRetry = await retryHandler(error, attempt);
            if (shouldRetry.err) {
                // retry handler failed
                return shouldRetry;
            }
            if (!shouldRetry.val) {
                // don't retry
                return { err: error };
            }
            // fall through to retry
        }
    }
    return { err: fsFail("Max retry count reached") };
}

async function init(
    fs: FsFileSystemUninit,
    retryHandler: FsOpenRetryHandler | undefined,
): Promise<FsResult<FsFileSystem>> {
    let attempt = -1;
    while (true) {
        attempt++;
        const inited = await fs.init();
        if (!inited.err) {
            return inited;
        }
        if (!retryHandler) {
            return inited;
        }
        const shouldRetry = await retryHandler(inited.err, attempt);
        if (shouldRetry.err) {
            // retry handler failed
            return shouldRetry;
        }
        if (!shouldRetry.val) {
            // should not retry
            return inited;
        }
    }
}

/// Wrapper for window.showDirectoryPicker
function showDirectoryPicker(write: boolean): Promise<FileSystemHandle> {
    // @ts-expect-error showDirectoryPicker is not in the TS lib
    return window.showDirectoryPicker({ mode: write ? "readwrite" : "read" });
}

/// Wrapper for DataTransferItem.getAsFileSystemHandle
async function getAsFileSystemHandle(
    item: DataTransferItem,
): Promise<FileSystemHandle> {
    // @ts-expect-error getAsFileSystemHandle is not in the TS lib
    const handle = await item.getAsFileSystemHandle();
    if (!handle) {
        throw new Error("handle is null");
    }
    return handle;
}

/// Wrapper for DataTransferItem.webkitGetAsEntry
function webkitGetAsEntry(item: DataTransferItem): FileSystemEntry {
    const entry = item.webkitGetAsEntry();
    if (!entry) {
        throw new Error("entry is null");
    }
    return entry;
}

function createFromFileSystemHandle(
    handle: FileSystemHandle,
    write: boolean,
): FsResult<FsFileSystemUninit> {
    if (handle.kind !== "directory") {
        const err = fsErr(FsErr.IsFile, "Expected directory");
        return { err };
    }

    const fs = new FsImplHandleAPI(
        handle.name,
        handle as FileSystemDirectoryHandle,
        write,
    );

    return { val: fs };
}

function createFromFileSystemEntry(
    entry: FileSystemEntry,
): FsResult<FsFileSystemUninit> {
    if (entry.isFile || !entry.isDirectory) {
        const err = fsErr(FsErr.IsFile, "Expected directory");
        return { err };
    }
    const fs = new FsImplEntryAPI(
        entry.name,
        entry as FileSystemDirectoryEntry,
    );
    return { val: fs };
}

function createFromFileList(files: FileList): FsResult<FsFileSystemUninit> {
    if (!files.length) {
        const err = fsFail("Expected at least one file");
        return { err };
    }
    return { val: new FsImplFileAPI(files) };
}
