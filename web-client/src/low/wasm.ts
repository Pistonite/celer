//! low/wasm
//!
//! WebAssembly utilities

/// WebAssembly result type
/// 
/// The result type has a flag indicating success or failure, and a value.
export type Result<T, E> = [true, T] | [false, E]; 

/// Optional type for interfacing with Rust
export type Option<T> = T | undefined;

/// no-throw Wrapper function for calling WebAssembly functions
export const safeCall = async <T>(fn: () => Promise<T>): Promise<Result<T, any /* eslint-disable-line @typescript-eslint/no-explicit-any*/>> => {
    try {
        return [true, await fn()];
    } catch (e) {
        return [false, e];
    }
};

/// no-throw Wrapper function for calling WebAssembly functions that already return results
export const flatSafeCall = async <T, E>(fn: () => Promise<Result<T, E>>): Promise<Result<T, any /* eslint-disable-line @typescript-eslint/no-explicit-any */>> => {
    try {
        return await fn();
    } catch (e) {
        return [false, e];
    }
}

