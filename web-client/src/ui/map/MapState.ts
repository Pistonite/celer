//! Map logic that wraps L.Map

import L from "leaflet";
import "leaflet-rastercoords";
import "leaflet/dist/leaflet.css";
import "./leaflet-tilelayer-nogap";
import { ToolbarStore, documentSelector, settingsSelector, store, toolbarSelector } from "data/store";
import reduxWatch from "redux-watch";
import { DocumentMapLayer, DocumentMapLayerAttribution, DocumentMapLayerTilesetTransform, ExecutedDocument } from "data/model";
import { IconMarker } from "./IconMarker";

const log = (msg: string) => {
    console.log(`[Map] ${msg}`); // eslint-disable-line no-console
};

log("loading map module");

/// Storing map state as window global because HMR will cause the map to be recreated
declare global {
    interface Window {
        __theMapState: MapState | null;
    }
}

/// Map entry point
///
/// This should be called when the map component is first mounted
export const initMap = () => {
    if (window.__theMapState) {
        console.warn("[Map] found existing map instance. You are either in a dev environment or this is a bug");
        try {
            window.__theMapState.delete();
        } catch (e) {
            console.error(e);
            console.warn("[Map] failed to remove existing map instance");
        }
    }
    log("[Map] creating map");

    const map = new MapState();
    map.tryAttachAsyncUntilSuccess();
    window.__theMapState = map;
};

/// Container div id
export const RootContainerId = "map-root";
/// Leaflet map container div id
const LMapContainerId = "lmap-container";

/// Tile layer wrapper
type TilesetLayer = {
    /// The tile layer
    layer: L.TileLayer,
    /// The start Z value
    startZ: number,
    /// Coodinate transformation from game to map
    transform: DocumentMapLayerTilesetTransform,
    /// The raster coords of this layer
    rc: L.RasterCoords,
}

/// State of the current map.
///
/// Holds a reference to L.Map
class MapState {
    /// The map
    private map: L.Map;
    /// The attach update handle
    private attachUpdateHandle: number | null = null;
    /// The tileset layers
    private tilesetLayers: TilesetLayer[] = [];
    /// The active tileset layer
    private activeTilesetLayerIndex = -1;
    /// The icons
    private icons: IconMarker[] = [];

    constructor() {
        // Create map container
        const container = document.createElement("div");
        container.id = LMapContainerId;
        container.style.backgroundColor = "#000000";

        const map = L.map(container, {
            crs: L.CRS.Simple,
            renderer: L.canvas(),
            zoomControl: false,
        });

        // Subscribe to store updates

        const watchLayout = reduxWatch(() => settingsSelector(store.getState()).currentLayout);
        store.subscribe(watchLayout(() => {
            this.update("switching layout");
        }));

        const watchToolbar = reduxWatch(() => toolbarSelector(store.getState()));
        store.subscribe(watchToolbar((newVal, _oldVal) => {
            this.update("toolbar update");
            this.onToolbarUpdate(newVal);
        }));

        const watchDocument = reduxWatch(() => documentSelector(store.getState()));
        store.subscribe(watchDocument((newVal, oldVal) => {
            // console.log("document update");
            this.onDocumentUpdate(newVal.document, oldVal.document);
        }));

        this.map = map;
    }

    /// Delete the map and free up the resources
    delete() {
        if (this.attachUpdateHandle) {
            window.clearTimeout(this.attachUpdateHandle);
            this.attachUpdateHandle = null;
        }
        this.map.remove();
    }

