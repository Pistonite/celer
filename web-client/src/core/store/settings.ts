//! Settings store slice
//!
//! This is used for user settings, such as theme, layout, map settings, etc.
//! These settings need to be persisted to local storage.

import { DocSettingsState, docSettingsReducers, initialDocSettingsState } from "core/doc";
import { LayoutSettingsState, initialLayoutSettingsState, layoutSettingsReducers } from "core/layout";
import { MapSettingsState, initialMapSettingsState, mapSettingsReducers } from "core/map";
import { configureSlice } from "low/store";

/// Local storage key
const LOCAL_STORAGE_KEY = "Celer.Settings";

/// The settings slice state
export type SettingsState = LayoutSettingsState & MapSettingsState & DocSettingsState;

/// Try loading initial state from local storage on store init
const loadState = (): SettingsState => {
    const state = localStorage.getItem(LOCAL_STORAGE_KEY);
    const loadedState = state ? JSON.parse(state) : {};
    return {
        ...initialLayoutSettingsState,
        ...initialMapSettingsState,
        ...initialDocSettingsState,
        ...loadedState,
    };
};

/// The setting state slice
export const { settingsReducer, settingsActions, settingsSelector } =
    configureSlice({
        name: "settings",
        initialState: loadState(),
        reducers: {
            ...layoutSettingsReducers,
            ...mapSettingsReducers,
            ...docSettingsReducers,
        },
    });
