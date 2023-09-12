//! low/wasm
//!
//! WebAssembly utilities

/// WebAssembly result type
/// 
/// The result type has a flag indicating success or failure, and a value.
export type WasmResult<T> = [true, T] | [false, any]; // eslint-disable-line @typescript-eslint/no-explicit-any

/// no-throw Wrapper function for calling WebAssembly functions
export const wasmCall = async <T>(fn: () => Promise<T>): Promise<WasmResult<T>> => {
    try {
        return [true, await fn()];
    } catch (e) {
        return [false, e];
    }
};

declare global {
    interface Window {
        __yield: () => Promise<void>;
    }
}

window.__yield = () => {
    return new Promise((resolve) => {
        setTimeout(resolve, 0);
    });
}
