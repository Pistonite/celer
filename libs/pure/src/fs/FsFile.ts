import { ResultHandle } from "../result";
import { FsResult } from "./error";

/// Interface for operating on a file in the loaded file system
export interface FsFile {
    /// Path of the file relative to the root of the file system (the uploaded directory)
    readonly path: string;

    /// Returns if the content of the file in memory is newer than the file on disk
    isDirty(): boolean;

    /// Get the last modified time. May load it from file system if needed
    getLastModified(r: ResultHandle): Promise<FsResult<number>>;

    /// Get the text content of the file
    ///
    /// If the file is not loaded, it will load it.
    ///
    /// If the file is not a text file, it will return InvalidEncoding
    getText(r: ResultHandle): Promise<FsResult<string>>;

    /// Get the content of the file
    getBytes(r: ResultHandle): Promise<FsResult<Uint8Array>>

    /// Set the content in memory. Does not save to disk.
    /// Does nothing if file is closed
    setText(content: string): void;

    /// Set the content in memory. Does not save to disk.
    /// Does nothing if file is closed
    setBytes(content: Uint8Array): void;

    /// Load the file's content if it's not newer than fs
    ///
    /// Returns Ok if the file is newer than fs
    loadIfNotDirty(r: ResultHandle): Promise<FsResult<void>>;

    /// Load the file's content from FS.
    ///
    /// Overwrites any unsaved changes in memory only if the file was modified
    /// at a later time than the last in memory modification.
    ///
    /// If it fails, the file's content in memory will not be changed
    load(r: ResultHandle): Promise<FsResult<void>>;

    /// Save the file's content to FS if it is dirty.
    ///
    /// If not dirty, returns Ok
    writeIfNewer(r: ResultHandle): Promise<FsResult<void>>;

    /// Close the file. In memory content will be lost.
    /// Further operations on the file will fail
    close(): void;

}

