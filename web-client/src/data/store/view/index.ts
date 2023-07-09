//! View store slice
//!
//! The view slice stores global application state that doesn't need to persisted.
//! Such as toolbar, map view, and other UI states.

import {
    ReducerDeclWithPayload,
    configureSlice,
    withPayload,
} from "data/store/util";

import { MapViewStore, initialMapViewStore } from "./map";
import * as mapViewReducers from "./mapReducers";
import { DocViewStore, initialDocViewStore } from "./doc";
import * as docViewReducers from "./docReducers";

/// The toolbar slice state
export type ViewStore = MapViewStore &
    DocViewStore & {
        /// If the user is currently editing the layout
        isEditingLayout: boolean;
    };

const initialState: ViewStore = {
    ...initialMapViewStore,
    ...initialDocViewStore,
    isEditingLayout: false,
};

/* reducers: TODO may need to refactor */
const setIsEditingLayout: ReducerDeclWithPayload<ViewStore, boolean> =
    withPayload((state: ViewStore, value: boolean) => {
        state.isEditingLayout = value;
    });

/// The toolbar store slice
export const { viewReducer, viewActions, viewSelector } = configureSlice({
    name: "view",
    initialState,
    reducers: {
        setIsEditingLayout,
        ...mapViewReducers,
        ...docViewReducers,
    },
});
