//! Map types
use serde::Serialize;
use ts_rs::TS;

/// Metadata of the map
///
/// This includes configuration like map layers, coordinates, etc.
#[derive(Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct MapMetadata {
    /// The map layers. First is the lowest layer.
    pub layers: Vec<MapLayerAttr>,
    /// Mapping for the coordinates in the route.
    pub coord_map: MapCoordMap,
    /// Initial coordinates
    pub initial_coord: GameCoord,
    /// Initial zoom level
    pub initial_zoom: u32,
}

/// The mapping if 2 coordinates are specified in the route
///
/// For example, ["x", "z"] will map the coordinates 
/// to the x (horizontal) and z (height) axis of the map.
///
/// Default value of 0 will be assigned to the unmapped axis.
#[derive(Serialize, TS)]
#[ts(export)]
pub struct MapCoordMap {
    /// Mapping for 2d coordinates in the route.
    #[serde(rename = "2d")]
    pub mapping_2d: (Axis, Axis),
    // Mapping for 3d coordinates in the route.
    #[serde(rename = "3d")]
    pub mapping_3d: (Axis, Axis, Axis),
}

/// Attribute (definition) of a map layer in the route
#[derive(Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
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
    pub size: (u32, u32),
    /// Min and max zoom levels
    pub zoom_bounds: (u32, u32),
    /// Max native zoom of the tileset
    pub max_native_zoom: u32,
    /// Coordinate transformation
    ///
    /// This should transform (x, y) from the game's coordinate space to (x, y) in the raster image.
    pub transform: MapTilesetTransform,
    /// The minimum Z value this layer should be used
    ///
    /// This value is ignored for the first (lowest) layer
    pub start_z: f32,
    /// Attribution 
    pub attribution: MapAttribution,
}

/// The tileset transform
///
/// The transformed coordiante will be:
/// ```no-compile
/// (x, y) -> (x * scale[0] + translate[0], y * scale[1] + translate[1])
/// ```
#[derive(Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct MapTilesetTransform {
    /// The scale of the transformation
    pub scale: (f32, f32),
    /// The translation of the transformation
    pub translate: (f32, f32),
}

/// Attribution to display on the map
///
/// (displayed as &copy; LINK)
#[derive(Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct MapAttribution {
    /// Url of the attribution
    pub link: String,
    /// If the copyright sign should be displayed
    pub copyright: bool,
}

/// Axis of the map
#[derive(Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub enum Axis {
    /// Horizontal axis
    X,
    /// Vertical axis
    Y,
    /// Height axis
    Z,
}

/// Icon on the map
#[derive(Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct MapIcon {
    /// Internal icon name (usually kebab-case)
    pub id: String,
    /// Game coordinate for the icon
    pub coord: GameCoord,
    /// The corresponding line index in section of the document
    pub line_index: u32,
    /// The corresponding section number in the document
    pub section_index: u32,
    /// The priority of the icon (0 = primary, 1 = secondary)
    pub priority: u32,
}

/// Markers on the map
#[derive(Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct MapMarker {
    pub coord: GameCoord,
    /// The corresponding line index in section of the document
    pub line_index: u32,
    /// The corresponding section number in the document
    pub section_index: u32,
    /// Color of the marker
    pub color: String,
}

/// Paths on the map
///
/// The coordinates do not have to be on the same map layer.
/// The map will automatically split the path if it croses map layers.
#[derive(Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct MapLine {
    /// Color of the line
    pub color: String,
    /// Points on the line
    pub points: Vec<GameCoord>,
}

/// Coordinates representing a point (x, y, z) in the game
#[derive(Serialize, TS)]
#[ts(export)]
pub struct GameCoord(f32, f32, f32);
