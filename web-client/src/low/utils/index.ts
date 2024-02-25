//! low/utils
//!
//! Low level utilities that all layers can use

export * from "./Alert";
export * from "./IdleMgr";
export * from "./Debouncer";
export * from "./html";
export * from "./ReentrantLock";
export * from "./WorkerHost";
export * from "./Yielder";
export * from "./logging";

export const shallowArrayEqual = <T>(a: T[], b: T[]): boolean => {
    if (a.length !== b.length) {
        return false;
    }
    for (let i = 0; i < a.length; ++i) {
        if (a[i] !== b[i]) {
            return false;
        }
    }
    return true;
};

