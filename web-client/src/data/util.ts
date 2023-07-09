//! data/util
//!
//! Baseline utilities that all layers can use

/// Create a log manager
export const createLogMgr = (prefix: string) => {
    return {
        info: (msg: string) => {
            console.info(`[${prefix}] ${msg}`);  
        },
        warn: (msg: string) => {
            console.warn(`[${prefix}] ${msg}`);  
        },
        error: (msg: string) => {
            console.error(`[${prefix}] ${msg}`);  
        },
    };
};

/// A general-purpose debouncer
///
/// This is useful for debouncing expensive operations such as recreating the map
/// when changing the settings.
///
/// This is particularly useful outside of React where useTransition hook
/// is not available.
export class Debouncer {
    /// The timeout handle
    private handle: number | undefined;
    /// The delay in ms
    private delay: number;
    /// The callback action
    private callback: () => void;

    constructor(delay: number, callback: () => void) {
        this.delay = delay;
        this.callback = callback;
    }

    public dispatch() {
        if (this.handle) {
            clearTimeout(this.handle);
        }
        this.handle = window.setTimeout(this.callback, this.delay);
    }
}

/// Sort an array of 2-tuples by the second element,
/// which is an external order. The original array will be modified.
///
/// Return an array containing just the first elements
export const sortByExternalOrder = <T>(arr: [T, number][]): T[] => {
    return arr.sort((a, b) => a[1] - b[1]).map((x) => x[0]);
};
