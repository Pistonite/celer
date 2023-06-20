//! Utilities to get the default layout

import { GridFull, GridHalf, GridMajor, GridMinor, GridThird, Layout } from "./util";

/// Get the default landscape viewer layout
///
/// This is left document, right map, and map is slightly larger.
/// Toolbar is anchored on top of the viewer
export const getDefaultLandscapeViewerLayout = (): Layout => {
    return {
        toolbar: "viewer",
        toolbarAnchor: "top",
        viewer: {
            x: 0,
            y: 0,
            w: GridMinor,
            h: GridFull,
        },
        map: {
            x: GridMinor,
            y: 0,
            w: GridMajor,
            h: GridFull,
        },
    };
}

/// Get the default portrait viewer layout
///
/// This is top map, bottom document, half-half
/// Toolbar is anchored on top of the map
export const getDefaultPortraitViewerLayout = (): Layout => {
    return {
        toolbar: "map",
        toolbarAnchor: "top",
        map: {
            x: 0,
            y: 0,
            w: GridFull,
            h: GridHalf,
        },
        viewer: {
            x: 0,
            y: GridHalf,
            w: GridFull,
            h: GridHalf,
        },
    };
}

/// Get the default landscape editor layout
///
/// This is third-third-third, editor-viewer-map, from left to right.
/// Toolbar is anchored on top of the editor
export const getDefaultLandscapeEditorLayout = (): Layout => {
    return {
        toolbar: "editor",
        toolbarAnchor: "top",
        editor: {
            x: 0,
            y: 0,
            w: GridThird,
            h: GridFull,
        },
        viewer: {
            x: GridThird,
            y: 0,
            w: GridThird,
            h: GridFull,
        },
        map: {
            x: GridThird * 2,
            y: 0,
            w: GridThird,
            h: GridFull,
        },
    };
}

/// Get the default portrait editor layout
///
/// This is third-third-third, map-viewer-editor, from top to bottom.
/// Toolbar is anchored on top of the map
export const getDefaultPortraitEditorLayout = (): Layout => {
    return {
        toolbar: "map",
        toolbarAnchor: "top",
        map: {
            x: 0,
            y: 0,
            w: GridFull,
            h: GridThird,
        },
        viewer: {
            x: 0,
            y: GridThird,
            w: GridFull,
            h: GridThird,
        },
        editor: {
            x: 0,
            y: GridThird * 2,
            w: GridFull,
            h: GridThird,
        },
    };
}

/// Get the default mobile layout
///
/// This is just the document viewer, with toolbar on top
export const getDefaultMobileLayout = (): Layout => {
    return {
        toolbar: "viewer",
        toolbarAnchor: "top",
        viewer: {
            x: 0,
            y: 0,
            w: GridFull,
            h: GridFull,
        },
    };
}

