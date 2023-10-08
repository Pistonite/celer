//! Map component connected to Leaflet

import "./Map.css";

import { useEffect, useRef } from "react";
import { useSelector, useStore } from "react-redux";
import { LoadScreen, ErrorScreen, ErrorBoundary } from "ui/shared";
import { AppStore, documentSelector } from "core/store";

import { MapState, initMap } from "./MapState";
import { RootContainerId } from "./MapContainerMgr";

/// Map root container that the leaflet map instance binds to
export const MapRoot: React.FC = () => {
    const { serial, document } = useSelector(documentSelector);
    const store = useStore();
    const mapState = useRef<MapState | null>(null);
    /* eslint-disable react-hooks/exhaustive-deps*/
    useEffect(() => {
        // attach the map only if doc is loaded
        if (document) {
            // create the map if needed
            if (mapState.current === null) {
                mapState.current = initMap(store as AppStore);
            }
            mapState.current.attach();
        }
    }, [serial, store]);
    /* eslint-enable react-hooks/exhaustive-deps*/

    if (!document) {
        return <LoadScreen color="green" />;
    }

    if (document.project.map.layers.length <= 0) {
        return <ErrorScreen message="This map has no layers" />;
    }

    return (
        <ErrorBoundary>
            <div id={RootContainerId}></div>
        </ErrorBoundary>
    );
};
