//! map setting state reducers

import { withPayload } from "low/store";

import { LayerMode, MapSettingsState, SectionMode, VisualSize } from "./state";

/// Set the current line section mode
export const setLineSectionMode = withPayload<MapSettingsState, SectionMode>((state, value) => {
    state.lineSectionMode = value;
});

/// Set the current line layer mode
export const setLineLayerMode= withPayload< MapSettingsState, LayerMode >((state, value) => {
    state.lineLayerMode = value;
});

/// Set if lines not on current layer should fade
export const setFadeNonCurrentLayerLines= withPayload< MapSettingsState, boolean >((state, value) => {
    state.fadeNonCurrentLayerLines = value;
});

/// Set the current icon section mode
export const setIconSectionMode= withPayload< MapSettingsState, SectionMode >((state, value) => {
    state.iconSectionMode = value;
});

/// Set the current icon layer mode
export const setIconLayerMode= withPayload< MapSettingsState, LayerMode >((state, value) => {
    state.iconLayerMode = value;
});

/// Set if icons not on current layer should fade
export const setFadeNonCurrentLayerIcons= withPayload< MapSettingsState, boolean >((state, value) => {
    state.fadeNonCurrentLayerIcons = value;
});

/// Set the current marker section mode
export const setMarkerSectionMode= withPayload< MapSettingsState, SectionMode >((state, value) => {
    state.markerSectionMode = value;
});

/// Set the current marker layer mode
export const setMarkerLayerMode= withPayload< MapSettingsState, LayerMode >((state, value) => {
    state.markerLayerMode = value;
});

/// Set if markers not on current layer should fade
export const setFadeNonCurrentLayerMarkers= withPayload< MapSettingsState, boolean >((state, value) => {
    state.fadeNonCurrentLayerMarkers = value;
});

/// Set primary icon size
export const setPrimaryIconSize= withPayload< MapSettingsState, VisualSize >((state, value) => {
    state.primaryIconSize = value;
});

/// Set secondary icon size
export const setSecondaryIconSize= withPayload< MapSettingsState, VisualSize >((state, value) => {
    state.secondaryIconSize = value;
});

/// Set other icon size
export const setOtherIconSize= withPayload< MapSettingsState, VisualSize >((state, value) => {
    state.otherIconSize = value;
});

/// Set line size (thickness)
export const setLineSize= withPayload<MapSettingsState, VisualSize> ((state, value) => {
        state.lineSize = value;
    });

/// Set arrow size
export const setArrowSize= withPayload< MapSettingsState, VisualSize >((state, value) => {
    state.arrowSize = value;
});

/// Set arrow frequency
export const setArrowFrequency= withPayload< MapSettingsState, VisualSize >((state, value) => {
    state.arrowFrequency = value;
});

/// Set marker size
export const setMarkerSize= withPayload< MapSettingsState, VisualSize >((state, value) => {
    state.markerSize = value;
});
