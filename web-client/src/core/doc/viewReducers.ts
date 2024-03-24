//! Reducers for doc view state

import { withPayload } from "low/store";

import { DocViewState, KeyBindingName } from "./state";

/// Set the current document location
export const setDocLocation = withPayload<
    DocViewState,
    {
        /// Section index
        section: number;
        /// Line index in the section
        line: number;
    }
>((state, { section, line }) => {
    state.currentSection = section;
    state.currentLine = line;
});

/// Set the current editing keybinding
export const setEditingKeyBinding = withPayload<
    DocViewState,
    KeyBindingName | undefined
>((state, editingKeyBinding) => {
    state.editingKeyBinding = editingKeyBinding;
});

export const setSuppressRecompile = withPayload<DocViewState, boolean>(
    (state, suppressRecompile) => {
        state.suppressRecompile = suppressRecompile;
    },
);
