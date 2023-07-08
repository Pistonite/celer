//! Part of the map that manages icons, markers and lines on the map

import L from "leaflet";

import { LayerMode, SectionMode, SettingsStore, ViewStore, VisualSize } from "data/store";
import { ExecDoc, MapIcon } from "data/model";

import { MapLog } from "./util";
import { MapLayerMgr } from "./MapLayerMgr";
import { IconMarker } from "./IconMarker";

/// Class to manage the state of icons, markers and lines on the map
export class MapVisualMgr {
    private icons: MapVisualGroup = new MapVisualGroup([]);
    private lines: MapVisualGroup = new MapVisualGroup([]);
    private markers: MapVisualGroup = new MapVisualGroup([]);

    /// Recreate the layers and update the visuals
    ///
    /// Use this option if the document has changed, or the map layer has changed,
    /// or the settings have changed.
    public recreate(
        map: L.Map, 
        layerMgr: MapLayerMgr,
        doc: ExecDoc, 
        view: ViewStore, 
        settings: SettingsStore,
    ) {
        MapLog.info("initializing map visuals");
        this.initIcons(map, layerMgr, doc, settings);
        this.update(map, view, settings);
    }

    /// Update the visuals on the map according to the state without recreating the layers.
    public update(
        map: L.Map, 
        view: ViewStore, 
        settings: SettingsStore,
    ) {
        this.icons.setActiveSection(map, settings.iconSectionMode, view.currentSection);
        this.lines.setActiveSection(map, settings.lineSectionMode, view.currentSection);
        this.markers.setActiveSection(map, settings.markerSectionMode, view.currentSection);
    }

    /// Initialize the visuals on the map
    private initIcons(
        map: L.Map, 
        layerMgr: MapLayerMgr,
        doc: ExecDoc, 
        settings: SettingsStore,
    ) {
        // remove current icons
        this.icons.removeAll(map);
        const registeredIcons = doc.project.icons;

        const layerGroups = doc.map.map(({ icons }) => {
            const iconMarkers = icons.map((icon) => {
                const iconSrc = registeredIcons[icon.id];
                if (!iconSrc) {
                    MapLog.warn(`Icon ${icon.id} is not registered`);
                    return undefined;
                }
                const size = getIconSizeWithSettings(icon, settings);
                if (!size) {
                    return undefined;
                }

                const layer = getIconLayerWithSettings(
                    icon,
                    layerMgr,
                    settings,
                )
                // note that 0 is a valid layer index
                if (layer === undefined) {
                    // icon is not on a layer that should be visible
                    return undefined;
                }

                // get the opacity of the icon
                const opacity =
                    layer === layerMgr.getActiveLayerIndex() ? 1 : 0.5;
                const [latlng] = layerMgr.unproject(icon.coord);

                const iconMarker = new IconMarker(latlng, iconSrc, opacity, size);
                iconMarker.on("click", () => {
                    console.log("clicked icon, line " + icon.lineNumber );
                });
                return iconMarker;
            }).filter(Boolean) as IconMarker[];
            return L.layerGroup(iconMarkers);
        });

        this.icons = new MapVisualGroup(layerGroups);
    }

}

/// Class to manage one type of visual
class MapVisualGroup {
    /// Visuals for each section
    private sectionLayers: L.LayerGroup[];
    /// Current active section mode
    private sectionMode: SectionMode;
    /// Current active section index
    private sectionIndex: number;

    constructor(layers: L.LayerGroup[]) {
        this.sectionLayers = layers;
        this.sectionMode = SectionMode.None;
        this.sectionIndex = -1;
    }

    /// Set the current visible section based on the mode
    ///
    /// If `mode` is `Current`, then `i` is the index of the current section.
    public setActiveSection(map: L.Map, mode: SectionMode, i?: number) {
        if (this.sectionMode === mode) {
            // Nothing to do if the mode is the same
            if (mode !== SectionMode.Current || i === this.sectionIndex) {
                return;
            }
        } 
        this.removeAll(map);
        if (mode === SectionMode.Current && (i === undefined || i < 0 || i >= this.sectionLayers.length)) {
            // Index is invalid, we will keep the map empty
            MapLog.warn("Invalid section index: " + i);
            return;
        }
        if (mode === SectionMode.None) {
            return;
        }
        // add the new layer
        this.sectionMode = mode;
        if (mode === SectionMode.Current) {
            this.sectionIndex = i as number;
            this.sectionLayers[i as number].addTo(map);
        } else {
            this.sectionLayers.forEach((layer) => {
                layer.addTo(map);
            });
        }
    }

    /// Remove all visuals from the map
    public removeAll(map: L.Map) {
        this.sectionLayers.forEach((layer) => {
            map.removeLayer(layer);
        });
        this.sectionMode = SectionMode.None;
        this.sectionIndex = -1;
    }
}

/// Get the layer index for an icon based on the settings and the icon's z coordinate
///
/// returns undefined if the layer cannot be resolved, or icons on that layer should not 
/// be visible based on the settings
const getIconLayerWithSettings = (icon: MapIcon, layerMgr: MapLayerMgr, settings: SettingsStore): number | undefined => {
    const z = icon.coord[2];
    switch (settings.iconLayerMode) {
        case LayerMode.CurrentOnly:
            return layerMgr.getLayerByZInRelativeRange(z, 0, 0);
        case LayerMode.CurrentAndAdjacent:
            return layerMgr.getLayerByZInRelativeRange(z, -1, 1);
        case LayerMode.All:
            return layerMgr.getLayerByZ(z);
    }
};

/// Get icon size based on the priority and the settings
///
/// Returns the size in pixels, or 0 if is size is `Hidden`
const getIconSizeWithSettings = (icon: MapIcon, settings: SettingsStore): number => {
    let size = settings.otherIconSize;
    switch (icon.priority) {
        case 0:
            size = settings.primaryIconSize;
            break;
        case 1:
            size = settings.secondaryIconSize;
            break;
    }
    switch (size) {
        case VisualSize.Small:
            return 24;
        case VisualSize.Regular:
            return 32;
        case VisualSize.Large:
            return 48;
        case VisualSize.Hidden:
            return 0;
    }
};
