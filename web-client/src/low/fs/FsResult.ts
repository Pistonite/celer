
/// Result type for file system operations
export const FsResultCodes = {
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
    /// The file was not modified since the last check
    NotModified: 6,
} as const;

export type FsResultCode = typeof FsResultCodes[keyof typeof FsResultCodes];

export type FsOkResult<T> = {
    code: typeof FsResultCodes["Ok"];
    value: T;
};

export type FsResult<T> = FsOkResult<T> | {
    code: typeof FsResultCodes[keyof Omit<typeof FsResultCodes, "Ok">];
};

/// Set value directly on the result and bypass type checking
export const setOkValue = <A, B>(result: FsResult<A>, value: B): FsOkResult<B> => {
    const r = result as unknown as FsOkResult<B>;
    r.code = FsResultCodes.Ok;
    r.value = value;
    return r;
}

export const setErrValue = <A, B>(result: FsResult<A>, err: FsResultCode): FsResult<B> => {
    // @ts-expect-error just let me do this pls
    delete result.value;
    const r = result as unknown as FsResult<B>;
    r.code = err;
    return r;
}

