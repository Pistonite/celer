export type Option<T> = T | undefined;

export type Result<T, E> = UncheckedResult<T, E> & (Ok<T> | Err<E>);

export interface UncheckedResult<T, E> {
    /// Returns true if this is an Ok value
    isOk: () => this is Ok<T>;

    /// Returns true if this is an Err value
    isErr: () => this is Err<E>;

    /// Make this an Ok with the given value
    /// May mutate `this`. The previous reference should no longer be used
    makeOk: <T2>(value: T2) => Result<T2, never>;

    /// Make this an Err with the given value
    /// May mutate `this`. The previous reference should no longer be used
    makeErr: <E2>(value: E2) => Result<never, E2>;

    /// Apply fn to value if this is an Ok, leaving an Err unchanged
    /// May mutate `this`. The previous reference should no longer be used
    map: <T2>(fn: (value: T) => T2) => Result<T2, E>;

    /// Apply fn to value if this is an Err, leaving an Ok unchanged
    /// May mutate `this`. The previous reference should no longer be used
    mapErr: <E2>(fn: (value: E) => E2) => Result<T, E2>;
}

export interface Ok<T> extends UncheckedResult<T, never> {
    inner: () => T;
}

export interface Err<E> extends UncheckedResult<never, E> {
    inner: () => E;
}

export function allocOk<T>(value: T): Result<T, never>;
export function allocOk(): Result<void, never>;
export function allocErr<E>(value: E): Result<never, E>;
export function wrap<T>(fn: () => T): Result<T, unknown>;
export function wrapAsync<T>(fn: () => Promise<T>): Promise<Result<T, unknown>>;
