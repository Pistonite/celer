//! Map logic that wraps L.Map

import L from "leaflet";
import "leaflet/dist/leaflet.css";
import "./leaflet-tilelayer-nogap";

import reduxWatch from "redux-watch";

import {
    ViewStore,
    documentSelector,
    settingsSelector,
    store,
    viewActions,
    viewSelector,
} from "data/store";
import { ExecutedDocument } from "data/model";

import { IconMarker } from "./IconMarker";
import { MapLog, roughlyEquals } from "./util";
import { MapContainerMgr } from "./MapContainerMgr";
import { MapLayerMgr } from "./MapLayerMgr";

MapLog.info("loading map module");

/// Storing map state as window global because HMR will cause the map to be recreated
declare global {
    interface Window {
        __theMapState: MapState | null;
    }
}

/// Map entry point
export const initMap = (): MapState => {
    if (window.__theMapState) {
        MapLog.warn(
            "found existing map instance. You are either in a dev environment or this is a bug",
        );
        return window.__theMapState;
    }
    MapLog.info("creating map");

    const map = new MapState();
    window.__theMapState = map;

    return map;
};

const FlyOptions = {
    duration: 0.2, // seconds
    easeLinearity: 0.8,
};

/// State of the current map.
///
/// Holds a reference to L.Map
class MapState {
    /// The map
    private map: L.Map;
    /// Container Manager
    private containerMgr: MapContainerMgr;
    /// Layer Manager
    private layerMgr: MapLayerMgr;
    /// The icons
    private icons: IconMarker[] = [];

    constructor() {
        this.containerMgr = new MapContainerMgr();
        this.layerMgr = new MapLayerMgr();

        // Create map
        const map = L.map(this.containerMgr.createMapContainer(), {
            crs: L.CRS.Simple,
            renderer: L.canvas(),
            zoomControl: false,
        });

        // Subscribe to store updates
        const watchLayout = reduxWatch(
            () => settingsSelector(store.getState()).currentLayout,
        );
        store.subscribe(
            watchLayout(() => {
                this.map.invalidateSize();
            }),
        );

        const watchView = reduxWatch(() => viewSelector(store.getState()));
        store.subscribe(
            watchView((newVal, oldVal) => {
                this.onViewUpdate(newVal, oldVal);
            }),
        );

        const watchDocument = reduxWatch(() =>
            documentSelector(store.getState()),
        );
        store.subscribe(
            watchDocument((newVal, oldVal) => {
                this.onDocumentUpdate(newVal.document, oldVal.document);
            }),
        );

        const updateView = () => {
            const center = this.layerMgr.project(this.map.getCenter());
            if (!center) {
                return;
            }
            store.dispatch(
                viewActions.setMapView({
                    center,
                    zoom: this.map.getZoom(),
                }),
            );
        };

        map.on("zoomend", () => {
            updateView();
        });

        map.on("moveend", () => {
            updateView();
        });

        this.map = map;
    }

    /// Attach the map to the root container
    public attach() {
        this.containerMgr.attach(this.map);
    }

    /// Called when the document is updated
    ///
    /// This will update the map layers if needed, and will always redraw the map icons and lines
    private onDocumentUpdate(
        newDoc: ExecutedDocument,
        oldDoc: ExecutedDocument,
    ) {
        if (!newDoc.loaded) {
            // do nothing if doc is not loaded
            // we should be notified again when doc loads
            return;
        }
        // If the project name and version is the same, assume the map layers are the same
        if (
            newDoc.project.name !== oldDoc.project.name ||
            newDoc.project.version !== oldDoc.project.version
        ) {
            const { initialCoord, initialZoom, layers } = newDoc.project.map;
            this.layerMgr.initLayers(this.map, layers, initialCoord);
            const [center, _] = this.layerMgr.unproject(initialCoord);
            // initLayers above already sets the correct layer
            this.map.setView(center, initialZoom);
        }
        // Redraw all the icons
        this.updateIcons(newDoc);
    }

    private onViewUpdate(view: ViewStore, oldView: ViewStore) {
        if (view.isEditingLayout !== oldView.isEditingLayout) {
            this.map.invalidateSize();
        }

        if (view.currentMapLayer !== this.layerMgr.getActiveLayerIndex()) {
            this.layerMgr.setActiveLayer(this.map, view.currentMapLayer);
            // redraw icons
            const doc = documentSelector(store.getState()).document;
            this.updateIcons(doc);
        }

        const currentMapView = view.currentMapView;
        if (Array.isArray(currentMapView)) {
            if (currentMapView.length === 0) {
                MapLog.warn("invalid map view");
            } else if (currentMapView.length === 1) {
                setTimeout(() => {
                    const [center, layer] = this.layerMgr.unproject(
                        currentMapView[0],
                    );
                    this.setLayerInStore(layer);
                    this.map.flyTo(center, undefined, FlyOptions);
                }, 0);
            } else {
                // find the min max x and y, and min z
                let minX = Infinity;
                let minY = Infinity;
                let minZ = Infinity;
                let maxX = -Infinity;
                let maxY = -Infinity;
                currentMapView.forEach((coord) => {
                    const [x, y, z] = coord;
                    minX = Math.min(minX, x);
                    minY = Math.min(minY, y);
                    minZ = Math.min(minZ, z);
                    maxX = Math.max(maxX, x);
                    maxY = Math.max(maxY, y);
                });
                const [corner1, layer] = this.layerMgr.unproject([
                    minX,
                    minY,
                    minZ,
                ]);
                const [corner2, _] = this.layerMgr.unproject([
                    maxX,
                    maxY,
                    minZ,
                ]);
                const bounds = L.latLngBounds(corner1, corner2);
                setTimeout(() => {
                    this.setLayerInStore(layer);
                    this.map.flyToBounds(bounds, FlyOptions);
                });
            }
        } else {
            // update map zoom
            // we don't update center here because it will be inaccurate when zooming
            if (!roughlyEquals(currentMapView.zoom, this.map.getZoom())) {
                this.map.setZoom(currentMapView.zoom);
            }
        }
    }

    /// Change the current layer
    private setLayerInStore(index: number) {
        if (index !== viewSelector(store.getState()).currentMapLayer) {
            store.dispatch(viewActions.setMapLayer(index));
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
        // create new icon markers
        this.icons = mapIcons
            .map((icon) => {
                const iconSrc = registeredIcons[icon.id];
                if (!iconSrc) {
                    MapLog.warn(`Icon ${icon.id} is not registered`);
                    return undefined;
                }

                const z = icon.coord[2];
                const layer = this.layerMgr.getLayerByZInRelativeRange(
                    z,
                    -1,
                    1,
                );
                if (!layer) {
                    // icon is not on current layer or adjacent layers
                    return undefined;
                }

                // get the opacity of the icon
                // current layer = 1
                // adjacent layers = 0.5
                const opacity =
                    layer === this.layerMgr.getActiveLayerIndex() ? 1 : 0.5;
                const [latlng] = this.layerMgr.unproject(icon.coord);
                return new IconMarker(latlng, iconSrc, opacity);
            })
            .filter(Boolean) as IconMarker[];
        // add new icons
        this.icons.forEach((icon) => icon.addTo(this.map));
    }
}
