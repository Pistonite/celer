//! Toolbar store slice
//!
//! The toolbar slice stores global application state that doesn't need to persisted.
//! These states are mostly toolbar states.

import { ReducerDeclWithPayload, configureSlice, withPayload } from "data/store/util";

/// The toolbar slice state
export type ToolbarStore = {
    /// If the user is currently editing the layout
    isEditingLayout: boolean;
    /// If the user currently has the settings dialog open
    isSettingsOpen: boolean;
};

const initialState: ToolbarStore = {
    isEditingLayout: false,
    isSettingsOpen: false
};


/* reducers: TODO may need to refactor */
const setIsEditingLayout: ReducerDeclWithPayload<
    ToolbarStore, boolean
> = withPayload((state: ToolbarStore, value: boolean) => {
    state.isEditingLayout = value;
});

/// The toolbar store slice
export const {
    toolbarReducer,
    toolbarActions,
    toolbarSelector
} = configureSlice({
    name: "toolbar",
    initialState,
    reducers: {
        setIsEditingLayout
    }
});