import { Result } from "low/utils";

/// Result type for file system operations
export const FsResultCodes = {
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
} as const;

export type FsResultCode = (typeof FsResultCodes)[keyof typeof FsResultCodes];

export type FsResult<T> = Result<T, FsResultCode>;
