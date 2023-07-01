//! Utility for the map logic

import { Axis, DocumentMapCoordMap, GameCoord, RouteCoord } from "data/model";

/// Convert a route coordinate to a game coordinate using the coordMap
export const toGameCoord = (routeCoord: RouteCoord, coordMap: DocumentMapCoordMap): GameCoord => {
    const coord: Record<Axis, number> = {
        x: 0,
        y: 0,
        z: 0,
    };

    const mapper = routeCoord[2] === undefined ? coordMap["2d"] : coordMap["3d"];

    mapper.forEach((axis, i) => {
        coord[axis] = routeCoord[i] as number;
    });

    return [coord.x, coord.y, coord.z];
}
