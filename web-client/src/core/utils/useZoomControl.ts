//! Hook for controlling map zoom from react

import { useSelector } from "react-redux";

import { useActions, viewSelector, viewActions } from "data/store";

export const useZoomControl = (isZoomIn: boolean): (() => void) | undefined => {
    const { currentMapView, currentZoomBounds: [min, max] } = useSelector(viewSelector);
    const { setMapZoom } = useActions(viewActions);

    if (Array.isArray(currentMapView)) {
        // map is being adjusted, so zoom control should be disabled
        return undefined;
    }

    const zoom = currentMapView.zoom;

    if (isZoomIn) {
        if (zoom < max) {
            return () => {
                setMapZoom(Math.min(zoom+1, max));
            };
        }
    } else {
        if (zoom > min) {
            return () => {
                setMapZoom(Math.max(zoom-1, min));
            };
        }
    }

    return undefined;
};
