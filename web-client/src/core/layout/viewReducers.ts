//! Layout view state reducers

import { ReducerDeclWithPayload, withPayload } from "low/store";

import { LayoutViewState } from "./state";

/// Set if the user is editing the layout
export const setIsEditingLayout: ReducerDeclWithPayload<
    LayoutViewState,
    boolean
> = withPayload((state: LayoutViewState, value: boolean) => {
    state.isEditingLayout = value;
});
