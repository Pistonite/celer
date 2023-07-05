//! Reducers for map view state

import { ReducerDeclWithPayload, withPayload } from "data/store/util";
import { GameCoord } from "data/model";

import { MapViewStore, MapView } from "./state";

/// Set the current map layer
export const setMapLayer: ReducerDeclWithPayload<
    MapViewStore, number
> = withPayload((state: MapViewStore, value: number) => {
    state.currentMapLayer = value;
});

/// Set the current map center and zoom
export const setMapView: ReducerDeclWithPayload<
    MapViewStore, GameCoord[] | MapView
> = withPayload((state: MapViewStore, value: GameCoord[] | MapView) => {
    state.currentMapView = value;
});

/// Set the current map zoom without changing center
export const setMapZoom: ReducerDeclWithPayload<
    MapViewStore, number
> = withPayload((state: MapViewStore, value: number) => {
    state.currentMapView = {
        center: [0, 0, 0], // center will be set by the map
        zoom: value,
    };
});

/// Set map zoom bound
export const setMapZoomBounds: ReducerDeclWithPayload<
    MapViewStore, [number, number]
> = withPayload((state: MapViewStore, value: [number, number]) => {
    state.currentZoomBounds = value;
});

