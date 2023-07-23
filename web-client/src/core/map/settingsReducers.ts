//! map setting state reducers

import { ReducerDeclWithPayload, withPayload } from "low/store";

import { LayerMode, MapSettingsState, SectionMode, VisualSize } from "./state";

/// Set the current line section mode
export const setLineSectionMode: ReducerDeclWithPayload<
    MapSettingsState,
    SectionMode
> = withPayload((state: MapSettingsState, value: SectionMode) => {
    state.lineSectionMode = value;
});

/// Set the current line layer mode
export const setLineLayerMode: ReducerDeclWithPayload<MapSettingsState, LayerMode> =
    withPayload((state: MapSettingsState, value: LayerMode) => {
        state.lineLayerMode = value;
    });

/// Set if lines not on current layer should fade
export const setFadeNonCurrentLayerLines: ReducerDeclWithPayload<
    MapSettingsState,
    boolean
> = withPayload((state: MapSettingsState, value: boolean) => {
    state.fadeNonCurrentLayerLines = value;
});

/// Set the current icon section mode
export const setIconSectionMode: ReducerDeclWithPayload<
    MapSettingsState,
    SectionMode
> = withPayload((state: MapSettingsState, value: SectionMode) => {
    state.iconSectionMode = value;
});

/// Set the current icon layer mode
export const setIconLayerMode: ReducerDeclWithPayload<MapSettingsState, LayerMode> =
    withPayload((state: MapSettingsState, value: LayerMode) => {
        state.iconLayerMode = value;
    });

/// Set if icons not on current layer should fade
export const setFadeNonCurrentLayerIcons: ReducerDeclWithPayload<
    MapSettingsState,
    boolean
> = withPayload((state: MapSettingsState, value: boolean) => {
    state.fadeNonCurrentLayerIcons = value;
});

/// Set the current marker section mode
export const setMarkerSectionMode: ReducerDeclWithPayload<
    MapSettingsState,
    SectionMode
> = withPayload((state: MapSettingsState, value: SectionMode) => {
    state.markerSectionMode = value;
});

/// Set the current marker layer mode
export const setMarkerLayerMode: ReducerDeclWithPayload<
    MapSettingsState,
    LayerMode
> = withPayload((state: MapSettingsState, value: LayerMode) => {
    state.markerLayerMode = value;
});

/// Set if markers not on current layer should fade
export const setFadeNonCurrentLayerMarkers: ReducerDeclWithPayload<
    MapSettingsState,
    boolean
> = withPayload((state: MapSettingsState, value: boolean) => {
    state.fadeNonCurrentLayerMarkers = value;
});

/// Set primary icon size
export const setPrimaryIconSize: ReducerDeclWithPayload<
    MapSettingsState,
    VisualSize
> = withPayload((state: MapSettingsState, value: VisualSize) => {
    state.primaryIconSize = value;
});

/// Set secondary icon size
export const setSecondaryIconSize: ReducerDeclWithPayload<
    MapSettingsState,
    VisualSize
> = withPayload((state: MapSettingsState, value: VisualSize) => {
    state.secondaryIconSize = value;
});

/// Set other icon size
export const setOtherIconSize: ReducerDeclWithPayload<MapSettingsState, VisualSize> =
    withPayload((state: MapSettingsState, value: VisualSize) => {
        state.otherIconSize = value;
    });

/// Set line size (thickness)
export const setLineSize: ReducerDeclWithPayload<MapSettingsState, VisualSize> =
    withPayload((state: MapSettingsState, value: VisualSize) => {
        state.lineSize = value;
    });

/// Set arrow size
export const setArrowSize: ReducerDeclWithPayload<MapSettingsState, VisualSize> =
    withPayload((state: MapSettingsState, value: VisualSize) => {
        state.arrowSize = value;
    });

/// Set arrow frequency
export const setArrowFrequency: ReducerDeclWithPayload<
    MapSettingsState,
    VisualSize
> = withPayload((state: MapSettingsState, value: VisualSize) => {
    state.arrowFrequency = value;
});

/// Set marker size
export const setMarkerSize: ReducerDeclWithPayload<MapSettingsState, VisualSize> =
    withPayload((state: MapSettingsState, value: VisualSize) => {
        state.markerSize = value;
    });
