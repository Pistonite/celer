//! Layout store reducers

import { ReducerDecl, ReducerDeclWithPayload, withPayload } from "data/store/util";
import { Layout, LayoutSettings, fitLayoutToGrid } from "./util";

/// Modify the current layout
export const setCurrentLayout: ReducerDeclWithPayload<LayoutSettings, {
    /// The new layout to set
    layout: Layout
}> = withPayload((state, { layout }) => {
    if (state.currentLayout >= 0 && state.currentLayout < state.savedLayouts.length) {
        state.savedLayouts[state.currentLayout] = fitLayoutToGrid(layout);
    }
});

/// Switch to a layout
export const switchLayout: ReducerDeclWithPayload<LayoutSettings, {
    /// The layout index to switch to
    index: number
}> = withPayload((state, { index }) => {
    state.currentLayout = index;
});

/// Add a layout and switch to it
export const addAndSwitchLayout: ReducerDeclWithPayload<LayoutSettings, {
    /// The layout to add
    layout: Layout
}> = withPayload((state, { layout }) => {
    state.savedLayouts.push(fitLayoutToGrid(layout));
    state.currentLayout = state.savedLayouts.length - 1;
});

/// Delete current layout and switch to default layout
export const deleteCurrentLayout: ReducerDecl<LayoutSettings> = (state) => {
    if (state.currentLayout >= 0 && state.currentLayout < state.savedLayouts.length) {
        state.savedLayouts.splice(state.currentLayout, 1);
    }
    state.currentLayout = -1;
};

