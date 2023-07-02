//! ExecutedDocument
//!
//! This is the object returned from the engine.

import { DocumentIconMap, DocumentMapParameters, DocumentMetadata, MapIcon } from "./util";

/// The executed document
///
/// This is the output of the engine, ready to be rendered in the map and document view.
///
/// All coordinates should be GameCoord at this point
export type ExecutedDocument = {
    /// If the document is loaded
    ///
    /// This is true if the document can be rendered,
    /// regardless if the document has engine or compiler errors.
    /// The engine and compiler errors can be included as lines in the document.
    loaded: boolean,
    /// TODO define all project properties
    project: DocumentMetadata & {
        /// The map parameters
        map: DocumentMapParameters,
        /// The icon map
        icons: DocumentIconMap,
    },
    map: ExecutedDocumentMap,
}

/// The computed map properties
///
/// This includes the lines and the icons
export type ExecutedDocumentMap = {
    /// TODO lines
    lines: any,
    /// The icons
    icons: MapIcon[],
}
