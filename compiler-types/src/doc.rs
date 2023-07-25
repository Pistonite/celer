//! Types for the doc
use serde::{Serialize, Deserialize};
use ts_rs::TS;

use crate::map::{GameCoord, MapIcon, MapLine, MapMarker};

/// A section in the executed document
#[derive(Default, Serialize, Deserialize, Debug, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ExecSection {
    /// Name of the section
    pub name: String,
    /// The lines in the section
    pub lines: Vec<ExecLine>,
    /// The map items in this section
    pub map: ExecMapSection,
}

/// Map items in a section
#[derive(Default, Serialize, Deserialize, Debug, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ExecMapSection {
    /// The icons
    icons: Vec<MapIcon>,
    /// The lines
    lines: Vec<MapLine>,
    /// The markers
    markers: Vec<MapMarker>,
}

/// A line in the executed document
#[derive(Default, Serialize, Deserialize, Debug, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ExecLine {
    /// Section number
    pub section: usize,
    /// Line index in section
    pub index: usize,
    /// Primary text content of the line
    pub text: Vec<DocRichText>,
    /// Line color
    pub line_color: String,
    /// Corresponding map coordinates
    pub map_coords: Vec<GameCoord>,
    /// Diagnostic messages
    pub diagnostics: Vec<DocDiagnostic>,
    /// The icon id to show on the document
    pub icon: Option<String>,
    /// Secondary text to show below the primary text
    pub secondary_text: Vec<DocRichText>,
    /// Counter text to display
    pub counter_text: Option<DocRichText>,
    /// The notes
    pub notes: Vec<DocNote>,
}

/// Diagnostic message
#[derive(Default, Serialize, Deserialize, Debug, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct DocDiagnostic {
    /// The diagnostic message
    pub msg: String,
    /// Type of the diagnostic
    #[serde(rename = "type")]
    pub msg_type: String,
    /// Source of the diagnostic
    ///
    /// User can filter diagnostics by source
    pub source: String,
}

/// Document rich text type
#[derive(Default, Serialize, Deserialize, Debug, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct DocRichText {
    /// The tag name of the text
    ///
    /// Each block only contains one tag
    tag: Option<String>,
    /// The text content
    text: String,
}

/// Document note block
#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[serde(tag = "type", rename_all = "camelCase")]
#[ts(export)]
pub enum DocNote {
    Text {
        content: Vec<DocRichText>,
    },
    Image {
        link: String,
    },
    Video {
        link: String,
    },
}
