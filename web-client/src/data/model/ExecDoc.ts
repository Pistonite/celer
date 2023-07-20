//! ExecutedDocument
//!
//! This is the object returned from the engine.

import {
    DocDiagnostic,
    DocIconMap,
    DocMapParameters,
    DocMetadata,
    DocNote,
    DocRichText,
    DocTagMap,
    GameCoord,
    MapIcon,
    MapLine,
    MapMarker,
} from "./util";

/// The executed document
///
/// This is the output of the engine, ready to be rendered in the map and document view.
///
/// All coordinates should be GameCoord at this point
export type ExecDoc = {
    /// If the document is loaded
    ///
    /// This is true if the document can be rendered,
    /// regardless if the document has engine or compiler errors.
    /// The engine and compiler errors can be included as lines in the document.
    loaded: boolean;
    /// TODO define all project properties
    project: DocMetadata & {
        /// The map parameters
        map: DocMapParameters;
        /// The icon map
        icons: DocIconMap;
        /// The tag map
        tags: DocTagMap;
    };
    /// The document sections
    route: ExecDocSection[];
    /// Map features for each document section
    map: ExecDocMapSection[];
};

/// Executed document section
export type ExecDocSection = {
    /// Name of the section
    name: string;
    /// The lines in the section
    lines: ExecDocLine[];
};

/// One line in the executed document
export type ExecDocLine = {
    /// Section number
    section: number;
    /// Line index in section
    index: number;
    /// primary text content of the line
    text: DocRichText[];
    /// Line color
    lineColor: string;
    /// Corresponding map coord
    mapCoord: GameCoord;
    /// Diagnostic messages
    diagnostics: DocDiagnostic[];
    /// The icon id to show on the document
    icon: string;
    /// Secondary text to show below the primary text
    secondaryText: DocRichText[];
    /// Counter text to display
    counterText?: DocRichText;
    /// The notes
    notes: DocNote[];
};

/// The computed map properties of a route section
export type ExecDocMapSection = {
    /// The icons
    icons: MapIcon[];
    /// The lines
    lines: MapLine[];
    /// The markers
    markers: MapMarker[];
};

/// Get the location relative to another location by line delta
///
/// If the new location is out of bound, the first or last line is returned.
/// The return value is always a valid line location
export const getRelativeLocation = (doc: ExecDoc, section: number, line: number, delta: number): { section: number, line: number} => {
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
}
