//! Utilities for document

import { DocPoorText, DocRichTextBlock, ExecDoc } from "low/celerc";

// import {
//     DocSettingsState,
//     PerDocSettings,
//     initialPerDocSettings,
// } from "./state";

// /// Get per-doc settings by doc id
// export const getPerDocSettings = (
//     state: DocSettingsState,
//     docId: string,
// ): PerDocSettings => {
//     if (!state.perDoc[docId]) {
//         return initialPerDocSettings;
//     }
//     return state.perDoc[docId];
// };
//

/// Get the previous or next <delta>-th split.
export const getRelativeSplitLocation = (
    doc: ExecDoc,
    section: number,
    line: number,
    delta: number,
    splitTypes: string[],
): { section: number; line: number } => {
    let currentSection = section;
    let currentLine = line;
    const lineDelta = delta > 0 ? 1 : -1;
    let remaining = delta > 0 ? delta : -delta;
    while (remaining !== 0) {
        const newLoc = getRelativeLocation(
            doc,
            currentSection,
            currentLine,
            lineDelta,
        );
        currentSection = newLoc.section;
        currentLine = newLoc.line;

        const line = doc.route[currentSection].lines[currentLine];
        if (!line.counterText || !line.counterText.tag) {
            // the line doesn't have a counter type
            continue;
        }
        const tagName = line.counterText.tag;
        const tag = doc.project.tags[tagName];
        if (!tag || !tag.splitType) {
            // the counter type is invalid or doesn't have a split type
            continue;
        }
        if (splitTypes.includes(tag.splitType)) {
            // found a split line
            remaining -= 1;
        }
    }

    return { section: currentSection, line: currentLine };
};

/// Get the location relative to another location by line delta
///
/// If the new location is out of bound, the first or last line is returned.
/// The return value is always a valid line location
export const getRelativeLocation = (
    doc: ExecDoc,
    section: number,
    line: number,
    delta: number,
): { section: number; line: number } => {
    // Convert to absolute line index
    let absLineIndex = line;
    for (let i = section - 1; i >= 0; i--) {
        absLineIndex += doc.route[i].lines.length;
    }
    // Add delta
    absLineIndex += delta;
    if (absLineIndex <= 0) {
        return { section: 0, line: 0 };
    }
    // Convert back to section and line
    for (let i = 0; i < doc.route.length; i++) {
        if (absLineIndex < doc.route[i].lines.length) {
            return { section: i, line: absLineIndex };
        }
        absLineIndex -= doc.route[i].lines.length;
    }
    // return last line if out of bound
    return {
        section: doc.route.length - 1,
        line: doc.route[doc.route.length - 1].lines.length - 1,
    };
};

/// Function to remove the tag from the text and return the just text content
export const removeTags = (text: Omit<DocRichTextBlock, "tag">[]): string => {
    return text.map(removeTag).join("");
};

export const removeTag = (text: Omit<DocRichTextBlock, "tag">): string => {
    return text.text;
};

/// Return just the text content of poor texts
export const removeLinks = (text: DocPoorText): string => {
    return text.map((t) => t.data).join("");
};

/// Get the default split types for a document defined in the config
export const getDefaultSplitTypes = (doc: ExecDoc): string[] => {
    const splitTags = doc.project.splits;
    const output: string[] = [];
    splitTags.forEach((tag) => {
        const splitType = doc.project.tags[tag]?.splitType;
        if (splitType) {
            output.push(splitType);
        }
    });
    return output;
};

/// Get all split types defined in the document tags
export const getAllSplitTypes = (doc: ExecDoc): string[] => {
    const output = new Set<string>();
    Object.values(doc.project.tags).forEach((tag) => {
        if (tag.splitType) {
            output.add(tag.splitType);
        }
    });
    const array = Array.from(output);
    array.sort();
    return array;
};
