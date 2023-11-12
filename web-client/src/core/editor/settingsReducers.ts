//! Reducers for editor settings
import { withPayload } from "low/store";

import { EditorMode, EditorSettingsState } from "./state";

export const setShowFileTree = withPayload<EditorSettingsState, boolean>(
    (state, showFileTree) => {
        state.showFileTree = showFileTree;
    },
);

export const setAutoSaveEnabled = withPayload<EditorSettingsState, boolean>(
    (state, autoSaveEnabled) => {
        state.autoSaveEnabled = autoSaveEnabled;
    },
);

export const setCompilerEntryPath = withPayload<EditorSettingsState, string>(
    (state, compilerEntryPath) => {
        state.compilerEntryPath = compilerEntryPath;
    },
);

export const setCompilerUseCachePack0 = withPayload<
    EditorSettingsState,
    boolean
>((state, compilerUseCachePack0) => {
    state.compilerUseCachePack0 = compilerUseCachePack0;
});

export const setEditorMode = withPayload<EditorSettingsState, EditorMode>(
    (state, editorMode) => {
        state.editorMode = editorMode;
    },
);
