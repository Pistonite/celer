//! Settings store slice
//!
//! This is used for user settings, such as theme, layout, map settings, etc.
//! These settings need to be persisted to local storage.

import { configureSlice } from "data/store/util";

import { LayoutSettings, initialLayoutSettings } from "./layout";
import * as layoutReducers from "./layoutReducers";
import { MapSettings, initialMapSettings } from "./map";
import * as mapReducers from "./mapReducers";

/// Local storage key
const LOCAL_STORAGE_KEY = "Celer.Settings";

/// The settings slice state
export type SettingsStore = LayoutSettings & MapSettings;

/// Try loading initial state from local storage
const loadState = (): SettingsStore => {
    const state = localStorage.getItem(LOCAL_STORAGE_KEY);
    const loadedState = state ? JSON.parse(state) : {};
    return {
        ...initialLayoutSettings,
        ...initialMapSettings,
        ...loadedState,
    };
};

/// The setting state slice
export const { settingsReducer, settingsActions, settingsSelector } =
    configureSlice({
        name: "settings",
        initialState: loadState(),
        reducers: {
            ...layoutReducers,
            ...mapReducers,
        },
    });

/// re-exports
export * from "./layoutUtil";
export * from "./map";
