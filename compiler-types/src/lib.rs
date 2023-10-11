use std::borrow::Cow;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

// this is only so that LSP server can detect the script
mod build;

mod map;
pub use map::*;
mod doc;
pub use doc::*;

/// The executed document
///
/// This is the final output of compiler with
/// map items separated from doc items
///
/// All coordinates should be [`GameCoord`] at this point
#[derive(Default, Serialize, Deserialize, Debug, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ExecDoc<'a> {
    /// Project metadata
    pub project: Cow<'a, RouteMetadata>,
    /// The preface
    pub preface: Vec<Vec<DocRichText>>,
    /// The route
    pub route: Vec<ExecSection>,
    /// Overall diagnostics (that don't apply to any line)
    pub diagnostics: Vec<DocDiagnostic>,
}

/// Metadata of the route project
///
/// This is produced by the bundling process and will not change afterwards
#[derive(PartialEq, Default, Serialize, Deserialize, Debug, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct RouteMetadata {
    /// Source of the route, could be a URL or any string
    pub source: String,
    /// Version of the project
    pub version: String,
    /// Display title of the project
    pub title: String,
    /// Map metadata
    pub map: MapMetadata,
    /// Arbitrary key-value pairs that can be used for statistics or any other value
    pub stats: HashMap<String, String>,
    /// Icon id to url map
    pub icons: HashMap<String, String>,
    /// Tag id to tag
    pub tags: HashMap<String, DocTag>,
}

/// Document tag type
///
/// Used to style text and provide extra function to the engine
#[derive(PartialEq, Default, Serialize, Deserialize, Debug, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct DocTag {
    /// Bold style
    #[serde(default)]
    bold: bool,
    /// Italic style
    #[serde(default)]
    italic: bool,
    /// Underline style
    #[serde(default)]
    underline: bool,
    /// Strikethrough style
    #[serde(default)]
    strikethrough: bool,
    /// Color of the text
    color: Option<String>,
    /// Background color of the text
    background: Option<String>,
}
