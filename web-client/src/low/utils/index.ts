//! low/utils
//!
//! Low level utilities that all layers can use

export * from "./Alert.ts";
export * from "./IdleMgr.ts";
export * from "./Debouncer.ts";
export * from "./html.ts";
export * from "./ReentrantLock.ts";
export * from "./SerialEvent.ts";
export * from "./WorkerHost.ts";
export * from "./Yielder.ts";
export * from "./logging.ts";

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
