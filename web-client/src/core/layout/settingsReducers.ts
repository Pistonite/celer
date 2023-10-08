//! Layout store reducers
import { StageMode } from "core/stage";
import { ReducerDecl, withPayload } from "low/store";

import { LayoutSettingsState, Layout, WidgetType } from "./state";
import {
    fitLayoutToGrid,
    getDefaultLayout,
    isCurrentLayoutDefault,
} from "./utils";

/// Modify the current layout
export const setCurrentLayout = withPayload<LayoutSettingsState, Layout>(
    (state, layout) => {
        if (!isCurrentLayoutDefault(state)) {
            state.savedLayouts[state.currentLayout] = fitLayoutToGrid(layout);
        }
    },
);

/// Set the toolbar location of the current layout
export const setCurrentLayoutToolbarLocation = withPayload<
    LayoutSettingsState,
    WidgetType
>((state, location) => {
    if (!isCurrentLayoutDefault(state)) {
        state.savedLayouts[state.currentLayout].toolbar = location;
    }
});

/// Set the toolbar anchor location of the current layout
export const setCurrentLayoutToolbarAnchor = withPayload<
    LayoutSettingsState,
    "top" | "bottom"
>((state, location) => {
    if (!isCurrentLayoutDefault(state)) {
        state.savedLayouts[state.currentLayout].toolbarAnchor = location;
    }
});

/// Switch to a layout
export const switchLayout = withPayload<LayoutSettingsState, number>(
    (state, index) => {
        state.currentLayout = index;
    },
);

/// Duplicate the current layout and switch to it
///
/// If the current layout is the default layout, the actual
/// current layout will be duplicated and switched to.
export const duplicateLayout = withPayload<LayoutSettingsState, StageMode>(
    (state, mode) => {
        if (isCurrentLayoutDefault(state)) {
            const layout = getDefaultLayout(
                window.innerWidth,
                window.innerHeight,
                mode,
            );
            state.savedLayouts.push(layout);
        } else {
            state.savedLayouts.push(state.savedLayouts[state.currentLayout]);
        }
        state.currentLayout = state.savedLayouts.length - 1;
    },
);

/// Delete current layout and switch to default layout
export const deleteCurrentLayout: ReducerDecl<LayoutSettingsState> = (
    state,
) => {
    if (!isCurrentLayoutDefault(state)) {
        state.savedLayouts.splice(state.currentLayout, 1);
    }
    state.currentLayout = -1;
};
