export type Result<T, E> = [true, T] | [false, E]; 
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

