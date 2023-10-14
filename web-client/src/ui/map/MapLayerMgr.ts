//! MapLayerMgr
import L from "leaflet";
import "leaflet-rastercoords";

import { AppDispatcher, viewActions } from "core/store";
import { MapLayerAttr, MapTilesetTransform, GameCoord } from "low/celerc";

import { MapLog, getAttributionHtml } from "./utils";

/// Tile layer wrapper
type MapLayer = {
    /// The tile layer
    layer: L.TileLayer;
    /// The start Z value
    startZ: number;
    /// Coodinate transformation from game to map
    transform: MapTilesetTransform;
    /// The raster coords of this layer
    rc: L.RasterCoords;
    /// Zoom bound of this layer
    zoomBounds: [number, number];
};

/// Manages the map tile layers and coordinate transformation to layers
export class MapLayerMgr {
    /// Reference to the app store for dispatching actions
    private dispatcher: AppDispatcher;
    /// The tileset layers
    private layers: MapLayer[] = [];
    /// The active tileset layer
    private activeLayerIndex = -1;

    constructor(dispatcher: AppDispatcher) {
        this.dispatcher = dispatcher;
    }

    /// Initialize the tileset layers, remove previous one if exists
    ///
    /// This will also set the map to the initial coord
    public initLayers(
        map: L.Map,
        mapLayers: MapLayerAttr[],
        initialCoord: GameCoord,
    ) {
        MapLog.info("initializing map layers");
        this.getActiveLayer()?.layer.remove();
        // create new tileset layers
        this.layers = mapLayers.map((layer) => {
            // Create raster coords for the layer
            const rc = new L.RasterCoords(map, layer.size);
            const tilesetLayer = L.tileLayer(layer.templateUrl, {
                noWrap: true,
                bounds: rc.getMaxBounds(),
                attribution: getAttributionHtml(layer.attribution),
                maxNativeZoom: layer.maxNativeZoom,
                minZoom: layer.zoomBounds[0],
                maxZoom: layer.zoomBounds[1],
            });
            return {
                layer: tilesetLayer,
                startZ: layer.startZ,
                transform: layer.transform,
                rc,
                zoomBounds: layer.zoomBounds,
            };
        });

        const initialLayer = this.getLayerByZ(initialCoord[2]);
        this.setActiveLayer(map, initialLayer);
    }

    private getActiveLayer(): MapLayer | undefined {
        return this.layers[this.activeLayerIndex];
    }

    /// Set the current layer.
    ///
    /// This will remove the previous layer and add the new one to the map
    public setActiveLayer(map: L.Map, index: number) {
        this.getActiveLayer()?.layer.remove();
        this.activeLayerIndex = index;
        const newLayer = this.getActiveLayer();
        if (newLayer) {
            newLayer.layer.addTo(map);

            this.dispatcher.dispatch(
                viewActions.setMapZoomBounds(newLayer.zoomBounds),
            );
            this.dispatcher.dispatch(viewActions.setMapLayer(index));
        }
    }

    /// Get the active layer index
    public getActiveLayerIndex(): number {
        return this.activeLayerIndex;
    }

    /// Get the latlng given a game coord
    ///
    /// The (x, y) of the game coord will be unprojected using the layer's coordinate transformation.
    /// Z will be used to find the layer.
    ///
    /// Returns [[0, 0], 0] if there are no layers
    public unproject(coord: GameCoord): [L.LatLng, number] {
        const [x, y, z] = coord;
        const layerIndex = this.getLayerByZ(z);
        const layer = this.layers[layerIndex];
        if (!layer) {
            return [L.latLng(0, 0), 0];
        }
        const { transform, rc } = layer;
        const mapX = transform.scale[0] * x + transform.translate[0];
        const mapY = transform.scale[1] * y + transform.translate[1];
        return [rc.unproject([mapX, mapY]), layerIndex];
    }

    /// Get the game coord from latlng, using the current active layer's coordinate transformation
    public project(coord: L.LatLng): GameCoord | undefined {
        const layer = this.getActiveLayer();
        if (!layer) {
            return undefined;
        }
        const { transform, rc } = layer;
        const { x: mapX, y: mapY } = rc.project(coord);

        const x = (mapX - transform.translate[0]) / transform.scale[0];
        const y = (mapY - transform.translate[1]) / transform.scale[1];
        return [x, y, layer.startZ + 1];
    }

    /// Get the layer given a z level.
    public getLayerByZ(z: number): number {
        return (
            this.getLayerByZInRange(z, 0, this.layers.length - 1) ??
            this.layers.length - 1
        );
    }

    /// Get the layer given a z level and a search range relative to the current active layer
    ///
    /// The range is inclusive (e.g. -1, 1 means search from active layer - 1 to active layer + 1)
    public getLayerByZInRelativeRange(
        z: number,
        minRelLayer: number,
        maxRelLayer: number,
    ): number | undefined {
        return this.getLayerByZInRange(
            z,
            this.activeLayerIndex + minRelLayer,
            this.activeLayerIndex + maxRelLayer,
        );
    }

    /// Get the layer given a z level and a absolute search range
    private getLayerByZInRange(
        z: number,
        minLayer: number,
        maxLayer: number,
    ): number | undefined {
        for (let layer = minLayer; layer <= maxLayer; layer++) {
            if (this.isZOnLayer(z, layer)) {
                return layer;
            }
        }
        return undefined;
    }

    /// Check if a z level is on the i-th layer
    public isZOnLayer(z: number, i: number) {
        if (i < 0 || i >= this.layers.length) {
            return false;
        }
        if (i !== 0 && z < this.layers[i].startZ) {
            // icon is below this layer
            return false;
        }
        if (i !== this.layers.length - 1 && z > this.layers[i + 1].startZ) {
            // icon is above this layer
            return false;
        }
        return true;
    }
}
