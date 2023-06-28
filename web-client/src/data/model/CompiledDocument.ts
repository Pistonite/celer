//! CompiledDocument
//!
//! This is the object returned from the compiler.

import { DocumentMapParameters, DocumentMetadata } from "./util";

/// The root object of the compiled document
export type CompiledDocument = {
    /// The project section.
    project: CompiledDocumentProject,
    /// TODO the default config section
    defaultConfig: any,
    /// TODO the route section
    route: any,
}

/// The project section of the compiled document
type CompiledDocumentProject = DocumentMetadata & {
    /// The map parameters
    map: DocumentMapParameters,
	 
}

