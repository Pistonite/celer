//! Types for the doc
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{ExecMapSection, GameCoord};

/// A section in the executed document
#[derive(PartialEq, Default, Serialize, Deserialize, Debug, Clone, TS)]
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

/// A line in the executed document
#[derive(PartialEq, Default, Serialize, Deserialize, Debug, Clone, TS)]
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
    /// The split name, if different from text
    pub split_name: Option<String>,
}

/// Diagnostic message
#[derive(PartialEq, Default, Serialize, Deserialize, Debug, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct DocDiagnostic {
    /// The diagnostic message
    pub msg: Vec<DocPoorText>,
    /// Type of the diagnostic
    ///
    /// The builtin ones are "error" and "warn", but this can be any value.
    /// Custom themes might utilize this for displaying extra messages.
    #[serde(rename = "type")]
    pub msg_type: String,
    /// Source of the diagnostic
    ///
    /// User can filter diagnostics by source
    pub source: String,
}

/// Document rich text type
#[derive(PartialEq, Default, Serialize, Deserialize, Debug, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct DocRichText {
    /// The tag name of the text
    ///
    /// Each block only contains one tag
    pub tag: Option<String>,
    /// The text content
    pub text: String,
    /// The hyperlink of the text.
    pub link: Option<String>,
}

impl DocRichText {
    /// Create a rich text block with no tag
    pub fn text(text: &str) -> Self {
        Self {
            tag: None,
            text: text.to_string(),
            link: None,
        }
    }

    /// Create a rich text block with a tag
    pub fn with_tag(tag: &str, text: &str) -> Self {
        Self {
            tag: Some(tag.to_string()),
            text: text.to_string(),
            link: None,
        }
    }
}

/// Document poor text type. Just text or link
#[derive(PartialEq, Serialize, Deserialize, Debug, Clone, TS)]
#[serde(tag = "type", content = "data", rename_all = "camelCase")]
#[ts(export)]
pub enum DocPoorText {
    Text(String),
    Link(String),
}

/// Document note block
#[derive(PartialEq, Serialize, Deserialize, Debug, Clone, TS)]
#[serde(tag = "type", rename_all = "camelCase")]
#[ts(export)]
pub enum DocNote {
    Text { content: Vec<DocRichText> },
    Image { link: String },
    Video { link: String },
}
