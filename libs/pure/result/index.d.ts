//! pure/result
//!
//! TypeScript based result return type. See README.md for more information.

/// Handle used to interact with the result system
/// inside a function
///
/// The functions here are just for TypeScript magic. See README.md
/// for how to use them
export interface ResultHandle {
    /// Erase the types inferred by TypeScript
    ///
    /// The typical usage is `r = r.erase()`
    /// NOTE: this does NOT actually erase the value!!!
    erase: () => ResultHandle,

    /// Put a result into the handle for checking
    put: <T, E>(r: Result<T, E>) => asserts this is Result<T, E>,

    /// Call a throwing function inside a result-handling function,
    /// capturing the result inside this handle
    tryCatch: <T>(r: ResultHandle, fn: () => T) => Result<T, unknown>,

    /// Await a throwing promise inside a result-handling function,
    /// capturing the result inside this handle
    tryCatchAsync: <T>(r: ResultHandle, promise: Promise<T>) => Promise<Result<T, unknown>>,

    /// Put an ok value into this handle
    ///
    /// Typically used at return position (i.e. `return r.putOk(value)`)
    putOk: <T, E>(value: T) => Result<T, E>,

    /// Put an ok value as void (undefined)
    ///
    /// Typically used at return position (i.e. `return r.voidOk()`)
    voidOk: <E>() => Result<void, E>,

    /// Put an error value into this handle
    ///
    /// Typically used at return position (i.e. `return r.putErr(error)`)
    putErr: <T, E>(error: E) => Result<T, E>,

    /// Create a new handle detached from this one
    ///
    /// See README.md for when this is needed
    fork: () => ResultHandle,
}

/// Type of result before it is checked
export interface UncheckedResult<T, E> {
    isOk: () => this is Ok<T>,
    isErr: () => this is Err<E>,
}


/// Type of result used internally in functions that take ResultHandle
/// This can be converted back to ResultHandle by calling `erase()`
export type Result<T, E> = ResultHandle & UncheckedResult<T, E> & (Ok<T> | Err<E>);

/// Result checked to be Ok
export interface Ok<T> extends StableOk<T> {
    /// Cast the value back to a result with any error type
    ///
    /// Used to re-return the result. See README.md for more information
    ret: <E>() => Result<T, E>,
}

/// Result checked to be Err
interface Err<E> extends StableErr<E> {
    /// Cast the value back to a result with any error type
    ///
    /// Used to re-return the result. See README.md for more information
    ret: <T>() => Result<T, E>,
}

/// Type of result returned by the tryXXX wrapper functions
///
/// This result is detached from the handle and will not leak information.
/// For example, an Ok result will only contain the value, not temporary error
/// previous stored.
export type StableResult<T, E> = StableUncheckedResult<T, E> & (StableOk<T> | StableErr<E>);
export interface StableUncheckedResult<T, E> {
    isOk: () => this is StableOk<T>,
    isErr: () => this is StableErr<E>,
}
export type StableOk<T> = { value: T };
export type StableErr<E> = { error: E };

/// Invoke a function that takes a ResultHandle and return a Result
export function tryInvoke<T, E>(fn: (r: ResultHandle) => Result<T, E>): StableResult<T, E>;

/// Invoke an async function that takes a ResultHandle and return a Promsie<Result>
///
/// Note that if the async function throws, it will NOT be captured
export function tryInvokeAsync<T, E>(fn: (r: ResultHandle) => Promise<Result<T, E>>): Promise<StableResult<T, E>>;

/// Wrap a function that may throw an error and return a Result, capturing the error
export function tryCatch<T>(fn: () => T): StableResult<T, unknown>;

/// Wrap a promise that may throw when awaited and return a Result, capturing the error
export function tryCatchAsync<T>(x: Promise<T>): Promise<StableResult<T, unknown>>;

