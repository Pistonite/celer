//! Utility types

/// Document icon map
///
/// This is a map of internal icon names to icon data.
/// Icon data should be URL to the icon. (Can be data URL)
export type DocIconMap = {
    /// internal icon name (usually kebab-case)
    [id: string]: string;
};

/// The metadata of the document
///
/// Usually part of document.project
export type DocMetadata = {
    /// Name of the project (usually kebab-case)
    name: string;
    /// Title/Display name of the project.
    ///
    /// This will be displayed in the title bar
    title: string;
    /// The author(s) of the project.
    authors: string[];
    /// The version of the project.
    version: string;
    /// The url of the project. (e.g. https://github.com/username/project)
    url: string;
};

/// Parameter for the map
export type DocMapParameters = {
    /// The map layers. First is the lowest layer.
    layers: DocMapLayer[];
    /// Mapping for the coordinates in the route.
    coordMap: DocMapCoordMap;
    /// Initial coordinates
    initialCoord: GameCoord;
    /// Initial zoom level
    initialZoom: number;
};

/// The mapping if 2 coordinates are specified in the route
///
/// For example, ["x", "z"] will map the coordinates to the x (horizontal) and z (height) axis of the map.
export type DocMapCoordMap = {
    "2d": [Axis, Axis];
    "3d": [Axis, Axis, Axis];
};

export type DocMapLayer = {
    /// Display name of the layer
    ///
    /// This is visible in the layer switch UI
    name: string;
    /// The tileset url template, with {x} {y} {z} as placeholders.
    ///
    /// The url should conform to the leaflet tile layer API: https://leafletjs.com/reference.html#tilelayer
    templateUrl: string;
    /// The raster coordinate size
    ///
    /// See: https://github.com/commenthol/leaflet-rastercoords.
    /// Form is [width, height]
    size: [number, number];
    /// Min and max zoom levels
    zoomBounds: [number, number];
    /// Max native zoom of the tileset
    maxNativeZoom: number;
    /// Coordinate transformation
    ///
    /// This should transform (x, y) from the game's coordinate space to (x, y) in the raster image.
    transform: DocMapLayerTilesetTransform;
    /// The minimum Z value this layer should be used
    ///
    /// This value is ignored for the first (lowest) layer
    startZ: number;
    /// Attribution (displayed as &copy; LINK)
    attribution: DocMapLayerAttribution;
};

export type DocMapLayerAttribution = {
    /// Url of the attribution
    link: string;
    /// If the copyright sign should be displayed
    copyright: boolean;
};

/// The tileset transform
///
/// The transformed coordiante will be (x, y) -> (x * scale[0] + translate[0], y * scale[1] + translate[1])
export type DocMapLayerTilesetTransform = {
    /// The scale of the transformation
    scale: [number, number];
    /// The translation of the transformation
    translate: [number, number];
};

export type Axis = "x" | "y" | "z";

/// Game coordinate
///
/// This is usually the in game coordinate.
/// This is not the same coordinates written in the route,
/// but the coordinates (x, y, z) after mapped to the game coordinate system
/// using the route's coordMap.
export type GameCoord = [number, number, number];

/// Route coordinate
///
/// This is the coordinate in the document, before mapping to GameCoord.
/// NOTE: the last element is only there so that TypeScript treats this as a different type from GameCoord.
export type RouteCoord = [number, number, number | undefined, undefined];

/// Map coordinate
///
/// This is the (x, y) coordinate on the map, corresponding to the map coordinate system.
/// This is only used internally in the map
export type MapCoord = [number, number];

/// Icon on the map
export type MapIcon = {
    /// Internal icon name (usually kebab-case)
    id: string;
    /// Game coordinate for the icon
    coord: GameCoord;
    /// The corresponding line index in section of the document
    lineIndex: number;
    /// The corresponding section number in the document
    sectionIndex: number;
    /// The priority of the icon (0 = primary, 1 = secondary)
    priority: number;
};

/// Markers on the map
export type MapMarker = {
    /// Coordinate of the marker
    coord: GameCoord;
    /// The corresponding line index in section of the document
    lineIndex: number;
    /// The corresponding section number in the document
    sectionIndex: number;
    /// Color of the marker
    color: string;
};

/// Paths on the map
///
/// The coordinates do not have to be on the same map layer.
/// The map will automatically split the path if it croses map layers.
export type MapLine = {
    /// Color of the line
    color: string;
    /// Points on the line
    points: GameCoord[];
};

export type DocNote = DocNoteText | DocNoteImage | DocNoteVideo;

/// Text as document note
export type DocNoteText = {
    type: "text";
    content: DocRichText[];
}

/// Image as document note
export type DocNoteImage = {
    type: "image";
    /// Image URL
    src: string;
}

/// Embedded video as document note
export type DocNoteVideo = {
    type: "video";
    /// Video URL of supported providers
    link: string;
}

export type DocDiagnostic = {
    /// The diagnostic message
    message: string;
    /// Type of the diagnostic
    type: "warn" | "error";
}

/// Document rich text type
export type DocRichText = {
    /// The tag name of the text
    ///
    /// Each block only contains one tag
    tag?: string;
    /// The text content
    text: string;
};

/// Rich text type with resolved tag
export type RichText = {
    /// The tag of the text
    tag?: DocTag;
    /// The text content
    text: string;
}

/// Document tag map
export type DocTagMap = {
    /// The tag name (usually kebab-case)
    [id: string]: DocTag;
};

/// Document tag type
///
/// Used to style text and provide extra function to the engine
export type DocTag = {
    /// Bold style
    bold?: boolean;
    /// Italic style
    italic?: boolean;
    /// Underline style
    underline?: boolean;
    /// Strikethrough style
    strikethrough?: boolean;
    /// Color of the text
    color?: string;
    /// Background color of the text
    background?: string;
    /// Link to direct to when clicked
    link?: string;
};

/// Document counter style
export type DocCounter = {
    /// Counter background color
    background: string;
    /// Counter text color
    color: string;
};
