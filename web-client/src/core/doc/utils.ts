//! Utilities for document

import { DocPoorText, DocRichText, DocTag, ExecDoc } from "low/compiler.g";

import {
    DocSettingsState,
    PerDocSettings,
    initialPerDocSettings,
} from "./state";

/// Get per-doc settings by doc id
export const getPerDocSettings = (
    state: DocSettingsState,
    docId: string,
): PerDocSettings => {
    if (!state.perDoc[docId]) {
        return initialPerDocSettings;
    }
    return state.perDoc[docId];
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

/// Rich text type with resolved tag
export type RichText = Omit<DocRichText, "tag"> & {
    /// The tag of the text
    tag?: DocTag;
};

/// Function to remove the tag from the text and return the just text content
export const removeTags = (text: Omit<RichText, "tag">[]): string => {
    return text.map(removeTag).join("");
};

export const removeTag = (text: Omit<RichText, "tag">): string => {
    return text.text;
};

/// Return just the text content of poor texts
export const removeLinks = (text: DocPoorText[]): string => {
    return text.map((t) => t.data).join("");
}
