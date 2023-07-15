//! Part of the map that manages icons, markers and lines on the map

import L from "leaflet";
import "leaflet-arrowheads";

import {
    LayerMode,
    SectionMode,
    SettingsStore,
    ViewStore,
    VisualSize,
} from "data/store";
import { ExecDoc, GameCoord, MapIcon } from "data/model";

import { MapLog } from "./util";
import { MapLayerMgr } from "./MapLayerMgr";
import { IconMarker } from "./IconMarker";

/// Opacity for non current layer visuals
const NonCurrentLayerOpacity = 0.3;
/// Color for grayed out lines and markers
const GrayedOutColor = "#666666";

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
        this.initIcons(layerMgr, doc, view, settings);
        this.initMarkers(layerMgr, doc, view, settings);
        this.initLines(layerMgr, doc, view, settings);
        this.update(map, view, settings);
    }

    /// Update the visuals on the map according to the state without recreating the layers.
    public update(map: L.Map, view: ViewStore, settings: SettingsStore) {
        // order is important here so icons are always
        // on top of markers and markers on top of lines
        this.lines.setActiveSection(
            map,
            settings.lineSectionMode,
            view.currentSection,
        );
        this.markers.setActiveSection(
            map,
            settings.markerSectionMode,
            view.currentSection,
        );
        this.icons.setActiveSection(
            map,
            settings.iconSectionMode,
            view.currentSection,
        );
    }

    /// Initialize the icons on the map
    private initIcons(
        layerMgr: MapLayerMgr,
        doc: ExecDoc,
        view: ViewStore,
        settings: SettingsStore,
    ) {
        // remove current icons
        this.icons.removeAll();
        const registeredIcons = doc.project.icons;
        const { currentSection } = view;

        const layerGroups = doc.map.map(({ icons }, sectionIndex) => {
            const iconMarkers = icons
                .map((icon) => {
                    const iconSrc = registeredIcons[icon.id];
                    if (!iconSrc) {
                        MapLog.warn(`Icon ${icon.id} is not registered`);
                        return undefined;
                    }
                    const size = getIconSizeWithSettings(icon, settings);
                    if (!size) {
                        return undefined;
                    }

                    const layer = getLayerFromZAndMode(
                        icon.coord[2],
                        layerMgr,
                        settings.iconLayerMode,
                    );
                    // note that 0 is a valid layer index
                    if (layer === undefined) {
                        // icon is not on a layer that should be visible
                        return undefined;
                    }

                    // get the opacity of the icon
                    const opacity = getOpacityForLayer(
                        layer,
                        layerMgr,
                        settings.fadeNonCurrentLayerIcons,
                    );
                    const grayedOut =
                        settings.iconSectionMode ===
                            SectionMode.CurrentHighlight &&
                        sectionIndex !== currentSection;
                    const [latlng] = layerMgr.unproject(icon.coord);

                    const iconMarker = new IconMarker(
                        latlng,
                        iconSrc,
                        opacity,
                        size,
                        grayedOut,
                        {
                            pane: "markerPane",
                        },
                    );
                    iconMarker.on("click", () => {
                        console.log(
                            `clicked icon, section ${icon.sectionIndex}, line ${icon.lineIndex}, layer ${layer}`,
                        );
                    });

                    return iconMarker;
                })
                .filter(Boolean) as IconMarker[];
            return L.layerGroup(iconMarkers);
        });

        this.icons = new MapVisualGroup(layerGroups);
    }

    /// Initialize the markers on the map
    private initMarkers(
        layerMgr: MapLayerMgr,
        doc: ExecDoc,
        view: ViewStore,
        settings: SettingsStore,
    ) {
        // remove current markers
        this.markers.removeAll();

        const size = getMarkerSizeWithSettings(settings);
        if (!size) {
            // Markers are all hidden
            this.markers = new MapVisualGroup([]);
            return;
        }

        const { currentSection } = view;

        const layerGroups = doc.map.map(({ markers }, sectionIndex) => {
            const markerLayers = markers
                .map((marker) => {
                    const layer = getLayerFromZAndMode(
                        marker.coord[2],
                        layerMgr,
                        settings.markerLayerMode,
                    );
                    // note that 0 is a valid layer index
                    if (layer === undefined) {
                        return undefined;
                    }

                    // get the opacity of the marker
                    const strokeOpacity = getOpacityForLayer(
                        layer,
                        layerMgr,
                        settings.fadeNonCurrentLayerMarkers,
                    );
                    const fillOpacity = strokeOpacity === 1 ? 0.5 : 0;
                    const grayedOut =
                        settings.markerSectionMode ===
                            SectionMode.CurrentHighlight &&
                        sectionIndex !== currentSection;

                    const [latlng] = layerMgr.unproject(marker.coord);

                    const markerLayer = L.circleMarker(latlng, {
                        radius: size,
                        weight: 2,
                        color: grayedOut ? GrayedOutColor : marker.color,
                        opacity: strokeOpacity,
                        fillOpacity: fillOpacity,
                        pane: "markerPane",
                    });
                    markerLayer.on("click", () => {
                        console.log(
                            `clicked marker, section ${marker.sectionIndex}, line ${marker.lineIndex}, layer ${layer}`,
                        );
                    });
                    return markerLayer;
                })
                .filter(Boolean) as L.CircleMarker[];
            return L.layerGroup(markerLayers, {
                pane: "markerPane",
            });
        });

        this.markers = new MapVisualGroup(layerGroups);
    }

    /// Init the lines
    private initLines(
        layerMgr: MapLayerMgr,
        doc: ExecDoc,
        view: ViewStore,
        settings: SettingsStore,
    ) {
        // remove current lines
        this.lines.removeAll();

        const size = getLineSizeWithSettings(settings);
        if (!size) {
            this.lines = new MapVisualGroup([]);
            return;
        }
        const arrowSize = getArrowSizeWithSettings(settings);
        const arrowFrequency = getArrowFrequencyWithSettings(settings);
        const { currentSection } = view;

        const layerGroups = doc.map.map(({ lines }, sectionIndex) => {
            // input map line data may contain lines that span multiple layers
            // we need to split them into multiple lines.
            // the input should only span one section, so we don't need to check that

            const polylines: MapPolyline[] = [];

            lines.forEach((line) => {
                // get the opacity of the line
                const grayedOut =
                    settings.lineSectionMode === SectionMode.CurrentHighlight &&
                    sectionIndex !== currentSection;

                let tempCoords: GameCoord[] = [];
                let layer: number | undefined = undefined;

                for (let i = 0; i < line.points.length; i++) {
                    const nextLayer = getLayerFromZAndMode(
                        line.points[i][2],
                        layerMgr,
                        settings.lineLayerMode,
                    );
                    if (nextLayer !== layer) {
                        // layer change
                        if (tempCoords.length > 0 && layer !== undefined) {
                            // add the middle point as the end of this polyline
                            const middleX =
                                (line.points[i][0] +
                                    tempCoords[tempCoords.length - 1][0]) /
                                2;
                            const middleY =
                                (line.points[i][1] +
                                    tempCoords[tempCoords.length - 1][1]) /
                                2;
                            // use the last point's z
                            tempCoords.push([
                                middleX,
                                middleY,
                                tempCoords[tempCoords.length - 1][2],
                            ]);

                            // add the previous line
                            polylines.push({
                                vertices: tempCoords,
                                color: grayedOut ? GrayedOutColor : line.color,
                                opacity: getOpacityForLayer(
                                    layer,
                                    layerMgr,
                                    settings.fadeNonCurrentLayerLines,
                                ),
                                hasArrow: arrowSize > 0 && !grayedOut,
                            });

                            tempCoords = [
                                [middleX, middleY, line.points[i][2]],
                            ];
                        } else {
                            tempCoords = [];
                        }
                        layer = nextLayer;
                    }
                    tempCoords.push(line.points[i]);
                }
                // add last line
                if (tempCoords.length > 1 && layer !== undefined) {
                    polylines.push({
                        vertices: tempCoords,
                        color: grayedOut ? GrayedOutColor : line.color,
                        opacity: getOpacityForLayer(
                            layer,
                            layerMgr,
                            settings.fadeNonCurrentLayerLines,
                        ),
                        hasArrow: arrowSize > 0 && !grayedOut,
                    });
                }
            });

            // convert polyline data to leaflet polylines
            const polylineLayers = polylines.map((polyline) => {
                const latlngs = polyline.vertices.map((coord) => {
                    return layerMgr.unproject(coord)[0];
                });
                const layer = L.polyline(latlngs, {
                    color: polyline.color,
                    opacity: polyline.opacity,
                    weight: size,
                });
                if (polyline.hasArrow) {
                    return layer.arrowheads({
                        size: `${arrowSize}px`,
                        frequency: `${arrowFrequency}px`,
                        fill: polyline.opacity === 1,
                        color: polyline.color,
                        fillOpacity: polyline.opacity,
                    });
                }
                return layer;
            });

            return L.layerGroup(polylineLayers);
        });

        this.lines = new MapVisualGroup(layerGroups);
    }
}

