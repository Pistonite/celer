//! low/utils
//!
//! Low level utilities that all layers can use

export * from "./Logger";
export * from "./Debouncer";
export * from "./Pool";
export * from "./FileSaver";
export * from "./Result";
export * from "./yielder";

export const isInDarkMode = () =>
    !!(
        window.matchMedia &&
        window.matchMedia("(prefers-color-scheme: dark)").matches
    );

/// Sleep for the given number of milliseconds
///
/// Example: await sleep(1000);
export const sleep = (ms: number) =>
    new Promise((resolve) => setTimeout(resolve, ms));
