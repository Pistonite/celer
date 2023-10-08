//! Reducers for editor settings
import { withPayload } from "low/store";

import { EditorSettingsState } from "./state";

/// Set if the file tree is shown
export const setShowFileTree = withPayload<EditorSettingsState, boolean>(
    (state, showFileTree) => {
        state.showFileTree = showFileTree;
    },
);

/// Set if auto load is enabled
export const setAutoLoadEnabled = withPayload<EditorSettingsState, boolean>(
    (state, autoLoadEnabled) => {
        state.autoLoadEnabled = autoLoadEnabled;
    },
);

/// Set if auto save is enabled
export const setAutoSaveEnabled = withPayload<EditorSettingsState, boolean>(
    (state, autoSaveEnabled) => {
        state.autoSaveEnabled = autoSaveEnabled;
    },
);

/// Set the time of inactivity after which auto load is disabled
export const setDeactivateAutoLoadAfterMinutes = withPayload<
    EditorSettingsState,
    number
>((state, deactivateAutoLoadAfterMinutes) => {
    state.deactivateAutoLoadAfterMinutes = deactivateAutoLoadAfterMinutes;
});
