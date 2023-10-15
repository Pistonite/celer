//! Reducers for editor settings
import { withPayload } from "low/store";

import { EditorSettingsState } from "./state";

export const setShowFileTree = withPayload<EditorSettingsState, boolean>(
    (state, showFileTree) => {
        state.showFileTree = showFileTree;
    },
);

export const setAutoLoadEnabled = withPayload<EditorSettingsState, boolean>(
    (state, autoLoadEnabled) => {
        state.autoLoadEnabled = autoLoadEnabled;
    },
);

export const setAutoSaveEnabled = withPayload<EditorSettingsState, boolean>(
    (state, autoSaveEnabled) => {
        state.autoSaveEnabled = autoSaveEnabled;
    },
);

export const setDeactivateAutoLoadAfterMinutes = withPayload<
    EditorSettingsState,
    number
>((state, deactivateAutoLoadAfterMinutes) => {
    state.deactivateAutoLoadAfterMinutes = deactivateAutoLoadAfterMinutes;
});

export const setCompilerEntryPath = withPayload<
    EditorSettingsState,
    string
>((state, compilerEntryPath) => {
    state.compilerEntryPath = compilerEntryPath;
});
