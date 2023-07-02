//! Reducers for map view state

import { ReducerDeclWithPayload, withPayload } from "data/store/util";
import { GameCoord } from "data/model";

import { MapViewStore } from "./state";

/// Set the current map layer
export const setMapLayer: ReducerDeclWithPayload<
    MapViewStore, number
> = withPayload((state: MapViewStore, value: number) => {
    state.currentMapLayer = value;
});

/// Set the current map center
export const setMapCenter: ReducerDeclWithPayload<
    MapViewStore, GameCoord[]
> = withPayload((state: MapViewStore, value: GameCoord[]) => {
    state.currentMapCenter = value;
});

/// Set the current map zoom
export const setMapZoom: ReducerDeclWithPayload<
    MapViewStore, number
> = withPayload((state: MapViewStore, value: number) => {
    state.currentMapZoom = value;
});

