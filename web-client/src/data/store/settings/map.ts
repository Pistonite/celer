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
    lineSize: VisualSize.Small, // this is the same thickness as the old app
    arrowSize: VisualSize.Regular,
    arrowFrequency: VisualSize.Regular,
    markerSize: VisualSize.Regular,
};
