//! View store slice
//!
//! The view slice stores global application state that doesn't need to persisted.
//! Such as toolbar, map view, and other UI states.

import { DocViewState, docViewReducers, initialDocViewState } from "core/doc";
import {
    StageViewState,
    stageViewReducers,
    initialStageViewState,
} from "core/stage";
import {
    LayoutViewState,
    initialLayoutViewState,
    layoutViewReducers,
} from "core/layout";
import { MapViewState, initialMapViewState, mapViewReducers } from "core/map";

import { configureSlice } from "low/store";

import {
    EditorViewState,
    initialEditorViewState,
    editorViewReducers,
} from "./editor";

export type ViewState = LayoutViewState &
    MapViewState &
    DocViewState &
    StageViewState &
    EditorViewState;

/// The toolbar store slice
export const { viewReducer, viewActions, viewSelector } = configureSlice({
    name: "view",
    initialState: {
        ...initialLayoutViewState,
        ...initialMapViewState,
        ...initialDocViewState,
        ...initialStageViewState,
        ...initialEditorViewState,
    },
    reducers: {
        ...layoutViewReducers,
        ...mapViewReducers,
        ...docViewReducers,
        ...stageViewReducers,
        ...editorViewReducers,
    },
});
