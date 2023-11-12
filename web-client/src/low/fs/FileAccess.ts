import { FsResult } from "./FsResult";

/// Interface for the compiler to access files
export interface FileAccess {
    /// Get the content of a file
    ///
    /// If checkChanged is true, the implementation may check if the file
    /// pointed to by the path was changed since the last time getFileContent was called.
    /// If it was not changed, the implementation could return NotModified as the error code.
    getFileContent(
        path: string,
        checkChanged: boolean,
    ): Promise<FsResult<Uint8Array>>;
}
