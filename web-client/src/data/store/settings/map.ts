//! map setting state

/// State type for map settings
export type MapSettings = {
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

    primaryIconSize: VisualSize;
    secondaryIconSize: VisualSize;
    otherIconSize: VisualSize;
    lineSize: VisualSize;
    markerSize: VisualSize;
};

/// Enum for section-based map display
export enum SectionMode {
    /// Show all sections
    All = "all",
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
export enum VisualSize {
    Hidden = 0 ,
    Small,
    Regular,
    Large,
}

/// initial map setting state
export const initialMapSettings: MapSettings = {
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
    otherIconSize: VisualSize.Regular,
    lineSize: VisualSize.Regular,
    markerSize: VisualSize.Regular,
};
