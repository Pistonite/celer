//! ExecutedDocument
//!
//! This is the object returned from the engine.

import {
    DocCounter,
    DocIconMap,
    DocMapParameters,
    DocMetadata,
    DocTagMap,
    ExecDocSection,
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
        /// The counter map
        ///
        /// Maps counter type to color
        counters: Record<string, DocCounter>;
    };
    /// The document sections
    route: ExecDocSection[];
    /// Map features for each document section
    map: ExecDocMapSection[];
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
