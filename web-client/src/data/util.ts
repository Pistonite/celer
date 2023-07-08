//! data/util
//!
//! Baseline utilities that all layers can use

/// Create a log manager
export const createLogMgr = (prefix: string) => {
    return {
        info: (msg: string) => {
            console.info(`[${prefix}] ${msg}`); // eslint-disable-line no-console
        },
        warn: (msg: string) => {
            console.warn(`[${prefix}] ${msg}`); // eslint-disable-line no-console
        },
        error: (msg: string) => {
            console.error(`[${prefix}] ${msg}`); // eslint-disable-line no-console
        },
    };
};
