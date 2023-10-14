//! Reducers for map view state

import { ReducerDeclWithPayload, withPayload } from "low/store";
import { GameCoord } from "low/celerc";

import { MapViewState, MapView } from "./state";

/// Set the current map layer
export const setMapLayer: ReducerDeclWithPayload<MapViewState, number> =
    withPayload((state: MapViewState, value: number) => {
        state.currentMapLayer = value;
    });

/// Set the current map center and zoom
export const setMapView: ReducerDeclWithPayload<
    MapViewState,
    GameCoord[] | MapView
> = withPayload((state: MapViewState, value: GameCoord[] | MapView) => {
    state.currentMapView = value;
});

/// Set the current map zoom without changing center
export const setMapZoom: ReducerDeclWithPayload<MapViewState, number> =
    withPayload((state: MapViewState, value: number) => {
        state.currentMapView = {
            center: [0, 0, 0], // center will be set by the map
            zoom: value,
        };
    });

/// Set map zoom bound
export const setMapZoomBounds: ReducerDeclWithPayload<
    MapViewState,
    [number, number]
> = withPayload((state: MapViewState, value: [number, number]) => {
    state.currentZoomBounds = value;
});
