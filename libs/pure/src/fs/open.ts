import { ResultHandle } from "../result";
import { errstr } from "../utils";
import { FsFileSystem, FsFileSystemUninit } from "./FsFileSystem";

import { FsErr, FsError, FsResult, fsErr, fsFail } from "./error";
import { FsImplFsa } from "./impl";
import { fsGetSupportStatus } from "./support";

/// Handle for handling top level open errors, and decide if the operation should be retried
export type FsOpenRetryHandler = (r: ResultHandle, error: FsError, attempt: number) => Promise<FsResult<boolean>>;

/// Open a file system for read-only access with a directory picker dialog
export async function fsOpenRead(r: ResultHandle, retryHandler?: FsOpenRetryHandler): Promise<FsResult<FsFileSystem>> {
    r.put(await createWithPicker(r, false, retryHandler));
    if (r.isErr()) {
        return r.ret();
    }
    return await init(r, r.value, retryHandler);
}

/// Open a file system for read-write access with a directory picker dialog
export async function fsOpenReadWrite(r: ResultHandle, retryHandler?: FsOpenRetryHandler): Promise<FsResult<FsFileSystem>> {
    r.put(await createWithPicker(r, true, retryHandler));
    if (r.isErr()) {
        return r.ret();
    }
    return await init(r, r.value, retryHandler);
}

/// Open a file system for read-only access from a DataTransferItem from a drag and drop event
export async function fsOpenReadFrom(
    r: ResultHandle, item: DataTransferItem, retryHandler?: FsOpenRetryHandler
): Promise<FsResult<FsFileSystem>> {
    r.put(await createFromDataTransferItem(r, item, false, retryHandler));
    if (r.isErr()) {
        return r.ret();
    }
    return await init(r, r.value, retryHandler);
}

/// Open a file system for read-write access from a DataTransferItem from a drag and drop event
export async function fsOpenReadWriteFrom(
    r: ResultHandle, item: DataTransferItem, retryHandler?: FsOpenRetryHandler
): Promise<FsResult<FsFileSystem>> {
    r.put(await createFromDataTransferItem(r, item, true, retryHandler));
    if (r.isErr()) {
        return r.ret();
    }
    return await init(r, r.value, retryHandler);
}

async function createWithPicker(
    r: ResultHandle, write: boolean, retryHandler: FsOpenRetryHandler | undefined
): Promise<FsResult<FsFileSystemUninit>> {
    let attempt = -1;
    
    while (true) {
        attempt++;
        const { implementation } = fsGetSupportStatus();
        if (implementation === "FileSystemAccess") {
            r.put(await r.tryCatchAsync(r, () => showDirectoryPicker(write)));
            if (r.isErr()) {
                // eslint-disable-next-line @typescript-eslint/no-explicit-any
                const isAbort = r.error && (r.error as any).name === "AbortError";
                const error = isAbort 
                    ? fsErr(FsErr.UserAbort, "User cancelled the operation")
                    : fsFail(errstr(r.error));
                if (retryHandler) {
                    r.put(await retryHandler(r = r.erase(), error, attempt));
                    if (r.isErr()) {
                        // retry handler failed
                        return r.ret();
                    };
                    if (r.value) {
                        // should retry
                        continue;
                    }
                }
                // don't retry
                return r.putErr(error);
            }
            const handle = r.value;
            return createFromFileSystemHandle(r, handle, write);
        }
        // FileEntry API only supported through drag and drop, so fallback to File API
        const inputElement = document.createElement("input");
        inputElement.id = "temp";
        inputElement.style.display = "none";
        document.body.appendChild(inputElement);
        inputElement.type = "file";
        inputElement.webkitdirectory = true;

        r.put(await new Promise<FsResult<FsFileSystemUninit>>((resolve) => {
            inputElement.addEventListener("change", (event) => {
                const files = (event.target as HTMLInputElement).files;
                if (!files) {
                    resolve(r.putErr(fsFail("Failed to get files from input element")));
                    return;
                }
                resolve(createFromFileList(r, files));
            });
            inputElement.click();
        }));
        inputElement.remove();
        
        if (r.isErr()) {
            const error = r.error;
            if (retryHandler) {
                r.put(await retryHandler(r = r.erase(), error, attempt));
                if (r.isErr()) {
                    // retry handler failed
                    return r.ret();
                };
                if (r.value) {
                    // should retry
                    continue;
                }
            }
            // don't retry
            return r.putErr(error);
        }
        return r;
    }
}

