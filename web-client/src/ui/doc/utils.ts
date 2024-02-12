//! Utilities

import { DocTagColor, DocTag } from "low/celerc";
import { DOMId, Logger, injectDOMStyle } from "low/utils";
import { DocLineContainerClass, DocSectionContainerClass, DocScroll } from "./components";

export const DocLog = new Logger("doc");

/// Scroll view type
export type ScrollView = {
    /// Top of the scroll
    scrollTop: number;
    /// Bottom of the scroll
    scrollBottom: number;
};

/// Get the scroll position of the document viewer
///
/// Returns undefined if the document viewer is not found
export const getScrollView = (): undefined | ScrollView => {
    const scrollElement = DocScroll.get();
    if (!scrollElement) {
        return undefined;
    }
    // scroll relative to container
    const scrollTop = scrollElement.scrollTop;
    const scrollBottom = scrollTop + scrollElement.clientHeight;
    return { scrollTop, scrollBottom };
};

/// Get the section and line indices from an element's data attributes
///
/// The first value is section index and the second is line index
export const getLineLocationFromElement = (
    element: HTMLElement,
): [number, number] => {
    const { line, section } = element.dataset;
    const sectionIndex = parseInt(section ?? "0");
    const lineIndex = parseInt(line ?? "0");
    return [sectionIndex, lineIndex];
};

/// Get the offset of the scroll container relative to baseElementId
///
/// use DocContainer for relative to the entire document
/// use DocContentContainer for relative to the main content
///
/// line.getBoundingClientRect().y - containerOffsetY = line position relative to container
export const getScrollContainerOffsetY = (
    element: DOMId<string, HTMLElement>,
): number => {
    const containerElement = element.get();
    if (!containerElement) {
        return 0;
    }
    return containerElement.getBoundingClientRect().y;
};

/// Find a line element by its section and line indices
export const findLineByIndex = (
    sectionIndex: number,
    lineIndex: number,
): HTMLElement | undefined => {
    return DocLineContainerClass.query(
        `[data-section="${sectionIndex}"][data-line="${lineIndex}"]`,
    );
};

/// Find a note container element by its section and line indices
export const findNoteByIndex = (
    sectionIndex: number,
    lineIndex: number,
): HTMLElement | undefined => {
    const e = document.querySelector(
        `.docnote-container[data-section="${sectionIndex}"][data-line="${lineIndex}"]`,
    );
    return (e as HTMLElement) ?? undefined;
};

/// Find a section container element by its section index
export const findSectionByIndex = (
    sectionIndex: number,
): HTMLElement | undefined => {
    return DocSectionContainerClass.query(`[data-section="${sectionIndex}"]`);
};

/// Get a line's scroll position view
export const getLineScrollView = (
    line: HTMLElement,
    containerOffsetY: number,
): ScrollView => {
    const top = line.getBoundingClientRect().y - containerOffsetY;
    return {
        scrollTop: top,
        scrollBottom: top + line.clientHeight,
    };
};

export const RichTextClassName = "rich-text";

/// Update the styles/classes for rich tags
export const updateDocTagsStyle = (tags: Readonly<Record<string, DocTag>>) => {
    const css = Object.entries(tags)
        .map(([tag, data]) => {
            let css = `.${getTagClassName(tag)}{`;
            if (data.bold) {
                css += "font-weight:bold;";
            }
            if (data.italic) {
                css += "font-style:italic;";
            }
            if (data.strikethrough || data.underline) {
                css += "text-decoration:";
                if (data.strikethrough) {
                    css += "line-through ";
                }
                if (data.underline) {
                    css += "underline";
                }
                css += ";";
            }
            if (data.color) {
                css += createCssStringForColor(data.color, "fg");
            }
            if (data.background) {
                css += createCssStringForColor(data.background, "bg");
            }
            return css + "}";
        })
        .join("");
    injectDOMStyle("rich-text", css);
    DocLog.info("rich text css updated.");
};

const createCssStringForColor = (color: DocTagColor, type: "fg" | "bg") => {
    if (typeof color === "string") {
        return `--rich-text-${type}-light:${color};--rich-text-${type}-dark:${color};`;
    }
    let css = "";
    if (color.light) {
        css += `--rich-text-${type}-light:${color.light};`;
    }
    if (color.dark) {
        css += `--rich-text-${type}-dark:${color.dark};`;
    }
    return css;
};

export const getTagClassName = (tag: string) => {
    return `rich-text-tag--${getTagCanonicalName(tag)}`;
};

/// Clean the tag name and only keep alphanumerical values and dashes
const getTagCanonicalName = (tag: string) => {
    return tag.toLowerCase().replaceAll(/[^a-z0-9-]/g, "-");
};
