//! Map state slice

import { GameCoord } from "low/celerc";

/// State type for map settings
export type MapSettingsState = {
    // section and layer modes
    lineSectionMode: SectionMode;
    lineLayerMode: LayerMode;
    fadeNonCurrentLayerLines: boolean;

    iconSectionMode: SectionMode;
    iconLayerMode: LayerMode;
    fadeNonCurrentLayerIcons: boolean;

    markerSectionMode: SectionMode;
    markerLayerMode: LayerMode;
    fadeNonCurrentLayerMarkers: boolean;

    // icon sizes
    primaryIconSize: VisualSize;
    secondaryIconSize: VisualSize;
    otherIconSize: VisualSize;
    // line and arrow sizes
    /// thickness
    lineSize: VisualSize;
    /// size of the arrow
    arrowSize: VisualSize;
    /// Frequency of the arrow. Larger means less frequent
    arrowFrequency: VisualSize;

    markerSize: VisualSize;
};

/// Enum for section-based map display
export enum SectionMode {
    /// Show all sections
    All = "all",
    /// Show all sections, but grey out non-current sections
    CurrentHighlight = "current-highlight",
    /// Only current section
    Current = "current",
    /// Hide everything
    None = "none",
}

/// Enum for how visuals on different layers are displayed
export enum LayerMode {
    /// Only show current layer
    CurrentOnly = "current",
    /// Show current and adjacent layers
    CurrentAndAdjacent = "adjacent",
    /// Show all layers
    All = "all",
}

/// Enum for visual size
///
/// Generic enum for setting size for visuals.
/// Depending on the context, it can mean different things.
/// For example, icon size, line thickness, etc.
export enum VisualSize {
    Hidden = 0,
    Small,
    Regular,
    Large,
}

/// initial map setting state
export const initialMapSettingsState: MapSettingsState = {
    lineSectionMode: SectionMode.All,
    lineLayerMode: LayerMode.CurrentAndAdjacent,
    fadeNonCurrentLayerLines: true,

    iconSectionMode: SectionMode.All,
    iconLayerMode: LayerMode.CurrentAndAdjacent,
    fadeNonCurrentLayerIcons: true,

    markerSectionMode: SectionMode.All,
    markerLayerMode: LayerMode.CurrentAndAdjacent,
    fadeNonCurrentLayerMarkers: true,

    primaryIconSize: VisualSize.Regular,
    secondaryIconSize: VisualSize.Regular,
    otherIconSize: VisualSize.Small,
    lineSize: VisualSize.Small, // this is the same thickness as the old app
    arrowSize: VisualSize.Regular,
    arrowFrequency: VisualSize.Regular,
    markerSize: VisualSize.Regular,
};

/// Map view state
export type MapViewState = {
    /// Current map layer the user is on
    currentMapLayer: number;
    /// Current map view
    ///
    /// The map listens to this value and updates the map view accordingly.
    /// If the value is an array, the map will fit so all points are visible.
    currentMapView: GameCoord[] | MapView;
    /// Min and max zoom
    ///
    /// This is usually tied to the layer. The map automatically updates this
    /// according to the current layer
    currentZoomBounds: [number, number];
};

/// Map view data
///
/// Center and zoom need to be updated together.
export type MapView = {
    center: GameCoord;
    zoom: number;
};

export const initialMapViewState: MapViewState = {
    currentMapView: { center: [0, 0, 0], zoom: 1 },
    currentMapLayer: 0,
    currentZoomBounds: [1, 1],
};
