//! Map component connected to Leaflet

import "./Map.css";
import "leaflet/dist/leaflet.css";
import L from "leaflet";
import "leaflet-rastercoords";
import "./leaflet-tilelayer-nogap";
import { useEffect, useRef } from "react";
import { documentSelector, settingsSelector, store, toolbarSelector } from "data/store";
import reduxWatch from "redux-watch";
import { useSelector } from "react-redux";
import { Loading } from "Loading";
import { MapState } from "./MapState";

console.log("[Map] loading map module");


/// Initialize leaflet map instance
const initMapInstance = (): MapState => {
    if ((window as any)._leaflet_map) {
        console.warn("[Map] found existing map instance. You are either in a dev environment or this is a bug");
        try {
            (window as any)._leaflet_map.underlying().remove();
        } catch (e) {
            console.error(e);
            console.warn("[Map] failed to remove existing map instance");
        }
    }
    console.log("[Map] creating map");

    const map = new MapState();
    (window as any)._leaflet_map = map;
    return map;
}

const mapState: MapState = initMapInstance();


/// Map container that binds to leaflet instance
export const Map: React.FC = () => {
    const { document } = useSelector(documentSelector);
    const rootRef = useRef<HTMLDivElement>(null);

    useEffect(() => {
        if(!rootRef.current) {
            return;
        }
        // see what the current container is
        const prevContainer = rootRef.current.children[0];
        if(prevContainer === mapState.underlying().getContainer()) {
            return;
        }

        // remove the previous container, might not be needed
        if (prevContainer) {
            console.log("[Map] removing previous map container");
            prevContainer.remove();
        }

        console.log("[Map] attaching map to container");
        mapState.attach(rootRef.current);

    }, [rootRef.current]);

    if (!document.loaded) {
        return <Loading color="green" />;
    }

    return (
        <div id="map-root" ref={rootRef}>
            
            
        </div>
    )
}