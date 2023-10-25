//! Public API Types
//!
//! These types are shared between compiler and client. They are exposed through TypeScript
//! definitions and WASM ABI

use std::borrow::Cow;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::macros::derive_wasm;

/// Compiler entry points (name, path) pairs
#[derive(PartialEq, Default, Serialize, Deserialize, Debug, Clone)]
#[derive_wasm(feature = "wasm")]
pub struct EntryPoints(#[tsify(type = "Record<string, string>")] pub HashMap<String, String>);

impl EntryPoints {
    /// Remove the aliases. Only keep the entry points that map directly to a path
    pub fn path_only(mut self) -> Self {
        self.0.retain(|_, v| v.starts_with('/'));
        self
    }
}

#[derive(PartialEq, Default, Serialize, Deserialize, Debug, Clone)]
#[derive_wasm(feature = "wasm")]
pub struct EntryPointsSorted(pub Vec<(String, String)>);

impl From<EntryPoints> for EntryPointsSorted {
    fn from(entry_points: EntryPoints) -> Self {
        let mut vec = entry_points.0.into_iter().collect::<Vec<_>>();
        vec.sort_by(|a, b| a.0.cmp(&b.0));
        Self(vec)
    }
}

/// The executed document
///
/// This is the final output of compiler with
/// map items separated from doc items
#[derive(Default, Serialize, Deserialize, Debug, Clone)]
#[derive_wasm(feature = "wasm")]
#[serde(rename_all = "camelCase")]
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
#[derive(PartialEq, Default, Serialize, Deserialize, Debug, Clone)]
#[derive_wasm(feature = "wasm")]
#[serde(rename_all = "camelCase")]
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
    #[tsify(type = "Record<string, string>")]
    pub stats: HashMap<String, String>,
    /// Icon id to url map
    #[tsify(type = "Record<string, string>")]
    pub icons: HashMap<String, String>,
    /// Tag id to tag
    #[tsify(type = "Record<string, DocTag>")]
    pub tags: HashMap<String, DocTag>,
}

/// Document tag type
///
/// Used to style text and provide extra function to the engine
#[derive(PartialEq, Default, Serialize, Deserialize, Debug, Clone)]
#[derive_wasm(feature = "wasm")]
#[serde(rename_all = "camelCase")]
pub struct DocTag {
    /// Bold style
    #[serde(default)]
    pub bold: bool,
    /// Italic style
    #[serde(default)]
    pub italic: bool,
    /// Underline style
    #[serde(default)]
    pub underline: bool,
    /// Strikethrough style
    #[serde(default)]
    pub strikethrough: bool,
    /// Color of the text (light, dark)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<DocColor>,
    /// Background color of the text (light, dark)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<DocColor>,
}

/// Document color specification
///
/// Used to specify color for [`DocTag`]s.
#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
#[derive_wasm(feature = "wasm")]
#[serde(untagged)]
pub enum DocColor {
    /// Single color for any background
    Single(String),
    /// Different colors for light and dark backgrounds
    LightDark {
        /// Color to set if the text is displayed with a light background
        #[serde(skip_serializing_if = "Option::is_none")]
        light: Option<String>,
        /// Color to set if the text is displayed with a dark background
        #[serde(skip_serializing_if = "Option::is_none")]
        dark: Option<String>,
    },
}

/// A section in the executed document
#[derive(PartialEq, Default, Serialize, Deserialize, Debug, Clone)]
#[derive_wasm(feature = "wasm")]
#[serde(rename_all = "camelCase")]
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
#[derive_wasm(feature = "wasm")]
#[serde(rename_all = "camelCase")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    /// Secondary text to show below the primary text
    pub secondary_text: Vec<DocRichText>,
    /// Counter text to display
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counter_text: Option<DocRichText>,
    /// The notes
    pub notes: Vec<DocNote>,
    /// The split name, if different from text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split_name: Option<String>,
}

/// Diagnostic message
#[derive(PartialEq, Default, Serialize, Deserialize, Debug, Clone)]
#[derive_wasm(feature = "wasm")]
#[serde(rename_all = "camelCase")]
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
#[derive(PartialEq, Default, Serialize, Deserialize, Debug, Clone)]
#[derive_wasm(feature = "wasm")]
#[serde(rename_all = "camelCase")]
pub struct DocRichText {
    /// The tag name of the text
    ///
    /// Each block only contains one tag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// The text content
    pub text: String,
    /// The hyperlink of the text.
    #[serde(skip_serializing_if = "Option::is_none")]
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
#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
#[derive_wasm(feature = "wasm")]
#[serde(tag = "type", content = "data", rename_all = "camelCase")]
pub enum DocPoorText {
    Text(String),
    Link(String),
}

/// Document note block
#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
#[derive_wasm(feature = "wasm")]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum DocNote {
    Text { content: Vec<DocRichText> },
    Image { link: String },
    Video { link: String },
}

