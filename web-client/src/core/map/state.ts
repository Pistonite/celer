//! Map state slice
import { z } from "zod";

import type { GameCoord } from "low/celerc";

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
export const SectionModeSchema = z.nativeEnum(SectionMode);

/// Enum for how visuals on different layers are displayed
export enum LayerMode {
    /// Only show current layer
    CurrentOnly = "current",
    /// Show current and adjacent layers
    CurrentAndAdjacent = "adjacent",
    /// Show all layers
    All = "all",
}
export const LayerModeSchema = z.nativeEnum(LayerMode);

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
export const VisualSizeSchema = z.nativeEnum(VisualSize);

/// State type for map settings
export const MapSettingsStateSchema = z.object({
    // section and layer modes
    lineSectionMode: SectionModeSchema,
    lineLayerMode: LayerModeSchema,
    fadeNonCurrentLayerLines: z.boolean(),

    iconSectionMode: SectionModeSchema,
    iconLayerMode: LayerModeSchema,
    fadeNonCurrentLayerIcons: z.boolean(),

    markerSectionMode: SectionModeSchema,
    markerLayerMode: LayerModeSchema,
    fadeNonCurrentLayerMarkers: z.boolean(),

    // icon sizes
    primaryIconSize: VisualSizeSchema,
    secondaryIconSize: VisualSizeSchema,
    otherIconSize: VisualSizeSchema,
    // line and arrow sizes
    /// thickness
    lineSize: VisualSizeSchema,
    /// size of the arrow
    arrowSize: VisualSizeSchema,
    /// Frequency of the arrow. Larger means less frequent
    arrowFrequency: VisualSizeSchema,

    markerSize: VisualSizeSchema,
});
export type MapSettingsState = z.infer<typeof MapSettingsStateSchema>;

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
