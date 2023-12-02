//! Utilities

import { DocColor, DocTag } from "low/celerc";
import { DOMClass, DOMId, Logger } from "low/utils";

export const DocLog = new Logger("doc");

// Constants

/// The scrolling div of the document (parent of container)
export const DocScroll = new DOMId<HTMLElement>("doc-scroll");
/// The container div of the document
export const DocContainer = new DOMId<HTMLElement>("doc-container");
/// The container div of main doc content (excluding preface)
export const DocContentContainer = new DOMId<HTMLElement>(
    "doccontent-container",
);
/// The note panel
export const DocNoteContainer = new DOMId<HTMLElement>("doc-side");
/// Class for the doc line container
export const DocLineContainerClass = "docline-container";

/// Class for the section heads (title) in the document
export const DocSectionHead = new DOMClass("docsection-head");
DocSectionHead.style({
    "box-sizing": "border-box",
    padding: "16px 0px 16px 64px",
    " span": {
        "word-wrap": "break-word",
    },
});

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
export const getScrollContainerOffsetY = <E extends HTMLElement>(
    element: DOMId<E>,
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
    const e = document.querySelector(
        `.${DocLineContainerClass}[data-section="${sectionIndex}"][data-line="${lineIndex}"]`,
    );
    return (e as HTMLElement) ?? undefined;
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
    const e = document.querySelector(
        `.docsection-container[data-section="${sectionIndex}"]`,
    );
    return (e as HTMLElement) ?? undefined;
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
    const styleTag = getInjectedStyleTag("rich-text");
    styleTag.innerText = Object.entries(tags)
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
    DocLog.info("rich text css updated.");
};

/// Get or inject a style tag with the id. The id sets the "data-inject" attribute
export const getInjectedStyleTag = (id: string): HTMLStyleElement => {
    let styleTag = document.querySelector(`style[data-inject="${id}"`);
    if (!styleTag) {
        DocLog.info(`creating injected ${id} tag...`);
        styleTag = document.createElement("style");
        styleTag.setAttribute("data-inject", id);
        const head = document.querySelector("head");
        if (!head) {
            DocLog.error("cannot find `head`");
        } else {
            head.appendChild(styleTag);
        }
    }
    return styleTag as HTMLStyleElement;
};

const createCssStringForColor = (color: DocColor, type: "fg" | "bg") => {
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
