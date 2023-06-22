//! Utility types

/// Document icon map
///
/// This is a map of internal icon names to icon data.
/// Icon data should be URL to the icon. (Can be data URL)
export type DocumentIconMap = {
    /// internal icon name (usually kebab-case)
    [id: string]: string;
}

/// The metadata of the document
///
/// Usually part of document.project
export type DocumentMetadata = {
    /// Name of the project (usually kebab-case)
    name: string,
    /// Title/Display name of the project.
    ///
    /// This will be displayed in the title bar
    title: string,
    /// The author(s) of the project.
    authors: string[],
    /// The version of the project.
    version: string,
    /// The url of the project. (e.g. https://github.com/username/project)
    url: string,
}



/// Parameter for the map
export type DocumentMapParameters = {
    /// The map layers. First is the lowest layer.
    layers: DocumentMapLayer[]
    /// Mapping for the coordinates in the route.
    coordMap: {
        /// The mapping if 2 coordinates are specified in the route
        ///
        /// For example, ["x", "z"] will map the coordinates to the x (horizontal) and z (height) axis of the map.
        _2d: [Axis, Axis],
        _3d: [Axis, Axis, Axis],
    },
    /// Min and max zoom levels
    zoomBounds: [number, number]
    /// Attribution
    attribution: {
        /// Url of the attribution
        link: string
        /// Text of the attribution
        text: string
        /// If the copyright sign should be displayed
        copyRight: boolean
    }
}

export type DocumentMapLayer = {
    /// Display name of the layer
    ///
    /// This is visible in the layer switch UI
    name: string,
    /// The tileset url template, with {x} {y} {z} as placeholders.
    ///
    /// The url should conform to the leaflet tile layer API: https://leafletjs.com/reference.html#tilelayer
    templateUrl: string
    /// The raster coordinate size
    ///
    /// See: https://github.com/commenthol/leaflet-rastercoords.
    /// Form is [width, height]
    size: [number, number]
    /// Max native zoom of the tileset
    maxNativeZoom: number
    /// Coordinate transformation
    ///
    /// This should transform (x, y) from the game's coordinate space to (x, y) in the raster image.
    transform: DocumentMapLayerTilesetTransform
    /// The minimum Z value this layer should be used
    ///
    /// This value is ignored for the first (lowest) layer
    startZ: number,
}

/// The tileset transform
///
/// The transformed coordiante will be (x, y) -> (x * scale[0] + translate[0], y * scale[1] + translate[1])
export type DocumentMapLayerTilesetTransform = {
    /// The scale of the transformation
    scale: [number, number]
    /// The translation of the transformation
    translate: [number, number]
}

export type Axis = "x" | "y" | "z";


/// Game coordinate
///
/// This is usually the in game coordinate.
/// This is not the same coordinates written in the route,
/// but the coordinates (x, y, z) after mapped to the game coordinate system
/// using the route's coordMap.
export type GameCoord = [number, number, number];

/// Map coordinate
///
/// This is the (x, y) coordinate on the map, corresponding to the map coordinate system.
export type MapCoord = [number, number];

/// Icon on the map
export type MapIcon = {
    /// Internal icon name (usually kebab-case)
    id: string,
    /// Game coordinate for the icon
    coord: GameCoord,
    /// Opacity of the icon (0 = invisible, 1 = fully visible)
    opacity: number,
}

