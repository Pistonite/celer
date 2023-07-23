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
} from "./utils";

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
