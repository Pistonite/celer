//! Layout store reducers

import { ReducerDeclWithPayload, withPayload } from "data/store/util";
import { Layout, LayoutSettings } from "./types";

type IsEditorPayload = {
    /// If the app is in editor mode
    isEditor: boolean
}

/// Modify the current layout
export const setCurrentLayout: ReducerDeclWithPayload<LayoutSettings, IsEditorPayload & {
    /// The layout to set
    layout: Layout
}> = withPayload((state, { isEditor, layout }) => {
    if (isEditor) {
        if (state.currentEditorLayout > 0 && state.currentEditorLayout <= state.savedEditorLayouts.length) {
            state.savedEditorLayouts[state.currentEditorLayout - 1] = layout;
        }
    } else {
        if (state.currentViewerLayout > 0 && state.currentViewerLayout <= state.savedViewerLayouts.length) {
            state.savedViewerLayouts[state.currentViewerLayout - 1] = layout;
        }
    }
});

/// Switch to a layout
export const switchLayout: ReducerDeclWithPayload<LayoutSettings, IsEditorPayload & {
    /// The layout index to switch to
    index: number
}> = withPayload((state, { isEditor, index }) => {
    if (isEditor) {
        if (index >= 0 && index <= state.savedEditorLayouts.length) {
            state.currentEditorLayout = index;
        }
    } else {
        if (index >= 0 && index <= state.savedViewerLayouts.length) {
            state.currentViewerLayout = index;
        }
    }
});

/// Add a layout and switch to it
export const addAndSwitchLayout: ReducerDeclWithPayload<LayoutSettings, IsEditorPayload & {
    /// The layout to add
    layout: Layout
}> = withPayload((state, {isEditor, layout}) => {
    if (isEditor) {
        state.savedEditorLayouts.push(layout);
        state.currentEditorLayout = state.savedEditorLayouts.length;
    } else {
        state.savedViewerLayouts.push(layout);
        state.currentViewerLayout = state.savedViewerLayouts.length;
    }
});

/// Delete current layout and switch to default layout
export const deleteCurrentLayout: ReducerDeclWithPayload<LayoutSettings, IsEditorPayload> = withPayload((state, {isEditor}) => {
    if (isEditor) {
        if (state.currentEditorLayout > 0 && state.currentEditorLayout <= state.savedEditorLayouts.length) {
            state.savedEditorLayouts.splice(state.currentEditorLayout - 1, 1);
            state.currentEditorLayout = 0;
        }
    } else {
        if (state.currentViewerLayout > 0 && state.currentViewerLayout <= state.savedViewerLayouts.length) {
            state.savedViewerLayouts.splice(state.currentViewerLayout - 1, 1);
            state.currentViewerLayout = 0;
        }
    }
});


