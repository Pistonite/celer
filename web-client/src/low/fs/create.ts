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

export const showDirectoryPicker = async (): Promise<FsResult<FileSys>> => {
    if (isFileSystemAccessAPISupported()) {
            try {
                // @ts-expect-error showDirectoryPicker is not in the TS lib
                const handle = await window.showDirectoryPicker({ mode: "readwrite" });
                if (!handle) {
                    console.error("Failed to get handle from showDirectoryPicker");
                    return allocErr(FsResultCodes.Fail);
                }
                return await createFsFromFileSystemHandle(handle);
            } catch (e) {
                console.error(e);
                // eslint-disable-next-line @typescript-eslint/no-explicit-any
                if ((e as any).name === "AbortError") {
                    return allocErr(FsResultCodes.UserAbort);
                }
                return allocErr(FsResultCodes.Fail);

            }
    }
    // Fallback to input type = file
    const inputElement = document.createElement("input");
    inputElement.id = "temp";
    inputElement.style.display = "none";
    document.body.appendChild(inputElement);
    inputElement.type = "file";
    inputElement.multiple = true;
    // inputElement.webkitdirectory = true;
    return await new Promise((resolve) => {
        // function cancel() {
        //     window.console.log(inputElement.files);
        //     window.console.log(inputElement.webkitEntries.length);
        //     document.body.onfocus = null;
        //     // resolve(
        //     //     allocErr(FsResultCodes.UserAbort));
        // }
        // document.body.onfocus = cancel;
        inputElement.addEventListener("change", (event) => {
            window.console.log((event.target as HTMLInputElement).webkitEntries);
            resolve(
                allocErr(FsResultCodes.NotSupported));
        });
        inputElement.click();
        console.info("clicked");
    });
}

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
