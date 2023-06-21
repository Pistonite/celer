//! Map logic that wraps L.Map

import L from "leaflet";
import "leaflet-rastercoords";
import "./leaflet-tilelayer-nogap";
import { documentSelector, settingsSelector, store, toolbarSelector } from "data/store";
import reduxWatch from "redux-watch";
import { DocumentMapLayer, DocumentMapLayerTilesetTransform, ExecutedDocument } from "data/model";

/// Leaflet map container div id
const LMapContainerId = "lmap-container";

/// Tile layer wrapper
type TilesetLayer = {
    /// The tile layer
    layer: L.TileLayer,
    /// The start Z value
    startZ: number,
    /// Coodinate transformation from game to map
    transform: DocumentMapLayerTilesetTransform
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
            ]
        }

   

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
        });
        tmpContainer.remove();

        const rc = new L.RasterCoords(map, [24000, 20000]);
        rc.setMaxBounds();



        


        map.setView(rc.unproject([0, 0]), 3);
        L.marker(rc.unproject(transform(testPointGame1))).addTo(map);
        L.marker(rc.unproject(transform(testPointGame2))).addTo(map);
        L.marker(rc.unproject(transform(testPointGame3))).addTo(map);

        // Subscribe to store updates
  
        const watchLayout = reduxWatch(() => settingsSelector(store.getState()).currentLayout);
        store.subscribe(watchLayout(() => {
            this.update("switching layout");
        }));
        
        const watchToolbar = reduxWatch(() => toolbarSelector(store.getState()));
        store.subscribe(watchToolbar(() => {
            this.update("toolbar update");
        }));

        const watchDocument = reduxWatch(() => documentSelector(store.getState()));
        store.subscribe(watchDocument((newVal, oldVal) => {
            this.onDocumentUpdate(newVal, oldVal);
        }));
        
        this.map = map;
        this.tilesetLayer = null;
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
            // TODO update map layers
            const tilesetLayer = L.tileLayer('https://objmap.zeldamods.org/game_files/maptex/{z}/{x}/{y}.png', {
            attribution: '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors',
            maxNativeZoom: 7,
            bounds: rc.getMaxBounds(),
            noWrap: true,
        });

        tilesetLayer.addTo(map);
        }
    }

    /// Initialize the tileset layer, remove previous one if exists
    private initTilesetLayer(mapLayers: DocumentMapLayer[]) {
        this.getActiveTilesetLayer()?.layer.remove();
        // create new tileset layers
        this.tilesetLayers = mapLayers.map((layer) => {
            // Create raster coords for the layer
            const rc = new L.RasterCoords(this.map, layer.tileset.size);
        })
        const tilesetLayer = L.tileLayer('https://objmap.zeldamods.org/game_files/maptex/{z}/{x}/{y}.png', {
    });
    }


    private getActiveTilesetLayer(): TilesetLayer | null {
        if (this.activeTilesetLayerIndex < 0 || this.activeTilesetLayerIndex >= this.tilesetLayers.length) {
            return null;
        }
        return this.tilesetLayers[this.activeTilesetLayerIndex];
    }

}