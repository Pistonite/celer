//! Layout view state reducers

import { withPayload } from "low/store";

import type { LayoutViewState } from "./state";

/// Set if the user is editing the layout
export const setIsEditingLayout = withPayload<LayoutViewState, boolean>(
    (state, value) => {
        state.isEditingLayout = value;
    },
);