    /// Attempt to attach the map to the root container until success
    tryAttachAsyncUntilSuccess() {
        if (this.attachUpdateHandle) {
            // already trying
            return;
        }
        if (this.attach()) {
            // attached
            return;
        }
        this.attachUpdateHandle = window.setTimeout(() => {
            this.attachUpdateHandle = null;
            this.tryAttachAsyncUntilSuccess();
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
    private attach(root?: HTMLElement | undefined | null) {
        if (!root) {
            const rootInDom = document.getElementById(RootContainerId);
            if (!rootInDom) {
                return false;
            }
            root = rootInDom;
        }
        // see what the current container is
        const prevContainer = root.children[0];
        if (prevContainer === this.map.getContainer()) {
            return true;
        }

        // remove the previous container, might not be needed
        if (prevContainer) {
            prevContainer.remove();
        }

        log("attaching map to container");

        // Remove from the old place
        this.map.getContainer().remove();
        // add to new place
        root.appendChild(this.map.getContainer());
        this.update();

        return true;
    }

    /// Update the map
    ///
    /// This will call invalidateSize() to refresh the map
    private update(reason?: string) {
        if (reason) {
            log(`updating map due to ${reason}`);
        }
        this.map.invalidateSize();
    }

    /// Called when the document is updated
    ///
    /// This will update the map layers if needed, and will always redraw the map icons and lines
    private onDocumentUpdate(newDoc: ExecutedDocument, oldDoc: ExecutedDocument) {
        // If the project name and version is the same, assume the map layers are the same
        if (newDoc.project.name !== oldDoc.project.name || newDoc.project.version !== oldDoc.project.version) {
            this.initTilesetLayers(newDoc.project.map.layers);
        }
        // Update the current tileset layer
        this.setActiveTilesetLayer(0);
        // Redraw all the icons
        this.updateIcons(newDoc);

    }

    /// Initialize the tileset layers, remove previous one if exists
    private initTilesetLayers(mapLayers: DocumentMapLayer[]) {
        this.getActiveTilesetLayer()?.layer.remove();
        // create new tileset layers
        this.tilesetLayers = mapLayers.map((layer) => {
            // Create raster coords for the layer
            const rc = new L.RasterCoords(this.map, layer.size);
            const tilesetLayer = L.tileLayer(layer.templateUrl, {
                noWrap: true,
                bounds: rc.getMaxBounds(),
                attribution: this.getAttributionHtml(layer.attribution),
                maxNativeZoom: layer.maxNativeZoom,
            });
            return {
                layer: tilesetLayer,
                startZ: layer.startZ,
                transform: layer.transform,
                rc,
            };
        });
    }

    private setActiveTilesetLayer(index: number) {
        this.getActiveTilesetLayer()?.layer.remove();
        this.activeTilesetLayerIndex = index;
        this.getActiveTilesetLayer()?.layer.addTo(this.map);
    }

    private getActiveTilesetLayer(): TilesetLayer | null {
        if (this.activeTilesetLayerIndex < 0 || this.activeTilesetLayerIndex >= this.tilesetLayers.length) {
            return null;
        }
        return this.tilesetLayers[this.activeTilesetLayerIndex];
    }

    private getAttributionHtml(attribution: DocumentMapLayerAttribution): string | undefined {
        if (!attribution.link) {
            return undefined;
        }
        return `${attribution.copyright ? "&copy;" : ""}<a href="${attribution.link}">${attribution.link}</a>`;
    }

    private onToolbarUpdate(toolbar: ToolbarStore) {
        if (toolbar.currentMapLayer !== this.activeTilesetLayerIndex) {
            this.setActiveTilesetLayer(toolbar.currentMapLayer);
            // redraw icons
            const doc = documentSelector(store.getState()).document;
            this.updateIcons(doc);
        }
    }

    /// Update the icons on the map
    ///
    /// Requires tileset layers to be up-to-date (for transforms to work)
    /// This will filter the icons based on the layer and other settings
    private updateIcons(doc: ExecutedDocument) {
        const registeredIcons = doc.project.icons;
        const mapIcons = doc.map.icons;
        // remove existing icons
        this.icons.forEach((icon) => icon.remove());
        this.icons = [];
        const activeLayer = this.getActiveTilesetLayer();
        if (!activeLayer) {
            return;
        }
        // create new icon markers
        this.icons = mapIcons.map((icon) => {
            const iconSrc = registeredIcons[icon.id];
            if (!iconSrc) {
                console.warn(`[Map] Icon ${icon.id} is not registered`);
                return undefined;
            }

            const [x, y, z] = icon.coord;

            // Get the layer the icon is on
            let layer = this.activeTilesetLayerIndex - 1;
            for (; layer <= this.activeTilesetLayerIndex + 1; layer++) {
                if (this.isZOnLayer(z, layer)) {
                    break;
                }
            }
            // icon is not on current layer or adjacent layers
            if (layer > this.activeTilesetLayerIndex + 1) {
                return undefined;
            }

            // get the opacity of the icon
            // current layer = 1
            // adjacent layers = 0.5
            const opacity = layer === this.activeTilesetLayerIndex ? 1 : 0.5;
            const { transform, rc } = this.tilesetLayers[layer];
            const mapX = transform.scale[0] * x + transform.translate[0];
            const mapY = transform.scale[1] * y + transform.translate[1];
            const latlng = rc.unproject([mapX, mapY]);
            return new IconMarker(latlng, iconSrc, opacity);
        }).filter(Boolean) as IconMarker[];
        // add new icons
        this.icons.forEach((icon) => icon.addTo(this.map));
    }

    /// Check if a z level is on the i-th layer
    private isZOnLayer(z: number, i: number) {
        if (i < 0 || i >= this.tilesetLayers.length) {
            return false;
        }
        if (i !== 0 && z < this.tilesetLayers[i].startZ) {
            // icon is below this layer
            return false;
        }
        if (i !== this.tilesetLayers.length - 1 && z > this.tilesetLayers[i + 1].startZ) {
            // icon is above this layer
            return false;
        }
        return true;
    }
}
