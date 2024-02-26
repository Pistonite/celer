//! Utilities

import { DOMId } from "low/utils";
import {
    DocLineContainerClass,
    DocSectionContainerClass,
    DocScroll,
    DocNoteContainerClass,
} from "./components";

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
    return DocNoteContainerClass.query(
        `[data-section="${sectionIndex}"][data-line="${lineIndex}"]`,
    );
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
