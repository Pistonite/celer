//! Map logic that wraps L.Map

import L from "leaflet";
import "leaflet-rastercoords";
import "./leaflet-tilelayer-nogap";
import { ToolbarStore, documentSelector, settingsSelector, store, toolbarSelector } from "data/store";
import reduxWatch from "redux-watch";
import { DocumentIconMap, DocumentMapLayer, DocumentMapLayerAttribution, DocumentMapLayerTilesetTransform, ExecutedDocument, MapIcon } from "data/model";
import { IconMarker } from "./IconMarker";

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
export class MapState {
    /// The map
    private map: L.Map;
    /// The tileset layers
    private tilesetLayers: TilesetLayer[] = [];
    /// The active tileset layer
    private activeTilesetLayerIndex = -1;
    /// The icons
    private icons: IconMarker[] = [];

    constructor() {
        //const testPointGame: [number, number] = [0, 0];
        const testPointGame1: [number, number] = [4737.48, 3772.09];
        const testPointGame2: [number, number] = [0, 0];
        const testPointGame3: [number, number] = [-4446.80, -3803.04];

        const a = 2;
        const b = 12000;
        const c = 2;
        const d = 10000;

        const transform = (point: [number, number]): [number, number] => {
            return [
                b + a * point[0],
                d + c * point[1],
            ];
        };

        const tmpContainer = document.createElement("div");
        tmpContainer.id = LMapContainerId;
        tmpContainer.style.backgroundColor = "#000000";
        const tmpDiv = document.querySelector("#tmp");
        if (!tmpDiv) {
            throw new Error("[Map] The temp div is not found. The map cannot be created!");
        }
        tmpDiv.appendChild(tmpContainer);

        // probably only need to create the map
        // since the tile layer is dynamic
        //const crs = L.extend({}, L.CRS.Simple);
        const map = L.map(tmpContainer, {
            crs: L.CRS.Simple,
            renderer: L.canvas(),
            zoomControl: false,
        });
        tmpContainer.remove();

        const rc = new L.RasterCoords(map, [24000, 20000]);
        rc.setMaxBounds();






        map.setView(rc.unproject([0, 0]), 3);
        // new IconMarker(rc.unproject(transform(testPointGame1)),
        //     "https://icons.pistonite.org/icon/shrine.shrine.none.69a2d5.c1fefe.69a2d5.c1fefe.69a2d5.c1fefe.png",0.5)
        // .addTo(map);
        // L.marker(rc.unproject(transform(testPointGame1))).addTo(map);
        L.marker(rc.unproject(transform(testPointGame2))).addTo(map);
        L.marker(rc.unproject(transform(testPointGame3))).addTo(map);

        // Subscribe to store updates

        const watchLayout = reduxWatch(() => settingsSelector(store.getState()).currentLayout);
        store.subscribe(watchLayout(() => {
            this.update("switching layout");
        }));

        const watchToolbar = reduxWatch(() => toolbarSelector(store.getState()));
        store.subscribe(watchToolbar((newVal, oldVal) => {
            this.update("toolbar update");
            this.onToolbarUpdate(newVal);
        }));

        const watchDocument = reduxWatch(() => documentSelector(store.getState()));
        store.subscribe(watchDocument((newVal, oldVal) => {
            console.log("document update");
            this.onDocumentUpdate(newVal.document, oldVal.document);
        }));

        this.map = map;
    }

    /// Get the underlying L.Map
    ///
    /// This can be used to do one-off operations that are not supported by this class.
    /// In most cases, shared logic should be added to this class.
    underlying(): L.Map {
        return this.map;
    }

    /// Attach the map to a container HTMLElement root
    ///
    /// This will add the map container as a child to the root
    attach(container: HTMLElement) {
        // Remove from the old place
        this.map.getContainer().remove();
        // add to new place
        container.appendChild(this.map.getContainer());
        this.update();
    }

    /// Update the map
    ///
    /// This will call invalidateSize() to refresh the map
    update(reason?: string) {
        if (reason) {
            console.log(`[Map] updating map due to ${reason}`);
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
                console.warn(`[map] Icon ${icon.id} is not registered`);
                return undefined;
            }
            
            const [x, y, z] = icon.coord;

            // Get the layer the icon is on
            let layer = this.activeTilesetLayerIndex-1;
            for(;layer <= this.activeTilesetLayerIndex+1; layer++) {
                if (this.isZOnLayer(z, layer)) {
                    break;
                }
            } 
            // icon is not on current layer or adjacent layers
            if (layer > this.activeTilesetLayerIndex+1) {
                return undefined;
            }

            // get the opacity of the icon
            // current layer = 1
            // adjacent layers = 0.5
            const opacity = layer === this.activeTilesetLayerIndex ? 1 : 0.5;
            const { transform, rc }  = this.tilesetLayers[layer];
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
