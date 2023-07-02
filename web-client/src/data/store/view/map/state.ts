//! Map part of the view store
//!
//! This stores the current state of the map

import { GameCoord } from "data/model";

export type MapViewStore = {
    /// Current map layer the user is on
    currentMapLayer: number;
    /// Current map center
    ///
    /// If the value is an array with one element, that element is the center coord.
    /// Otherwise, the map should fit the bounds so that all points are in view.
    currentMapCenter: GameCoord[];
    /// Current map zoom
    currentMapZoom: number;
}

export const initialMapViewStore: MapViewStore = {
    currentMapLayer: 0,
    currentMapCenter: [[0, 0, 0]],
    currentMapZoom: 1, // 0 zoom may cause issues in the map calculation
};
