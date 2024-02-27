import { Result, Void } from "pure/result";

/// Result type for file system operations
export const FsErr = {
    /// Generic error
    Fail: 1,
    /// The operation does not apply to the root directory
    IsRoot: 2,
    /// Invalid encoding
    InvalidEncoding: 3,
    /// Not supported
    NotSupported: 4,
    /// The operation does not apply to a file
    IsFile: 5,
    /// The file was not modified since the last check
    NotModified: 6,
    /// Permission error
    PermissionDenied: 7,
    /// User abort
    UserAbort: 8,
    /// Not found
    NotFound: 9,
    /// Trying to do stuff to a closed file
    IsClosed: 10,
    /// If the path is invalid, for example trying to get the parent of root
    InvalidPath: 11,
    /// Trying to operate on a file that has been closed
    Closed: 12,
    /// The operation does not apply to a directory
    IsDirectory: 5,
} as const;

export type FsErr = (typeof FsErr)[keyof typeof FsErr];
export type FsError = {
    readonly code: FsErr;
    readonly message: string;
};

export function fsErr(code: FsErr, message: string): FsError {
    return { code, message };
}

export function fsFail(message: string): FsError {
    return fsErr(FsErr.Fail, message);
}

export type FsResult<T> = Result<T, FsError>;
export type FsVoid = Void<FsError>;
