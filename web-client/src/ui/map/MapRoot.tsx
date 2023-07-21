//! Map component connected to Leaflet

import "./Map.css";

import { useEffect, useRef } from "react";
import { useSelector } from "react-redux";

import { LoadScreen, ErrorScreen, useAppStore } from "ui/shared";
import { documentSelector } from "core/store";

import { MapState, initMap } from "./MapState";
import { RootContainerId } from "./MapContainerMgr";

/// Map root container that the leaflet map instance binds to
export const MapRoot: React.FC = () => {
    const { document } = useSelector(documentSelector);
    const store = useAppStore();
    const mapState = useRef<MapState|null>(null);
    useEffect(() => {
        // attach the map only if doc is loaded
        if (document.loaded) {
            // create the map if needed
            if (mapState.current === null) {
                mapState.current = initMap(store);
            }
            mapState.current.attach();
        }
    }, [document.loaded, store]);

    if (!document.loaded) {
        return <LoadScreen color="green" />;
    }

    if (document.project.map.layers.length <= 0) {
        return <ErrorScreen message="This map has no layers" />;
    }

    return <div id={RootContainerId}></div>;
};
