//! MapContainerMgr
import L from "leaflet";

import { MapLog } from "./utils";

/// Container div id
export const RootContainerId = "map-root";
/// Leaflet map container div id
const LMapContainerId = "lmap-container";

/// Map container manager
//.
/// Responsible for attaching the map to the root container
export class MapContainerMgr {
    /// The attach update handle
    private attachUpdateHandle: number | null = null;

    public createMapContainer(): HTMLElement {
        const container = document.createElement("div");
        container.id = LMapContainerId;
        container.style.backgroundColor = "#000000";
        return container;
    }

    /// Attempt to attach the map to the root container until success
    public attach(map: L.Map, attempt?: number) {
        if (this.attachUpdateHandle) {
            // already trying
            return;
        }
        if (this.attachInternal(map)) {
            return;
        }
        if (attempt) {
            if (attempt === 10) {
                MapLog.warn("failed to attach to root container after max retries. Futher failures will be ignored");
            } else if (attempt < 10) {
                MapLog.warn("failed to attach to root container. Will retry in 1s");
            }
        }
        this.attachUpdateHandle = window.setTimeout(() => {
            this.attachUpdateHandle = null;
            this.attach(map, attempt ? attempt + 1 : 1);
        }, 1000);
    }

    /// Attach the map to a container HTMLElement root
    ///
    /// This will add the map container as a child to the root.
    /// If the root is not provided, it will search for the root by id
    /// and attached to that if found.
    ///
    /// Return true if the map ends up being attached to a container,
    /// either it is already attached, or newly attached.
    private attachInternal(map: L.Map): boolean {
        const root = document.getElementById(RootContainerId);
        if (!root) {
            return false;
        }
        // see what the current container is
        const prevContainer = root.children[0];
        if (prevContainer === map.getContainer()) {
            return true;
        }

        // remove the previous container, might not be needed
        if (prevContainer) {
            prevContainer.remove();
        }

        MapLog.info("attaching map to container");

        // Remove from the old place
        map.getContainer().remove();
        // add to new place
        root.appendChild(map.getContainer());
        // Update the size
        map.invalidateSize();

        return true;
    }
}
