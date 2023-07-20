//! View part of the view store
//!
//! This stores the current view state of the document viewer

import { KeyBindingName } from "data/store/settings";

export type DocViewStore = {
    /// Current section the user is on
    currentSection: number;
    /// Current line the user is on in the section
    currentLine: number;
    /// If the user is currently editing a key binding
    editingKeyBinding: KeyBindingName | undefined;
};

export const initialDocViewStore: DocViewStore = {
    currentSection: 0,
    currentLine: 0,
    editingKeyBinding: undefined,
};

