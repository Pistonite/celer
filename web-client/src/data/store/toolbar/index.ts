//! Toolbar store slice
//!
//! The toolbar slice stores global application state that doesn't need to persisted.
//! These states are mostly toolbar states.

import { ReducerDeclWithPayload, ValuePayload, configureSlice, withPayload } from "data/store/util";

/// The toolbar slice state
export type ToolbarStore = {
    /// If the user is currently editing the layout
    isEditingLayout: boolean;
};

const initialState: ToolbarStore = {
    isEditingLayout: false,
};


/* reducers: TODO may need to refactor */
const setIsEditingLayout: ReducerDeclWithPayload<
    ToolbarStore, ValuePayload<boolean>
> = withPayload((state, { value }) => {
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