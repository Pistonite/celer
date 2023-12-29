//! Public API Types
//!
//! These types are shared between compiler and client. They are exposed through TypeScript
//! definitions and WASM ABI

use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::comp::DocNote;
use crate::lang::{DocDiagnostic, DocPoorText, DocPoorTextBlock, DocRichText, DocRichTextBlock};
use crate::macros::derive_wasm;
use crate::prep::{GameCoord, RouteConfig};
use crate::util::StringMap;

/// The executed document
///
/// This is the final output of compiler with
/// map items separated from doc items
#[derive(Default, Serialize, Deserialize, Debug, Clone)]
#[derive_wasm]
pub struct ExecDoc<'a> {
    /// Project metadata
    pub project: Cow<'a, RouteConfig>,
    /// The preface
    pub preface: Vec<DocRichText>,
    /// The route
    pub route: Vec<ExecSection>,
    /// Overall diagnostics (that don't apply to any line)
    pub diagnostics: Vec<DocDiagnostic>,
}

/// A section in the executed document
#[derive(PartialEq, Default, Serialize, Deserialize, Debug, Clone)]
#[derive_wasm]
pub struct ExecSection {
    /// Name of the section
    pub name: String,
    /// The lines in the section
    pub lines: Vec<ExecLine>,
    /// The map items in this section
    pub map: ExecMapSection,
}

/// A line in the executed document
#[derive(PartialEq, Default, Serialize, Deserialize, Debug, Clone)]
#[derive_wasm]
pub struct ExecLine {
    /// Section number
    pub section: usize,
    /// Line index in section
    pub index: usize,
    /// Primary text content of the line
    pub text: DocRichText,
    /// Line color
    pub line_color: String,
    /// Corresponding map coordinates
    pub map_coords: Vec<GameCoord>,
    /// Diagnostic messages
    pub diagnostics: Vec<DocDiagnostic>,
    /// The icon id to show on the document
    pub icon: Option<String>,
    /// Secondary text to show below the primary text
    pub secondary_text: DocRichText,
    /// Counter text to display
    pub counter_text: Option<DocRichTextBlock>,
    /// The notes
    pub notes: Vec<DocNote>,
    /// The split name, if different from text
    pub split_name: Option<String>,
    /// If the line text is a banner
    pub is_banner: bool,
}

/// Map features for one section
#[derive(PartialEq, Default, Serialize, Deserialize, Debug, Clone)]
#[derive_wasm]
pub struct ExecMapSection {
    /// The icons
    pub icons: Vec<MapIcon>,
    /// The markers
    pub markers: Vec<MapMarker>,
    /// The lines
    pub lines: Vec<MapLine>,
}

/// Icon on the map
#[derive(PartialEq, Default, Serialize, Deserialize, Debug, Clone)]
#[derive_wasm]
pub struct MapIcon {
    /// Internal icon name (usually kebab-case)
    pub id: String,
    /// Game coordinate for the icon
    pub coord: GameCoord,
    /// The corresponding line index in section of the document
    pub line_index: usize,
    /// The corresponding section number in the document
    pub section_index: usize,
    /// The priority of the icon (0 = primary, 1 = secondary)
    pub priority: i64,
}

/// Markers on the map
#[derive(PartialEq, Default, Serialize, Deserialize, Debug, Clone)]
#[derive_wasm]
pub struct MapMarker {
    pub coord: GameCoord,
    /// The corresponding line index in section of the document
    pub line_index: usize,
    /// The corresponding section number in the document
    pub section_index: usize,
    /// Color of the marker
    pub color: String,
}

/// Paths on the map
///
/// The coordinates do not have to be on the same map layer.
/// The map will automatically split the path if it croses map layers.
#[derive(PartialEq, Default, Serialize, Deserialize, Debug, Clone)]
#[derive_wasm]
pub struct MapLine {
    /// Color of the line
    pub color: String,
    /// Points on the line
    pub points: Vec<GameCoord>,
}
