//! Reducers for editor settings
import { ReducerDeclWithPayload, withPayload } from "low/store";

import { EditorSettingsState } from "./state";

/// Set if the file tree is shown
export const setShowFileTree: ReducerDeclWithPayload<EditorSettingsState, boolean> =
    withPayload((state: EditorSettingsState, showFileTree: boolean) => {
        state.showFileTree = showFileTree;
    });

/// Set if auto load is enabled
export const setAutoLoadEnabled: ReducerDeclWithPayload<EditorSettingsState, boolean> =
    withPayload((state: EditorSettingsState, autoLoadEnabled: boolean) => {
        state.autoLoadEnabled = autoLoadEnabled;
    });

/// Set if auto save is enabled
export const setAutoSaveEnabled: ReducerDeclWithPayload<EditorSettingsState, boolean> =
    withPayload((state: EditorSettingsState, autoSaveEnabled: boolean) => {
        state.autoSaveEnabled = autoSaveEnabled;
    });

/// Set the time of inactivity after which auto load is disabled
export const setDeactivateAutoLoadAfterMinutes: ReducerDeclWithPayload<EditorSettingsState, number> =
    withPayload((state: EditorSettingsState, deactivateAutoLoadAfterMinutes: number) => {
        state.deactivateAutoLoadAfterMinutes = deactivateAutoLoadAfterMinutes;
    });
