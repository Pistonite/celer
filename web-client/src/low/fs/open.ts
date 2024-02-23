//! Utils for opening FileSys

import { ResultHandle } from "pure/result";

import { console } from "low/utils";

import {
    FileEntriesApiFileSys,
    isFileEntriesApiSupported,
} from "./FileEntriesApiFileSys";
import { FileSys } from "./FileSys";
import {
    FileSystemAccessApiFileSys,
    isFileSystemAccessApiSupported,
} from "./FileSystemAccessApiFileSys";
import { FsResult, FsResultCodes } from "./FsResult";
import { FileApiFileSys } from "./FileApiFileSys";

export async function showDirectoryPicker(r: ResultHandle): Promise<FsResult<FileSys>> {
    if (isFileSystemAccessApiSupported()) {
        try {
            // @ts-expect-error showDirectoryPicker is not in the TS lib
            const handle = await window.showDirectoryPicker({
                mode: "readwrite",
            });
            if (!handle) {
                console.error("Failed to get handle from showDirectoryPicker");
                return allocErr(FsResultCodes.Fail);
            }
            return createFsFromFileSystemHandle(handle);
        } catch (e) {
            console.error(e);
            // eslint-disable-next-line @typescript-eslint/no-explicit-any
            if (e && (e as any).name === "AbortError") {
                return allocErr(FsResultCodes.UserAbort);
            }
            return allocErr(FsResultCodes.Fail);
        }
    }
    // Fallback to File API
    const inputElement = document.createElement("input");
    inputElement.id = "temp";
    inputElement.style.display = "none";
    document.body.appendChild(inputElement);
    inputElement.type = "file";
    inputElement.webkitdirectory = true;
    return await new Promise((resolve) => {
        inputElement.addEventListener("change", (event) => {
            const files = (event.target as HTMLInputElement).files;
            if (!files) {
                resolve(allocErr(FsResultCodes.Fail));
                return;
            }
            resolve(createFsFromFileList(files));
        });
        inputElement.click();
    });
}

function 

export const createFsFromDataTransferItem = async (
    item: DataTransferItem,
): Promise<FsResult<FileSys>> => {
    // Prefer File System Access API since it supports writing
    if ("getAsFileSystemHandle" in item) {
        if (isFileSystemAccessApiSupported()) {
            try {
                // @ts-expect-error getAsFileSystemHandle is not in the TS lib
                const handle = await item.getAsFileSystemHandle();
                if (!handle) {
                    console.error("Failed to get handle from DataTransferItem");
                    return allocErr(FsResultCodes.Fail);
                }
                return createFsFromFileSystemHandle(handle);
            } catch (e) {
                console.error(e);
            }
        }
    }
    console.warn(
        "Failed to create FileSys with FileSystemAccessAPI. Trying FileEntriesAPI",
    );
    if ("webkitGetAsEntry" in item) {
        if (isFileEntriesApiSupported()) {
            try {
                const entry = item.webkitGetAsEntry();
                if (!entry) {
                    console.error("Failed to get entry from DataTransferItem");
                    return allocErr(FsResultCodes.Fail);
                }
                return createFsFromFileSystemEntry(entry);
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

const createFsFromFileSystemHandle = (
    handle: FileSystemHandle,
): FsResult<FileSys> => {
    if (handle.kind !== "directory") {
        return allocErr(FsResultCodes.IsFile);
    }

    const fs = new FileSystemAccessApiFileSys(
        handle.name,
        handle as FileSystemDirectoryHandle,
    );

    return allocOk(fs);
};

const createFsFromFileSystemEntry = (
    entry: FileSystemEntry,
): FsResult<FileSys> => {
    if (entry.isFile || !entry.isDirectory) {
        return allocErr(FsResultCodes.IsFile);
    }
    const fs = new FileEntriesApiFileSys(
        entry.name,
        entry as FileSystemDirectoryEntry,
    );
    return allocOk(fs);
};

const createFsFromFileList = (files: FileList): FsResult<FileSys> => {
    if (!files.length) {
        return allocErr(FsResultCodes.Fail);
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
    return allocOk(fs);
};
