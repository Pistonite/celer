//! IconMarker is used to draw icons on the map

import L from "leaflet";
import { MapLog } from "./util";

/// Icon marker options
export type IconMarkerOptions = L.CircleMarkerOptions & {
    /// HTML img element with the icon
    icon: HTMLImageElement;
    /// Size
    size: number;
    /// Opacity
    opacity: number;
};

interface LeafletLayer {
    _empty?(): boolean;
    _point?: L.Point;
    options?: IconMarkerOptions;
}

// extend L.Canvas to include a custom function to draw our icons
L.Canvas.include({
    /// Draw the icon marker at its position 
    drawIconMarker(layer: LeafletLayer, size: number, opacity: number) {
        // eslint-disable-line @typescript-eslint/no-explicit-any
        if (!layer) {
            return;
        }
        if (layer._empty && layer._empty()) {
            return;
        }

        const p = layer._point;
        if (!p) {
            return;
        }
        const ctx = this._ctx;
        const img = layer.options?.icon;
        if (!ctx || !img) {
            return;
        }
        ctx.globalAlpha = opacity;
        ctx.drawImage(
            img,
            p.x - size / 2,
            p.y - size / 2,
            size,
            size,
        );
        ctx.globalAlpha = 1;
    },
});

interface MarkerWithRenderer {
    _renderer?:{
        drawIconMarker(layer: LeafletLayer, size: number, opacity: number): void;
    } 
}

/// Icon marker class
///
/// Hacking the CircleMarker class to draw an icon
export class IconMarker extends L.CircleMarker {
    constructor(
        latlng: L.LatLngExpression,
        iconSrc: string,
        opacity: number,
        size: number,
        options: L.CircleMarkerOptions = {},
    ) {
        const icon = document.createElement("img");
        icon.src = iconSrc;
        super(
            latlng,
            Object.assign(options, { icon, size, opacity }) as L.CircleMarkerOptions,
        );
    }
    /// Hacking the updatePath function to draw our icon
    _updatePath() {
        const options = this.options as IconMarkerOptions;
        if (!options || !options.icon) {
            MapLog.warn("missing icon on icon marker");
            return;
        }
        const renderer  = (this as MarkerWithRenderer)._renderer; // eslint-disable-line @typescript-eslint/no-explicit-any
        if (!renderer || !renderer.drawIconMarker) {
            MapLog.warn("invalid icon markder renderer");
            return;
        }
        renderer.drawIconMarker(this as unknown as LeafletLayer, options.size, options.opacity);
    }
}
