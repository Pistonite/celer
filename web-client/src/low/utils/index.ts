//! low/utils
//!
//! Low level utilities that all layers can use

export * from "./Logger";
export * from "./Debouncer";


/// Switch theme by switching the css file in link tag
export function switchTheme(theme: string) {
    const linkTag = document.getElementById("docline-theme") as HTMLLinkElement;
    if (!linkTag) {
        console.error("Could not find theme link tag");
        return;
    }
    linkTag.href = `/themes/${theme}.min.css`;
}
