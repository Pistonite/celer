//! Reducers for map view state

import { withPayload } from "low/store";
import type { GameCoord } from "low/celerc";

import type { MapViewState, MapView } from "./state";

/// Set the current map layer
export const setMapLayer = withPayload<MapViewState, number>((state, value) => {
    state.currentMapLayer = value;
});

/// Set the current map center and zoom
export const setMapView = withPayload<MapViewState, GameCoord[] | MapView>(
    (state, value) => {
        state.currentMapView = value;
    },
);

/// Set the current map zoom without changing center
export const setMapZoom = withPayload<MapViewState, number>((state, value) => {
    state.currentMapView = {
        center: [0, 0, 0], // center will be set by the map
        zoom: value,
    };
});

/// Set map zoom bound
export const setMapZoomBounds = withPayload<MapViewState, [number, number]>(
    (state, value) => {
        state.currentZoomBounds = value;
    },
);