/// Internal type for a polyline data
///
/// Each polyline is only in one section and one layer
/// The vertices contain both start and end
///
/// Color and opacity are computed based on doc view and settings
type MapPolyline = {
    /// vertices
    vertices: GameCoord[];
    /// color
    color: string;
    /// opacity
    opacity: number;
    /// has arrow
    hasArrow: boolean;
};

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
        this.removeAll();
        if (
            mode === SectionMode.Current &&
            (i === undefined || i < 0 || i >= this.sectionLayers.length)
        ) {
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
            // only show current section
            this.sectionIndex = i as number;
            this.sectionLayers[i as number].addTo(map);
        } else {
            // show all sections
            this.sectionLayers.forEach((layer) => {
                layer.addTo(map);
            });
        }
    }

    /// Remove all visuals from the map
    public removeAll() {
        this.sectionLayers.forEach((layer) => {
            layer.remove();
        });
        this.sectionMode = SectionMode.None;
        this.sectionIndex = -1;
    }
}

/// Get the layer index for z index based on the settings
///
/// returns undefined if the layer cannot be resolved, or
/// the layer should not be visible based on the settings
const getLayerFromZAndMode = (
    z: number,
    layerMgr: MapLayerMgr,
    layerMode: LayerMode,
): number | undefined => {
    switch (layerMode) {
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
const getIconSizeWithSettings = (
    icon: MapIcon,
    settings: SettingsStore,
): number => {
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

/// Get marker size based on the settings
///
/// Returns the size in pixels, or 0 if is size is `Hidden`
const getMarkerSizeWithSettings = (settings: SettingsStore): number => {
    switch (settings.markerSize) {
        case VisualSize.Small:
            return 4;
        case VisualSize.Regular:
            return 6;
        case VisualSize.Large:
            return 8;
        case VisualSize.Hidden:
            return 0;
    }
};

/// Get line size based on the settings
///
/// Returns the size in pixels, or 0 if is size is `Hidden`
const getLineSizeWithSettings = (settings: SettingsStore): number => {
    switch (settings.lineSize) {
        case VisualSize.Small:
            return 2;
        case VisualSize.Regular:
            return 3;
        case VisualSize.Large:
            return 5;
        case VisualSize.Hidden:
            return 0;
    }
};

/// Get arrow size based on the settings
///
/// Returns the size in pixels, or 0 if is size is `Hidden`
const getArrowSizeWithSettings = (settings: SettingsStore): number => {
    switch (settings.arrowSize) {
        case VisualSize.Small:
            return 6;
        case VisualSize.Regular:
            return 8;
        case VisualSize.Large:
            return 10;
        case VisualSize.Hidden:
            return 0;
    }
};

const getArrowFrequencyWithSettings = (settings: SettingsStore): number => {
    switch (settings.arrowFrequency) {
        case VisualSize.Small: //dense
            return 50;
        case VisualSize.Regular: //regular
            return 100;
        case VisualSize.Large: //sparse
            return 150;
        case VisualSize.Hidden: //unreachable
            return 0;
    }
};

/// Get the opacity of the layer based on the settings
const getOpacityForLayer = (
    layer: number,
    layerMgr: MapLayerMgr,
    fadeOnNonCurrent: boolean,
): number => {
    if (!fadeOnNonCurrent) {
        return 1;
    }
    return layer === layerMgr.getActiveLayerIndex()
        ? 1
        : NonCurrentLayerOpacity;
};
