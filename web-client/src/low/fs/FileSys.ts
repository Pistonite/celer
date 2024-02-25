// import { ResultHandle } from "pure/result";
//
// import { FsPath } from "./FsPath";
// import { FsResult } from "./FsResult";
//
// /// Interface for using the browser's various file system API to access Files
// export interface FileSys {
//     /// Async init function
//     ///
//     /// The FileSys implementation may need to do some async initialization.
//     /// For example, request permission from the user.
//     init: (r: ResultHandle) => Promise<FsResult<void>>;
//
//     /// Get the root path of the file system for display
//     ///
//     /// The returned string has no significance in the file system itself.
//     /// It should only be used as an indicator to the user.
//     getRootName: () => string;
//
//     /// List files in a directory
//     ///
//     /// Returns a list of file names in the directory (not full paths).
//     /// Directory names end with a slash.
//     ///
//     /// Returns Fail if the underlying file system operation fails.
//     listDir: (r: ResultHandle, path: FsPath) => Promise<FsResult<string[]>>;
//
//     /// Read the file as a File object
//     ///
//     /// Returns Fail if the underlying file system operation fails.
//     readFile: (r: ResultHandle, path: FsPath) => Promise<FsResult<File>>;
//
//     /// Returns if this implementation supports writing to a file
//     isWritable: () => boolean;
//
//     /// Returns if this implementation only keeps a static snapshot of the directory structure
//     isStale: () => boolean;
//
//     /// Write content to a file
//     ///
//     /// Writes the content to the path specified.
//     /// If the content is a string, UTF-8 encoding is used.
//     ///
//     /// Will overwrite existing file.
//     ///
//     /// Returns Fail if the underlying file system operation fails.
//     /// Returns NotSupported if the browser does not support this
//     writeFile: (
//         r: ResultHandle,
//         path: FsPath,
//         content: string | Uint8Array,
//     ) => Promise<FsResult<void>>;
// }
