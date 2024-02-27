//! pure/result
//!
//! TypeScript based result return type. See README.md for more information.

// If these look weird, it's because TypeScript is weird
// This is to get type narrowing to work most of the time
export type Ok<T> = { val: T; err?: never };
export type Err<E> = { err: E; val?: never };
export type Void<E> = { val?: never; err?: never } | { err: E };

export type Result<T, E> = Ok<T> | Err<E>;

/// Wrap a function with try-catch and return a Result.
export function tryCatch<T, E = unknown>(fn: () => T): Result<T, E> {
    try {
        return { val: fn() };
    } catch (e) {
        return { err: e as E };
    }
}

/// Wrap an async function with try-catch and return a Promise<Result>.
export async function tryAsync<T, E = unknown>(
    fn: () => Promise<T>,
): Promise<Result<T, E>> {
    try {
        return { val: await fn() };
    } catch (e) {
        return { err: e as E };
    }
}
