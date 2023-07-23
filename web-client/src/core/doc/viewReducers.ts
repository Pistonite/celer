//! Reducers for doc view state

import { ReducerDeclWithPayload, withPayload } from "low/store";

import { DocViewState, KeyBindingName } from "./state";

/// Set the current document location
export const setDocLocation: ReducerDeclWithPayload<
    DocViewState,
    {
        /// Section index
        section: number;
        /// Line index in the section
        line: number;
    }
> = withPayload((state: DocViewState, { section, line }) => {
    state.currentSection = section;
    state.currentLine = line;
});

/// Set the current editing keybinding
export const setEditingKeyBinding: ReducerDeclWithPayload<
    DocViewState,
    KeyBindingName | undefined
> = withPayload((state: DocViewState, editingKeyBinding: KeyBindingName | undefined) => {
    state.editingKeyBinding = editingKeyBinding;
});
