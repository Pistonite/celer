//! Map component connected to Leaflet

import "./Map.css";

import { useEffect } from "react";
import { useSelector } from "react-redux";

import { LoadScreen, ErrorScreen } from "ui/components";
import { documentSelector } from "data/store";

import { initMap } from "./MapState";
import { RootContainerId } from "./MapContainerMgr";

const map = initMap();

/// Map container that the leaflet map instance binds to
export const Map: React.FC = () => {
    const { document } = useSelector(documentSelector);
    useEffect(() => {
        if (document.loaded) {
            map.attach();
        }
    }, [document.loaded]);

    if (!document.loaded) {
        return <LoadScreen color="green" />;
    }

    if (document.project.map.layers.length <= 0) {
        return <ErrorScreen message="This map has no layers" />;
    }

    return <div id={RootContainerId}></div>;
};
