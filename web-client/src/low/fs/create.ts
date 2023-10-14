//! Utils for creating FileSys

import { console, allocErr, allocOk } from "low/utils";

import {
    FileEntriesAPIFileSys,
    isFileEntriesAPISupported,
} from "./FileEntriesAPIFileSys";
import { FileSys } from "./FileSys";
import {
    FileSystemAccessAPIFileSys,
    isFileSystemAccessAPISupported,
} from "./FileSystemAccessApiFileSys";
import { FsResult, FsResultCodes } from "./FsResult";

export const createFsFromDataTransferItem = async (
    item: DataTransferItem,
): Promise<FsResult<FileSys>> => {
    // Prefer File System Access API since it supports writing
    if ("getAsFileSystemHandle" in item) {
        if (isFileSystemAccessAPISupported()) {
            try {
                // @ts-expect-error getAsFileSystemHandle is not in the TS lib
                const handle = await item.getAsFileSystemHandle();
                if (!handle) {
                    console.error("Failed to get handle from DataTransferItem");
                    return allocErr(FsResultCodes.Fail);
                }
                return await createFsFromFileSystemHandle(handle);
            } catch (e) {
                console.error(e);
            }
        }
    }
    console.warn(
        "Failed to create FileSys with FileSystemAccessAPI. Trying FileEntriesAPI",
    );
    if ("webkitGetAsEntry" in item) {
        if (isFileEntriesAPISupported()) {
            try {
                const entry = item.webkitGetAsEntry();
                if (!entry) {
                    console.error("Failed to get entry from DataTransferItem");
                    return allocErr(FsResultCodes.Fail);
                }
                return await createFsFromFileSystemEntry(entry);
            } catch (e) {
                console.error(e);
            }
        }
    }
    console.warn(
        "Failed to create FileSys with FileEntriesAPI. Editor is not supported",
    );
    return allocErr(FsResultCodes.NotSupported);
};

const createFsFromFileSystemHandle = async (
    handle: FileSystemHandle,
): Promise<FsResult<FileSys>> => {
    if (handle.kind !== "directory") {
        return allocErr(FsResultCodes.IsFile);
    }

    const fs = new FileSystemAccessAPIFileSys(
        handle.name,
        handle as FileSystemDirectoryHandle,
    );

    return allocOk(fs);
};

const createFsFromFileSystemEntry = async (
    entry: FileSystemEntry,
): Promise<FsResult<FileSys>> => {
    if (entry.isFile || !entry.isDirectory) {
        return allocErr(FsResultCodes.IsFile);
    }
    const fs = new FileEntriesAPIFileSys(
        entry.name,
        entry as FileSystemDirectoryEntry,
    );
    return allocOk(fs);
};
