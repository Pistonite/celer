//! Utils for creating FileSys

import { FileEntiresAPIFileSys, isFileEntriesAPISupported } from "./FileEntriesAPIFileSys";
import { FileSys } from "./FileSys";
import { FileSystemAccessAPIFileSys, isFileSystemAccessAPISupported } from "./FileSystemAccessApiFileSys";
import { FsResult, FsResultCode } from "./FsResult";

export const createFsFromDataTransferItem = async (item: DataTransferItem): Promise<FsResult<FileSys>> => {
    // Prefer File System Access API since it supports writing
    if ("getAsFileSystemHandle" in item) {
    if (isFileSystemAccessAPISupported()) {
        try {
            // @ts-expect-error getAsFileSystemHandle is not in the TS lib
            const handle = await item.getAsFileSystemHandle();
            if (!handle) {
                return {
                    code: FsResultCode.Fail,
                };
            }
            const result = await createFsFromFileSystemHandle(handle);
            if (result.code !== FsResultCode.NotSupported) {
                return result;
            }
        } catch (e) {
            console.error(e);
        }
    }
    }
    console.warn("Failed to create FileSys with FileSystemAccessAPI. Trying FileEntriesAPI");
    if (!isFileEntriesAPISupported()) {
        return {
            code: FsResultCode.NotSupported,
        };
    }
    if ("webkitGetAsEntry" in item) {
        try {
            const entry = item.webkitGetAsEntry();
            if (!entry) {
                console.error("Failed to get entry from DataTransferItem");
                return {
                    code: FsResultCode.Fail,
                };
            }
            const result = await createFsFromFileSystemEntry(entry);
            if (result.code !== FsResultCode.NotSupported) {
                return result;
            }
        } catch (e) {
            console.error(e);
        }
    }
    console.warn("Failed to create FileSys with FileEntriesAPI. Editor is not supported");
    return {
        code: FsResultCode.NotSupported,
    };
}

const createFsFromFileSystemHandle = async (handle: FileSystemHandle): Promise<FsResult<FileSys>> => {
    if (handle.kind !== "directory") {
        return {
            code: FsResultCode.IsFile,
        };
    }

    const rootName = handle.name;
    const fs = new FileSystemAccessAPIFileSys(rootName, handle as FileSystemDirectoryHandle);
    return {
        code: FsResultCode.Ok,
        value: fs,
    };
}

const createFsFromFileSystemEntry = async (entry: FileSystemEntry): Promise<FsResult<FileSys>> => {
    if (entry.isFile || !entry.isDirectory) {
        return {
            code: FsResultCode.IsFile,
        };
    }
    const rootName = entry.name;
    const fs = new FileEntiresAPIFileSys(rootName, entry as FileSystemDirectoryEntry);
    return {
        code: FsResultCode.Ok,
        value: fs,
    };
}
