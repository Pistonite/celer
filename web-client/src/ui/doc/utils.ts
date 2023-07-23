//! Utilities

import { RichText } from "core/doc";
import { DocRichText, DocTag} from "low/compiler.g";
import { Logger } from "low/utils";

export const DocLog = new Logger("doc");

// Constants

/// The id of the doc scrolling div (parent of container)
export const DocScrollId = "doc-scroll";
/// The id of the container of doc view
export const DocContainerId = "doc-container";
/// Class for the doc line container
export const DocLineContainerClass = "docline-container";

/// Helper function to resolve tag names to the tag definition
export const resolveTags = (
    tagMap: Record<string, DocTag>,
    docRichTexts: DocRichText[],
): RichText[] => {
    return docRichTexts.map((docRichText) => resolveTag(tagMap, docRichText));
};

export const resolveTag = (
    tagMap: Record<string, DocTag>,
    docRichText: DocRichText,
): RichText => {
    const { tag, text } = docRichText;
    if (!tag) {
        return { text };
    }

    const tagDef = tagMap[tag];
    if (!tagDef) {
        // Silently ignore unknown tag because compiler will add a warning (TODO: make sure you actually you taht)
        return { text };
    }
    return {
        text,
        tag: tagDef,
    };
};

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

/// Get the offset of the scroll container
///
/// line.getBoundingClientRect().y - containerOffsetY = line position relative to container
export const getScrollContainerOffsetY = (): number => {
    const containerElement = document.getElementById(DocContainerId);
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
