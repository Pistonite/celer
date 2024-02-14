//! Map component connected to Leaflet

import { useEffect, useRef } from "react";
import { useSelector, useStore } from "react-redux";
import { ErrorBoundary, HintScreen, LoadScreen } from "ui/shared";
import { AppStore, documentSelector, viewSelector } from "core/store";

import { MapState, initMap } from "./MapState";
import { MapContainer } from "./MapContainerMgr";
import { useMapStyles } from "./styles";

/// Map root container that the leaflet map instance binds to
export const MapRoot: React.FC = () => {
    const { serial, document } = useSelector(documentSelector);
    const { stageMode, compileInProgress } = useSelector(viewSelector);
    const styles = useMapStyles();
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
        if (stageMode === "edit" && !compileInProgress) {
            return (
                <HintScreen>
                    Map will be shown here once a project is opened
                </HintScreen>
            );
        }
        return <LoadScreen color="green" />;
    }

    const { map } = document.project;
    if (!map) {
        return <HintScreen>This document has no map</HintScreen>;
    }
    if (map.layers.length <= 0) {
        return <HintScreen>This map has no layers</HintScreen>;
    }

    return (
        <ErrorBoundary>
            <div id={MapContainer.id} className={styles.mapContainer} />
        </ErrorBoundary>
    );
};
