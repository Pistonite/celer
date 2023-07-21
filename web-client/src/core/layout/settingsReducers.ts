//! Layout store reducers

import {
    ReducerDecl,
    ReducerDeclWithPayload,
    withPayload,
} from "low/store";

import { LayoutSettingsState, Layout } from "./state";
import { fitLayoutToGrid } from "./utils";

/// Modify the current layout
export const setCurrentLayout: ReducerDeclWithPayload<LayoutSettingsState, Layout> =
    withPayload((state, layout) => {
        if (
            state.currentLayout >= 0 &&
            state.currentLayout < state.savedLayouts.length
        ) {
            state.savedLayouts[state.currentLayout] = fitLayoutToGrid(layout);
        }
    });

/// Switch to a layout
export const switchLayout: ReducerDeclWithPayload<LayoutSettingsState, number> =
    withPayload((state, index) => {
        state.currentLayout = index;
    });

/// Add a layout and switch to it
export const addAndSwitchLayout: ReducerDeclWithPayload<
    LayoutSettingsState,
    Layout
> = withPayload((state, layout) => {
    state.savedLayouts.push(fitLayoutToGrid(layout));
    state.currentLayout = state.savedLayouts.length - 1;
});

/// Delete current layout and switch to default layout
export const deleteCurrentLayout: ReducerDecl<LayoutSettingsState> = (state) => {
    if (
        state.currentLayout >= 0 &&
        state.currentLayout < state.savedLayouts.length
    ) {
        state.savedLayouts.splice(state.currentLayout, 1);
    }
    state.currentLayout = -1;
};
