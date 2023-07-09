//! IconMarker is used to draw icons on the map

import L from "leaflet";
import { LRUCache } from "lru-cache";

import { MapLog } from "./util";

// hacks into implementation details of leaflet
interface LLayer {
    _empty?(): boolean;
    _point?: L.Point;
    _renderer?: L.Canvas & {
        _ctx?: CanvasRenderingContext2D;
    };
}

/// Cache for the icon images. Uses LRU cache to limit memory usage
///
/// The URL of the icon is used as the key
const IconCache = new LRUCache<string, Icon>({
    max: 64,
});

/// Get the icon from the cache or create a new one
const getIcon = (iconSrc: string): Icon => {
    const icon = IconCache.get(iconSrc);
    if (icon) {
        return icon;
    }
    const img = document.createElement("img");
    img.src = iconSrc;
    const newIcon = {
        img,
        loaded: false,
    };
    img.onload = () => {
        newIcon.loaded = true;
    };
    IconCache.set(iconSrc, newIcon);
    return newIcon;
};

/// Wrapper for HTMLImageElement and a load status
type Icon = {
    /// The image element
    img: HTMLImageElement;
    /// Whether the image is loaded
    loaded: boolean;
};

/// Icon marker class
///
/// Hacking the CircleMarker class to draw an icon
export class IconMarker extends L.CircleMarker {
    /// The icon
    private icon: Icon;
    /// Opacity
    private opacity: number;
    /// Size
    private size: number;

    constructor(
        latlng: L.LatLngExpression,
        iconSrc: string,
        opacity: number,
        size: number,
        options: L.CircleMarkerOptions = {},
    ) {
        super( latlng, options);
        this.icon = getIcon(iconSrc);
        this.opacity = opacity;
        this.size = size;
    }

    /// Hacking the updatePath function to draw our icon
    _updatePath() {
        this.redrawInternal(0);
    }

    /// Draw the icon marker. If the icon is not loaded yet, it will retry later
    private redrawInternal(retryCount: number) {
        if (!this.icon.loaded) {
            if (retryCount > 10) {
                MapLog.warn(`resource from ${this.icon.img.src} is taking too long to load.`);
                return;
            }
            window.setTimeout(() => {
                this.redrawInternal(retryCount + 1);
            }, 500);
            return;
        }

        // check if layer is empty
        const layer = this as unknown as LLayer;
        if (layer._empty && layer._empty()) {
            return;
        }
        const p = layer._point;
        if (!p) {
            MapLog.warn("invalid icon marker point");
            return;
        }

        // check if renderer is valid
        const ctx  = layer._renderer?._ctx; // eslint-disable-line @typescript-eslint/no-explicit-any
        if (!ctx) {
            MapLog.warn("invalid icon markder renderer");
            return;
        }

        ctx.globalAlpha = this.opacity;
        ctx.drawImage(
            this.icon.img,
            p.x - this.size / 2,
            p.y - this.size / 2,
            this.size,
            this.size,
        );
        ctx.globalAlpha = 1;
    }
}
