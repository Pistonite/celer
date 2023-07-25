use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use serde_json::Value;
use celerctypes::{ExecDoc, RouteMetadata, DocRichText, GameCoord, DocDiagnostic, DocNote, MapIcon, MapMarker, MapLine, ExecLine, ExecMapSection};

/// Compiled Document
pub struct CompDoc {
    /// Project metadata
    project: RouteMetadata,
    // TODO: compiler info
    route: Vec<CompSection>,
}

impl From<CompDoc> for ExecDoc {
    fn from(comp_doc: CompDoc) -> Self {
        Default::default()
    }
}

/// Compiled Section
pub struct CompSection {
    /// Name of the section
    name: String,
    /// The lines in the section
    lines: Vec<CompLine>,
}

pub struct CompLine {
    /// Primary text content of the line
    text: Vec<DocRichText>,
    /// Main line color
    line_color: String,
    /// Main movements of this line
    movements: Vec<CompMovement>,
    /// Other movements of the line
    other_movements: Vec<Vec<CompMovementWithColor>>,
    /// Diagnostic messages
    diagnostics: Vec<DocDiagnostic>,
    /// Icon id to show on the document
    doc_icon: Option<String>,
    /// Icon id to show on the map
    map_icon: Option<String>,
    /// Map icon priority. 0=primary, 1=secondary, >2=other
    map_icon_priority: u32,
    markers: Vec<CompMarker>,
    /// Secondary text to show below the primary text
    pub secondary_text: Vec<DocRichText>,
    /// Counter text to display
    pub counter_text: Option<DocRichText>,
    /// The notes
    pub notes: Vec<DocNote>,
    /// The rest of the properties as json blobs
    ///
    /// These are ignored by ExecDoc, but the transformers can use them
    pub properties: HashMap<String, Value>,
}

impl CompLine {
    /// Execute the line.
    ///
    /// Map features will be added to the ExecMapSection
    pub fn exec(
        &self, 
        section_number: usize, 
        line_number: usize, 
        map_section: &mut ExecMapSection
    ) -> ExecLine {
    }
}

pub struct CompMarker {
    /// The coord of the marker
    at: GameCoord,
    /// The color of the marker
    color: String,
}

/// Compiled map movement
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompMovement {
    /// The target coord to move to
    to: GameCoord,
    /// If the movement is a warp
    warp: bool,
}

/// Compiled map movement with color
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompMovementWithColor {
    /// The color of the movement
    color: String,
    /// The movement
    #[serde(flatten)]
    movement: CompMovement,
}
