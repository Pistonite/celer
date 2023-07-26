use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use serde_json::Value;
use celerctypes::{ExecDoc, RouteMetadata, DocRichText, GameCoord, DocDiagnostic, MapIcon, MapMarker, MapLine, ExecLine, ExecMapSection, DocNote};

mod exec;

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
#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct CompSection {
    /// Name of the section
    name: String,
    /// The lines in the section
    lines: Vec<CompLine>,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
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
    /// Coordinate of the map icon
    map_coord: GameCoord,
    /// Map icon priority. 0=primary, 1=secondary, >2=other
    map_icon_priority: i32,
    /// Map markers
    markers: Vec<CompMarker>,
    /// Secondary text to show below the primary text
    secondary_text: Vec<DocRichText>,
    /// Counter text to display
    counter_text: Option<DocRichText>,
    /// The notes
    notes: Vec<DocNote>,
    /// The rest of the properties as json blobs
    ///
    /// These are ignored by ExecDoc, but the transformers can use them
    #[serde(skip)]
    properties: HashMap<String, Value>,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct CompMarker {
    /// The coord of the marker
    at: GameCoord,
    /// The color of the marker
    color: String,
}

/// Compiled map movement
#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct CompMovement {
    /// The target coord to move to
    to: GameCoord,
    /// If the movement is a warp
    warp: bool,
}

/// Compiled map movement with color
#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct CompMovementWithColor {
    /// The color of the movement
    color: String,
    /// The movement
    #[serde(flatten)]
    movement: CompMovement,
}
const DEFAULT_LINE_COLOR: &str = "#38f";
const DEFAULT_MARKER_COLOR: &str = "#f00";
