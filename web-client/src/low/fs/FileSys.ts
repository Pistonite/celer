import { FsPath } from "./FsPath";
import { FsResult } from "./FsResult";

/// Interface for using the browser's various file system API to access Files
export interface FileSys {
    /// Async init function
    ///
    /// The FileSys implementation may need to do some async initialization.
    /// For example, request permission from the user.
    init: () => Promise<FsResult<void>>;

    /// Get the root path of the file system for display
    ///
    /// The returned string has no significance in the file system itself.
    /// It should only be used as an indicator to the user.
    getRootName: () => string;

    /// List files in a directory
    ///
    /// Returns a list of file names in the directory (not full paths).
    /// Directory names end with a slash.
    ///
    /// Returns Fail if the underlying file system operation fails.
    listDir: (path: FsPath) => Promise<FsResult<string[]>>;

    /// Read the file as a File object
    ///
    /// Returns Fail if the underlying file system operation fails.
    readFile: (path: FsPath) => Promise<FsResult<File>>;

    // /// Read if the file has been modified since
    // ///
    // /// Returns NotModified if not modified
    // readIfModified: (
    //     path: FsPath,
    //     lastModified?: number,
    // ) => Promise<FsResult<[string, number]>>;

    /// Read file as raw bytes
    // readFileAsBytes: (path: FsPath) => Promise<FsResult<Uint8Array>>;

    /// Returns if this implementation supports writing to a file
    isWritable: () => boolean;

    /// Write content to a file
    ///
    /// Writes the content to the path specified.
    /// If the content is a string, UTF-8 encoding is used.
    ///
    /// Will overwrite existing file.
    ///
    /// Returns Fail if the underlying file system operation fails.
    /// Returns NotSupported if the browser does not support this
    writeFile: (path: FsPath, content: string) => Promise<FsResult<void>>;
}
