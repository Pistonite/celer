//! Hook for controlling map zoom from react

import { useSelector } from "react-redux";
import { documentSelector, viewActions, viewSelector } from "core/store";
import { useActions } from "low/store";

/// Zoom control hook
///
/// Returns a function that can be called to zoom in or out.
/// Returns undefined if the zoom control should be disabled
export const useZoomControl = (isZoomIn: boolean): (() => void) | undefined => {
    const { document } = useSelector(documentSelector);
    const {
        currentMapView,
        currentZoomBounds: [min, max],
    } = useSelector(viewSelector);
    const { setMapZoom } = useActions(viewActions);
    if (!document) {
        // document is not loaded
        return undefined;
    }

    if (Array.isArray(currentMapView)) {
        // map is being adjusted, so zoom control should be disabled
        return undefined;
    }

    const zoom = currentMapView.zoom;

    if (isZoomIn) {
        if (zoom < max) {
            return () => {
                setMapZoom(Math.min(zoom + 1, max));
            };
        }
    } else {
        if (zoom > min) {
            return () => {
                setMapZoom(Math.max(zoom - 1, min));
            };
        }
    }

    return undefined;
};
