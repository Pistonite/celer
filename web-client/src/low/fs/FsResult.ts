
/// Result type for file system operations
export const FsResultCode = {
    /// Operation succeeded
    Ok: 0,
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
} as const;

export type FsOkResult<T> = {
    code: typeof FsResultCode["Ok"];
    value: T;
};

export type FsResult<T> = FsOkResult<T> | {
    code: typeof FsResultCode[keyof Omit<typeof FsResultCode, "Ok">];
};

