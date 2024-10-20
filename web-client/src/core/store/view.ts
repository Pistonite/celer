//! View store slice
//!
//! The view slice stores global application state that doesn't need to persisted.
//! Such as toolbar, map view, and other UI states.

import type { DocViewState } from "core/doc";
import { docViewReducers, initialDocViewState } from "core/doc";
import type { StageViewState } from "core/stage";
import { stageViewReducers, initialStageViewState } from "core/stage";
import type { LayoutViewState } from "core/layout";
import { initialLayoutViewState, layoutViewReducers } from "core/layout";
import type { MapViewState } from "core/map";
import { initialMapViewState, mapViewReducers } from "core/map";

import { configureSlice } from "low/store";

import type { EditorViewState } from "./editor";
import { initialEditorViewState, editorViewReducers } from "./editor";

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
