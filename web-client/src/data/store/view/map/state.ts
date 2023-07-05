//! Map part of the view store
//!
//! This stores the current state of the map

import { GameCoord } from "data/model";

export type MapViewStore = {
    /// Current map layer the user is on
    currentMapLayer: number;
    /// Current map view
    ///
    /// The map listens to this value and updates the map view accordingly.
    /// If the value is an array, the map will fit so all points are visible.
    currentMapView: GameCoord[] | MapView;
    /// Min and max zoom
    ///
    /// This is usually tied to the layer. The map automatically updates this
    /// according to the current layer
    currentZoomBounds: [number, number]
}

/// Map view data
///
/// Center and zoom need to be updated together.
export type MapView = {
    center: GameCoord;
    zoom: number;
}

export const initialMapViewStore: MapViewStore = {
    currentMapView: { center: [0, 0, 0], zoom: 1 },
    currentMapLayer: 0,
    currentZoomBounds: [1, 1],
};
