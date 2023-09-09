import { FsPath } from "./FsPath";
import { FsResult, FsResultCode } from "./FsResult";

export interface FileSys {
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
    /// 
    /// This function cannot throw.
    listDir: (path: FsPath) => Promise<FsResult<string[]>>;

    /// Read the content of a file and last modified time
    ///
    /// Returns the content of the file as a string.
    ///
    /// Returns Fail if the underlying file system operation fails.
    /// Returns InvalidEncoding if the file is not valid UTF-8.
    /// 
    /// This function cannot throw.
    readFile: (path: FsPath) => Promise<FsResult<string>>;
    readFileAndModifiedTime: (path: FsPath) => Promise<FsResult<[string, number]>>;

    /// Read if the file has been modified since
    /// 
    /// Returns NotModified if not modified
    ///
    /// This function cannot throw.
    readIfModified: (path: FsPath, lastModified?: number) => Promise<FsResult<[string, number]>>;

    /// Returns if this implementation supports writing to a file
    isWritable: () => boolean;

    /// Write content to a file
    ///
    /// Writes the content to the path specified using UTF-8 encoding. Will overwrite existing file.
    /// Will not create new file.
    /// 
    /// Returns Fail if the underlying file system operation fails.
    /// Returns NotSupported if the browser does not support this
    writeFile: (path: FsPath, content: string) => Promise<FsResultCode>;

}
