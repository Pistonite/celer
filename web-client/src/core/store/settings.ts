//! Settings store slice
//!
//! This is used for user settings, such as theme, layout, map settings, etc.
//! These settings need to be persisted to local storage.

import type { DocSettingsState } from "core/doc";
import {
    DocSettingsStateSchema,
    docSettingsReducers,
    initialDocSettingsState,
} from "core/doc";
import type { LayoutSettingsState } from "core/layout";
import {
    initialLayoutSettingsState,
    layoutSettingsReducers,
} from "core/layout";
import type { MapSettingsState } from "core/map";
import {
    MapSettingsStateSchema,
    initialMapSettingsState,
    mapSettingsReducers,
} from "core/map";

import { configureSlice } from "low/store";
import { consoleKernel as console } from "low/utils";

import type { EditorSettingsState } from "./editor";
import { editorSettingsReducers, initialEditorSettingsState } from "./editor";

/// Local storage key
const LOCAL_STORAGE_KEY = "Celer.Settings";

/// The settings slice state
export type SettingsState = LayoutSettingsState &
    MapSettingsState &
    DocSettingsState &
    EditorSettingsState;

const SettingsStateSchema = DocSettingsStateSchema.merge(
    MapSettingsStateSchema,
).passthrough();

/// Try loading initial state from local storage on store init
const loadState = (): SettingsState => {
    const state = localStorage.getItem(LOCAL_STORAGE_KEY);
    const loadedState = state ? JSON.parse(state) : {};
    const initialState = getInitialState();
    const assignedState = Object.assign(getInitialState(), loadedState);
    const result = SettingsStateSchema.safeParse({
        ...initialState,
        ...assignedState,
    });
    if (result.success) {
        return result.data as SettingsState;
    }
    console.error("Failed to load settings from local storage");
    console.error(result.error);
    return initialState;
};

export const saveSettings = (state: SettingsState) => {
    localStorage.setItem(LOCAL_STORAGE_KEY, JSON.stringify(state));
};

const getInitialState = (): SettingsState => {
    return {
        ...initialLayoutSettingsState,
        ...initialMapSettingsState,
        ...initialDocSettingsState,
        ...initialEditorSettingsState,
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
            ...editorSettingsReducers,
            resetAllSettings: () => getInitialState(),
        },
    });
