import { FsResult, FsVoid } from "./FsError.ts";

/// Internal APIs for FsFileSystem
export interface FsFileSystemInternal {
    /// Read the file as a File object
    ///
    /// Returns Fail if the underlying file system operation fails.
    read: (path: string) => Promise<FsResult<File>>;

    /// Write content to a file on disk
    ///
    /// Writes the content to the path specified.
    /// If the content is a string, UTF-8 encoding is used.
    ///
    /// Will overwrite existing file.
    ///
    /// Returns Fail if the underlying file system operation fails.
    /// Returns NotSupported if the browser does not support this
    /// Returns PermissionDenied if the operation is supported, but permission is not given
    write: (path: string, content: Uint8Array) => Promise<FsVoid>;

    /// Forget about a file
    closeFile: (path: string) => void;
}
