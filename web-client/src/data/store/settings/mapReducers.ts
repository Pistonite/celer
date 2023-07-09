//! map setting state reducers

import { ReducerDeclWithPayload, withPayload } from "data/store/util";

import { LayerMode, MapSettings, SectionMode, VisualSize } from "./map";

/// Set the current line section mode
export const setLineSectionMode: ReducerDeclWithPayload<
    MapSettings,
    SectionMode
> = withPayload((state: MapSettings, value: SectionMode) => {
    state.lineSectionMode = value;
});

/// Set the current line layer mode
export const setLineLayerMode: ReducerDeclWithPayload<MapSettings, LayerMode> =
    withPayload((state: MapSettings, value: LayerMode) => {
        state.lineLayerMode = value;
    });

/// Set if lines not on current layer should fade
export const setFadeNonCurrentLayerLines: ReducerDeclWithPayload<
    MapSettings,
    boolean
> = withPayload((state: MapSettings, value: boolean) => {
    state.fadeNonCurrentLayerLines = value;
});

/// Set the current icon section mode
export const setIconSectionMode: ReducerDeclWithPayload<
    MapSettings,
    SectionMode
> = withPayload((state: MapSettings, value: SectionMode) => {
    state.iconSectionMode = value;
});

/// Set the current icon layer mode
export const setIconLayerMode: ReducerDeclWithPayload<MapSettings, LayerMode> =
    withPayload((state: MapSettings, value: LayerMode) => {
        state.iconLayerMode = value;
    });

/// Set if icons not on current layer should fade
export const setFadeNonCurrentLayerIcons: ReducerDeclWithPayload<
    MapSettings,
    boolean
> = withPayload((state: MapSettings, value: boolean) => {
    state.fadeNonCurrentLayerIcons = value;
});

/// Set the current marker section mode
export const setMarkerSectionMode: ReducerDeclWithPayload<
    MapSettings,
    SectionMode
> = withPayload((state: MapSettings, value: SectionMode) => {
    state.markerSectionMode = value;
});

/// Set the current marker layer mode
export const setMarkerLayerMode: ReducerDeclWithPayload<
    MapSettings,
    LayerMode
> = withPayload((state: MapSettings, value: LayerMode) => {
    state.markerLayerMode = value;
});

/// Set if markers not on current layer should fade
export const setFadeNonCurrentLayerMarkers: ReducerDeclWithPayload<
    MapSettings,
    boolean
> = withPayload((state: MapSettings, value: boolean) => {
    state.fadeNonCurrentLayerMarkers = value;
});

/// Set primary icon size
export const setPrimaryIconSize: ReducerDeclWithPayload<
    MapSettings,
    VisualSize
> = withPayload((state: MapSettings, value: VisualSize) => {
    state.primaryIconSize = value;
});

/// Set secondary icon size
export const setSecondaryIconSize: ReducerDeclWithPayload<
    MapSettings,
    VisualSize
> = withPayload((state: MapSettings, value: VisualSize) => {
    state.secondaryIconSize = value;
});

/// Set other icon size
export const setOtherIconSize: ReducerDeclWithPayload<MapSettings, VisualSize> =
    withPayload((state: MapSettings, value: VisualSize) => {
        state.otherIconSize = value;
    });

/// Set line size (thickness)
export const setLineSize: ReducerDeclWithPayload<MapSettings, VisualSize> =
    withPayload((state: MapSettings, value: VisualSize) => {
        state.lineSize = value;
    });

/// Set arrow size
export const setArrowSize: ReducerDeclWithPayload<MapSettings, VisualSize> =
    withPayload((state: MapSettings, value: VisualSize) => {
        state.arrowSize = value;
    });

/// Set arrow frequency
export const setArrowFrequency: ReducerDeclWithPayload<
    MapSettings,
    VisualSize
> = withPayload((state: MapSettings, value: VisualSize) => {
    state.arrowFrequency = value;
});

/// Set marker size
export const setMarkerSize: ReducerDeclWithPayload<MapSettings, VisualSize> =
    withPayload((state: MapSettings, value: VisualSize) => {
        state.markerSize = value;
    });
