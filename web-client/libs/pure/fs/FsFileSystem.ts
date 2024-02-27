import { FsFile } from "./FsFile.ts";
import { FsResult } from "./FsError.ts";

/// File system before it is initialized
///
/// This is an internal type used inside fsOpen functions
export interface FsFileSystemUninit {
    /// Initialize the file system
    init(): Promise<FsResult<FsFileSystem>>;
}

/// Initialized file system
export interface FsFileSystem {
    /// Get the root path of the file system for display
    ///
    /// The returned string has no significance in the file system itself.
    /// It should only be used as an indicator to the user.
    readonly root: string;

    /// Capabilities of this file system implementation
    /// See README.md for more information
    readonly capabilities: FsCapabilities;

    /// List files in a directory
    ///
    /// The input path should be relative to the root (of the uploaded directory).
    ///
    /// Returns a list of file names in the directory (not full paths).
    /// Directory names end with a slash.
    ///
    /// Returns Fail if the underlying file system operation fails.
    listDir: (path: string) => Promise<FsResult<string[]>>;

    /// Get a file object for operations
    ///
    /// The returned object can store temporary state for the file, such
    /// as newer content. Calling openFile with the same path will
    /// return the same object.
    ///
    /// Note that opening a file doesn't actually block the file
    /// from being modified by programs other than the browser.
    ///
    /// You can make the FsFileSystem forget about the file by
    /// calling `close` on the file object.
    getFile: (path: string) => FsFile;

    /// Get all paths that `getFile` has been called with but not `close`d
    getOpenedPaths: () => string[];
}

/// Capabilities of the file system implementation
export type FsCapabilities = {
    /// Can the browser directly write to the file system
    write: boolean;
    /// Can the browser detect live updates:
    /// - Change of modified time
    /// - Change of directory structure (new, renamed, deleted files)
    live: boolean;
};
