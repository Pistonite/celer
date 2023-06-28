import L from "leaflet";

/// Icon marker options
export type IconMarkerOptions = L.CircleMarkerOptions & {
    /// HTML img element with the icon
    icon: HTMLImageElement;
    /// Opacity
    opacity: number;
};

/// Icon marker size
const IconSize = 32;

// extend L.Canvas to include a custom function to draw our icons
L.Canvas.include({
    /// Draw the icon marker at its position using hard-coded size
    drawIconMarker(layer: any, opacity: number) { // eslint-disable-line @typescript-eslint/no-explicit-any
        if (!layer) {
            return;
        }
        if (layer._empty && layer._empty()){
            return;
        }

        const p: L.Point = layer._point;
        if (!p) {
            return;
        }
        const ctx = this._ctx;
        const img: HTMLImageElement = layer.options?.icon;
        if (!ctx || !img){
            return;
        }
        ctx.globalAlpha = opacity;
        ctx.drawImage(img, p.x - IconSize / 2, p.y - IconSize / 2, IconSize, IconSize);
        ctx.globalAlpha = 1;
    }
});

/// Icon marker class
///
/// Hacking the CircleMarker class to draw an icon
export class IconMarker extends L.CircleMarker {

    constructor(latlng: L.LatLngExpression, iconSrc: string, opacity: number, options: L.CircleMarkerOptions = {}) {
        const icon = document.createElement("img");
        icon.src = iconSrc;
        super(latlng, Object.assign(options, { icon, opacity }) as L.CircleMarkerOptions);
    }
    /// Hacking the updatePath function to draw our icon
    _updatePath() {
        const options = this.options as IconMarkerOptions;
        if (!options || !options.icon) {
            console.warn("[map] IconMarker: no icon provided");
            return;
        }
        const renderer = (this as any)._renderer; // eslint-disable-line @typescript-eslint/no-explicit-any
        if (!renderer || !renderer.drawIconMarker) {
            console.warn("[map] IconMarker: invalid renderer");
            return;
        }
        renderer.drawIconMarker(this, options.opacity);
    }

}