/// Metadata of the map
///
/// This includes configuration like map layers, coordinates, etc.
#[derive(PartialEq, Default, Serialize, Deserialize, Debug, Clone)]
#[derive_wasm(feature = "wasm")]
#[serde(rename_all = "camelCase")]
pub struct MapMetadata {
    /// The map layers. First is the lowest layer.
    pub layers: Vec<MapLayerAttr>,
    /// Mapping for the coordinates in the route.
    pub coord_map: MapCoordMap,
    /// Initial coordinates
    pub initial_coord: GameCoord,
    /// Initial zoom level
    pub initial_zoom: u64,
    /// Initial map line color
    pub initial_color: String,
}

/// The mapping if 2 coordinates are specified in the route
///
/// For example, ["x", "z"] will map the coordinates
/// to the x (horizontal) and z (height) axis of the map.
///
/// Default value of 0 will be assigned to the unmapped axis.
#[derive(PartialEq, Default, Serialize, Deserialize, Debug, Clone)]
#[derive_wasm(feature = "wasm")]
pub struct MapCoordMap {
    /// Mapping for 2d coordinates in the route.
    #[serde(rename = "2d")]
    pub mapping_2d: (Axis, Axis),
    // Mapping for 3d coordinates in the route.
    #[serde(rename = "3d")]
    pub mapping_3d: (Axis, Axis, Axis),
}

/// Attribute (definition) of a map layer in the route
#[derive(PartialEq, Default, Serialize, Deserialize, Debug, Clone)]
#[derive_wasm(feature = "wasm")]
#[serde(rename_all = "camelCase")]
pub struct MapLayerAttr {
    /// Display name of the layer
    ///
    /// This is visible in the layer switch UI
    pub name: String,

    /// The tileset url template, with {x} {y} {z} as placeholders.
    ///
    /// The url should conform to the leaflet tile layer API:
    /// https://leafletjs.com/reference.html#tilelayer
    pub template_url: String,

    /// The raster coordinate size
    ///
    /// See: https://github.com/commenthol/leaflet-rastercoords.
    /// Form is [width, height]
    pub size: (u64, u64),

    /// Min and max zoom levels
    pub zoom_bounds: (u64, u64),

    /// Max native zoom of the tileset
    pub max_native_zoom: u64,

    /// Coordinate transformation
    ///
    /// This should transform (x, y) from the game's coordinate space to (x, y) in the raster image.
    pub transform: MapTilesetTransform,

    /// The minimum Z value this layer should be used
    ///
    /// This value is ignored for the first (lowest) layer
    pub start_z: f64,

    /// Attribution
    pub attribution: MapAttribution,
}

/// The tileset transform
///
/// The transformed coordiante will be:
/// ```no-compile
/// (x, y) -> (x * scale[0] + translate[0], y * scale[1] + translate[1])
/// ```
#[derive(PartialEq, Default, Serialize, Deserialize, Debug, Clone)]
#[derive_wasm(feature = "wasm")]
#[serde(rename_all = "camelCase")]
pub struct MapTilesetTransform {
    /// The scale of the transformation
    pub scale: (f64, f64),
    /// The translation of the transformation
    pub translate: (f64, f64),
}

/// Attribution to display on the map
///
/// (displayed as &copy; LINK)
#[derive(PartialEq, Default, Serialize, Deserialize, Debug, Clone)]
#[derive_wasm(feature = "wasm")]
#[serde(rename_all = "camelCase")]
pub struct MapAttribution {
    /// Url of the attribution
    pub link: String,
    /// If the copyright sign should be displayed
    #[serde(default)]
    pub copyright: bool,
}

/// Axis of the map
#[derive(PartialEq, Default, Serialize, Deserialize, Debug, Clone)]
#[derive_wasm(feature = "wasm")]
#[serde(rename_all = "camelCase")]
pub enum Axis {
    /// Horizontal axis
    #[default]
    X,
    /// Vertical axis
    Y,
    /// Height axis
    Z,
    /// Negative Horizontal axis
    #[serde(rename = "-x")]
    NegX,
    /// Negative Vertical axis
    #[serde(rename = "-y")]
    NegY,
    /// Negative Height axis
    #[serde(rename = "-z")]
    NegZ,
}

/// Map features for one section
#[derive(PartialEq, Default, Serialize, Deserialize, Debug, Clone)]
#[derive_wasm(feature = "wasm")]
#[serde(rename_all = "camelCase")]
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
#[derive_wasm(feature = "wasm")]
#[serde(rename_all = "camelCase")]
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
#[derive_wasm(feature = "wasm")]
#[serde(rename_all = "camelCase")]
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
#[derive_wasm(feature = "wasm")]
#[serde(rename_all = "camelCase")]
pub struct MapLine {
    /// Color of the line
    pub color: String,
    /// Points on the line
    pub points: Vec<GameCoord>,
}

/// Coordinates representing a point (x, y, z) in the game
#[derive(PartialEq, Default, Serialize, Deserialize, Debug, Clone)]
#[derive_wasm(feature = "wasm")]
pub struct GameCoord(pub f64, pub f64, pub f64);
