//! Utility for the map logic

import { MapAttribution } from "low/celerc";
import { Logger } from "low/utils";

/// Map module logger
export const MapLog = new Logger("map");

/// Epsilon for floating point comparison in the map
const EPSILON = 1e-3;

/// Compare 2 floating point numbers that are close enough
export const roughlyEquals = (a: number, b: number): boolean => {
    return Math.abs(a - b) < EPSILON;
};

// This should be part of the compiler TODO
// Convert a route coordinate to a game coordinate using the coordMap
// export const toGameCoord = (
//     routeCoord: number[],
//     coordMap: MapCoordMap,
// ): GameCoord => {
//     const coord: Record<Axis, number> = {
//         x: 0,
//         y: 0,
//         z: 0,
//     };
//
//     const mapper =
//         routeCoord[2] === undefined ? coordMap["2d"] : coordMap["3d"];
//
//     mapper.forEach((axis, i) => {
//         coord[axis] = routeCoord[i] as number;
//     });
//
//     return [coord.x, coord.y, coord.z];
// };

/// Get attribution html to be used in the map
///
/// This uses `innerText` to sanitize the link.
export const getAttributionHtml = (
    attribution: MapAttribution,
): string | undefined => {
    if (!attribution.link) {
        return undefined;
    }
    const a = document.createElement("a");
    a.href = attribution.link;
    a.innerText = attribution.link;
    return `${attribution.copyright ? "&copy;" : ""}${a.outerHTML}`;
};
