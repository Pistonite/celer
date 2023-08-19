use std::collections::HashMap;

use serde::{Serialize, Deserialize};
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
pub struct ExecDoc {
    /// Project metadata
    pub project: RouteMetadata,
    /// The route
    pub route: Vec<ExecSection>,
}

/// Metadata of the route project
///
/// This is produced by the bundling process and will not change afterwards
#[derive(PartialEq, Default, Serialize, Deserialize, Debug, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct RouteMetadata {
    /// Reference id of the project. Something like username/project
    pub name: String,
    /// Version of the project
    pub version: String,
    /// Display title of the project
    pub title: String,
    /// Map metadata
    pub map: MapMetadata,
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
    bold: bool,
    /// Italic style
    italic: bool,
    /// Underline style
    underline: bool,
    /// Strikethrough style
    strikethrough: bool,
    /// Color of the text
    color: Option<String>,
    /// Background color of the text
    background: Option<String>,
}

