//! Utility for the map logic

import { MapAttribution } from "low/celerc";

/// Epsilon for floating point comparison in the map
const EPSILON = 1e-3;

/// Compare 2 floating point numbers that are close enough
export const roughlyEquals = (a: number, b: number): boolean => {
    return Math.abs(a - b) < EPSILON;
};

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
