//! Reducers for doc view state

import { ReducerDeclWithPayload, withPayload } from "data/store/util";
import { KeyBindingName } from "data/store/settings";

import { DocViewStore } from "./doc";

/// Set the current document location
export const setDocLocation: ReducerDeclWithPayload<
    DocViewStore,
    {
        /// Section index
        section: number;
        /// Line index in the section
        line: number;
    }
> = withPayload((state: DocViewStore, { section, line }) => {
    state.currentSection = section;
    state.currentLine = line;
});

/// Set the current editing keybinding
export const setEditingKeyBinding: ReducerDeclWithPayload<
    DocViewStore,
    KeyBindingName | undefined
> = withPayload((state: DocViewStore, editingKeyBinding: KeyBindingName | undefined) => {
    state.editingKeyBinding = editingKeyBinding;
});
