//! Map logic that wraps L.Map

import L from "leaflet";
import "leaflet/dist/leaflet.css";
import "./leaflet-tilelayer-nogap";

import reduxWatch from "redux-watch";
import {
    AppStore,
    ViewState,
    documentSelector,
    settingsSelector,
    viewActions,
    viewSelector,
} from "core/store";
import { ExecDoc } from "low/compiler.g";
import { Debouncer } from "low/utils";
import { SectionMode } from "core/map";

import { MapLog, roughlyEquals } from "./utils";
import { MapContainerMgr } from "./MapContainerMgr";
import { MapLayerMgr } from "./MapLayerMgr";
import { MapVisualMgr } from "./MapVisualMgr";

MapLog.info("loading map module");

/// Storing map state as window global because HMR will cause the map to be recreated
declare global {
    interface Window {
        __theMapState: MapState | null;
    }
}

/// Map entry point
export const initMap = (store: AppStore): MapState => {
    if (window.__theMapState) {
        window.__theMapState.delete();
    }
    MapLog.info("creating map");

    const map = new MapState(store);
    window.__theMapState = map;

    return map;
};

/// Map options for flying to a point
const FlyOptions = {
    duration: 0.2, // seconds
    easeLinearity: 0.8,
};

/// State of the current map.
///
/// Holds a reference to L.Map
export class MapState {
    /// Reference to the app store
    private store: AppStore;
    /// The map
    private map: L.Map;
    /// Container Manager
    private containerMgr: MapContainerMgr;
    /// Layer Manager
    private layerMgr: MapLayerMgr;
    /// The visual (icons, lines, markers)
    private visualMgr: MapVisualMgr;
    /// Debouncer for recreating the visuals
    private recreateVisualsDebouncer: Debouncer;
    /// Cleanup function
    private cleanup: () => void;

    constructor(store: AppStore) {
        this.containerMgr = new MapContainerMgr();
        this.layerMgr = new MapLayerMgr(store);
        this.visualMgr = new MapVisualMgr(this.layerMgr, store);

        // Create map
        const map = L.map(this.containerMgr.createMapContainer(), {
            crs: L.CRS.Simple,
            renderer: L.canvas(),
            zoomControl: false,
        });

        // Subscribe to store updates
        const watchSettings = reduxWatch(() =>
            settingsSelector(store.getState()),
        );
        const unwatchSettings = store.subscribe(
            watchSettings((_newVal, _oldVal) => {
                this.onSettingsUpdate();
            }),
        );

        const watchView = reduxWatch(() => viewSelector(store.getState()));
        const unwatchView = store.subscribe(
            watchView((newVal, oldVal) => {
                this.onViewUpdate(newVal, oldVal);
            }),
        );

        const watchDocument = reduxWatch(() =>
            documentSelector(store.getState()),
        );
        const unwatchDocument = store.subscribe(
            watchDocument((newVal, oldVal) => {
                if (newVal.serial !== oldVal.serial) {
                    this.onDocumentUpdate(newVal.document, oldVal.document);
                }
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

        // setup update debouncers
        this.recreateVisualsDebouncer = new Debouncer(200, () => {
            const state = store.getState();
            const { document } = documentSelector(state);
            if (!document) {
                return;
            }
            this.visualMgr.recreate(
                this.map,
                document,
                viewSelector(state),
                settingsSelector(state),
            );
        });

        this.map = map;
        this.store = store;
        this.cleanup = () => {
            unwatchSettings();
            unwatchView();
            unwatchDocument();
        };

        // update document initially
        const { document } = documentSelector(store.getState());
        if (document) {
            this.onDocumentUpdate(document);
        }
    }

    /// Delete the map state
    public delete() {
        MapLog.info("deleting map");
        this.map.getContainer().remove();
        this.map.remove();
        this.cleanup();
    }

    /// Attach the map to the root container
    public attach() {
        this.containerMgr.attach(this.map);
    }

    /// If the map is currently attached
    private isAttached() {
        return this.map.getContainer().isConnected;
    }

    /// Called when the settings is updated
    private onSettingsUpdate() {
        /// Update the size since the layout could have changed
        this.map.invalidateSize();
        /// Recreate the visuals
        this.recreateVisualsDebouncer.dispatch();
    }

    /// Called when the document is updated
    ///
    /// This will update the map layers if needed, and will always redraw the map visuals
    private onDocumentUpdate(newDoc: ExecDoc, oldDoc?: ExecDoc) {
        // TODO #82: this needs to be changed. Otherwise changing the map in web editor will not take effect
        // until version is changed, which is weird
        //
        // If the project name and version is the same, assume the map layers are the same
        if (
            !oldDoc ||
            newDoc.project.source !== oldDoc.project.source ||
            newDoc.project.version !== oldDoc.project.version
        ) {
            const { initialCoord, initialZoom, layers } = newDoc.project.map;
            this.layerMgr.initLayers(this.map, layers, initialCoord);
            const [center, _] = this.layerMgr.unproject(initialCoord);
            // initLayers above already sets the correct layer
            this.map.setView(center, initialZoom);
        }
        // recreate the visuals
        this.recreateVisualsDebouncer.dispatch();
    }

    private onViewUpdate(view: ViewState, oldView: ViewState) {
        if (view.isEditingLayout !== oldView.isEditingLayout) {
            this.map.invalidateSize();
        }

        const state = this.store.getState();
        const settings = settingsSelector(state);
        const layerChanged = view.currentMapLayer !== oldView.currentMapLayer;
        const sectionChanged = view.currentSection !== oldView.currentSection;

        if (layerChanged) {
            this.layerMgr.setActiveLayer(this.map, view.currentMapLayer);
        }

        // visuals should be recreated if:
        // 1. layer changed
        // 2. section changed and section mode is current highlight
        const shouldRecreateVisuals =
            layerChanged ||
            (sectionChanged &&
                (settings.iconSectionMode === SectionMode.CurrentHighlight ||
                    settings.lineSectionMode === SectionMode.CurrentHighlight ||
                    settings.markerSectionMode ===
                        SectionMode.CurrentHighlight));

        if (shouldRecreateVisuals) {
            this.recreateVisualsDebouncer.dispatch();
        } else {
            // only update the visuals based on the view and settings
            this.visualMgr.update(this.map, view, settings);
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
                    if (this.isAttached()) {
                        // only fly to bounds if map is attached
                        this.map.flyToBounds(bounds, FlyOptions);
                    }
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
        if (index !== viewSelector(this.store.getState()).currentMapLayer) {
            this.store.dispatch(viewActions.setMapLayer(index));
        }
    }
}
