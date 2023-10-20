//! Utilities

import { DocColor, DocRichText, DocTag } from "low/celerc";
import { Logger } from "low/utils";

export const DocLog = new Logger("doc");

// Constants

/// The id of the doc scrolling div (parent of container)
export const DocScrollId = "doc-scroll";
/// The id of the container of doc view
export const DocContainerId = "doc-container";
/// The id of the container of main doc content (excluding preface)
export const DocContentContainerId = "doc-content-container";
/// Class for the doc line container
export const DocLineContainerClass = "docline-container";

/// Helper function to resolve tag names to the tag definition
// export const resolveTags = (
//     tagMap: Record<string, DocTag>,
//     docRichTexts: DocRichText[],
// ): RichText[] => {
//     return docRichTexts.map((docRichText) => resolveTag(tagMap, docRichText));
// };

// export const resolveTag = (
//     tagMap: Record<string, DocTag>,
//     docRichText: DocRichText,
// ): RichText => {
//     const { tag, text, link } = docRichText;
//     if (!tag) {
//         return { text, link };
//     }

//     const tagDef = tagMap[tag];
//     if (!tagDef) {
//         // Silently ignore unknown tag because compiler will add a warning (TODO: make sure you actually you taht)
//         return { text, link };
//     }
//     return {
//         text,
//         tag: tagDef,
//         link,
//     };
// };

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
    const scrollElement = document.getElementById(DocScrollId);
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
/// use DocContainerId for relative to the entire document
/// use DocContentContainerId for relative to the main content
///
/// line.getBoundingClientRect().y - containerOffsetY = line position relative to container
export const getScrollContainerOffsetY = (baseElementId: string): number => {
    const containerElement = document.getElementById(baseElementId);
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
    let styleTag = document.querySelector("style[data-inject=\"rich-text\"");
    if (!styleTag) {
        DocLog.info("creating rich text css...");
        styleTag = document.createElement("style");
        styleTag.setAttribute("data-inject", "rich-text");
        const head = document.querySelector("head");
        if (!head) {
            DocLog.error("cannot find `head`");
            return;
        }
        head.appendChild(styleTag);
    }
    (styleTag as HTMLStyleElement).innerText=Object.entries(tags).map(([tag, data]) => {
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
            css += ";"
        }
        if (data.color) {
            css += createCssStringForColor(data.color, "fg");
        }
        if (data.background)
        {
            css += createCssStringForColor(data.background, "bg");
        }
        return css + "}";
    }).join("");
    DocLog.info("rich test css updated.");
}

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
}

export const getTagClassName = (tag: string) => {
    return `rich-text-tag--${getTagCanonicalName(tag)}`;
}

/// Clean the tag name and only keep alphanumerical values and dashes
const getTagCanonicalName = (tag: string) => {
    return tag.toLowerCase().replaceAll(/[^a-z0-9\-]/g, "-");
}