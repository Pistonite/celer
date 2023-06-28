//! Settings store slice
//!
//! This is used for user settings, such as theme, layout, map settings, etc.
//! These settings need to be persisted to local storage.

import { ReducerDecl, configureSlice } from "data/store/util";
import { LayoutSettings, initialLayoutSettings, LayoutReducers } from "./layout";

/// Local storage key
const LOCAL_STORAGE_KEY = "Celer.Settings";

/// The settings slice state
export type SettingsStore = LayoutSettings;

/// Try loading initial state from local storage
const loadState = (): SettingsStore => {
    const state = localStorage.getItem(LOCAL_STORAGE_KEY);
    const loadedState = state ? JSON.parse(state) : {};
    return {
        ...initialLayoutSettings,
        ...loadedState,
    };
};

/// TODO remove this
const setCurrentViewingLayoutTest: ReducerDecl<SettingsStore> = (state) => {
    console.log(state);
};

/// The setting state slice
export const {
    settingsReducer,
    settingsActions,
    settingsSelector
} = configureSlice({
    name: "settings",
    initialState: loadState(),
    reducers: {
        ...LayoutReducers,
        setCurrentViewingLayoutTest
    }
});

// re-exports
export * from "./layout/defaults";
export * from "./layout/util";