async function createFromDataTransferItem(
    r: ResultHandle, 
    item: DataTransferItem, 
    write: boolean, 
    retryHandler: FsOpenRetryHandler | undefined
): Promise<FsResult<FsFileSystemUninit>> {
    let attempt = -1;
    while (true) {
        attempt++;
        const { implementation } = fsGetSupportStatus();
        // Prefer File System Access API since it supports writing
        if ("getAsFileSystemHandle" in item && implementation === "FileSystemAccess") {
            r.put(r.tryCatch(r, () => getAsFileSystemHandle(item)));
            if (r.isOk()) {
                const handle = r.value;
                return createFromFileSystemHandle(r, handle, write);
            }
            // handle error
            if (retryHandler) {
                const error = fsFail("Failed to get handle from DataTransferItem");
                r.put(await retryHandler(r = r.erase(), error, attempt));
                if (r.isErr()) {
                    // retry handler failed
                    return r.ret();
                };
                if (r.value) {
                    // should retry
                    continue;
                }
                // don't retry
                return r.putErr(error);
            }
            // fall through
        }

        // if FileSystemAccess doesn't work, try FileEntry
        
        if ("webkitGetAsEntry" in item && implementation === "FileEntry") {
            r.put(r.tryCatch(r, () => webkitGetAsEntry(item)));
            if (r.isOk()) {
                const entry = r.value;
                return createFromFileSystemEntry(r, entry);
            }
            // handle error
            if (retryHandler) {
                const error = fsFail("Failed to get entry from DataTransferItem");
                r.put(await retryHandler(r = r.erase(), error, attempt));
                if (r.isErr()) {
                    // retry handler failed
                    return r.ret();
                };
                if (r.value) {
                    // should retry
                    continue;
                }
                // don't retry
                return r.putErr(error);
            }
        }
        break;
    }

    return r.putErr(fsErr(FsErr.NotSupported, "File system is not supported in the current environment"));
}

async function init(
    r: ResultHandle, fs: FsFileSystemUninit, retryHandler: FsOpenRetryHandler | undefined
): Promise<FsResult<FsFileSystem>> {
    let attempt = -1;
    while(true) {
        attempt++;
        r.put(await fs.init(r));
        if (r.isOk()) {
            return r;
        }
        if (!retryHandler) {
            return r;
        }
        const error = r.error;
        r = r.erase();
        r.put(await retryHandler(r, error, attempt));
        if (r.isErr()) {
            // retry handler failed
            return r.ret();
        }
        if (!r.value) {
            // should not retry
            return r.putErr(error);
        }
    }
}

/// Wrapper for window.showDirectoryPicker
function showDirectoryPicker(write: boolean): Promise<FileSystemHandle> {
    // @ts-expect-error showDirectoryPicker is not in the TS lib
    return window.showDirectoryPicker({ mode: write ? "readwrite" : "read" });
}

/// Wrapper for DataTransferItem.getAsFileSystemHandle
function getAsFileSystemHandle(item: DataTransferItem): FileSystemHandle {
    // @ts-expect-error getAsFileSystemHandle is not in the TS lib
    const handle = item.getAsFileSystemHandle();
    if (!handle) {
        throw new Error("Failed to get handle from DataTransferItem");
    }
    return handle;
}

/// Wrapper for DataTransferItem.webkitGetAsEntry
function webkitGetAsEntry(item: DataTransferItem): FileSystemEntry {
    const entry = item.webkitGetAsEntry();
    if (!entry) {
        throw new Error("Failed to get entry from DataTransferItem");
    }
    return entry;
}

function createFromFileSystemHandle(
    r: ResultHandle, handle: FileSystemHandle, write: boolean
): FsResult<FsFileSystemUninit> {
    if (handle.kind !== "directory") {
        return r.putErr(fsErr(FsErr.IsFile, "Expected directory"));
    }

    const fs = new FsImplFsa(
        handle.name,
        handle as FileSystemDirectoryHandle,
        write
    );

    return r.putOk(fs);
};

function createFromFileSystemEntry(
    r: ResultHandle, entry: FileSystemEntry,
): FsResult<FsFileSystemUninit> {
    if (entry.isFile || !entry.isDirectory) {
        return r.putErr(fsErr(FsErr.IsFile, "Expected directory"));
    }
    const fs = new FileEntriesApiFileSys(
        entry.name,
        entry as FileSystemDirectoryEntry,
    );
    return r.putOk(fs);
}

function createFromFileList(
    r: ResultHandle, files: FileList
): FsResult<FsFileSystemUninit> {
    if (!files.length) {
        return r.putErr(fsFail("Expected at least one file"));
    }
    const rootName = files[0].webkitRelativePath.split("/", 1)[0];
    const fileMap: Record<string, File> = {};
    for (let i = 0; i < files.length; i++) {
        const file = files[i];
        // remove "<root>/"
        const path = file.webkitRelativePath.slice(rootName.length + 1);
        fileMap[path] = file;
    }
    const fs = new FileApiFileSys(rootName, fileMap);
    return r.putOk(fs);
};
