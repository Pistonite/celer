//! Utilities for layout logic

import { Draft, produce } from "immer";

import { Layout, LayoutDim, LayoutSettingsState, WidgetType } from "./state";

/// The Grid size
///
/// This means the layout is broken into a GRID_SIZE x GRID_SIZE grid
export const GridFull = 36;
/// 1/2 grid size
export const GridHalf = GridFull / 2;
/// 1/3 grid size
export const GridThird = GridFull / 3;
/// The larger widget size where 2 parts are about the same
export const GridMajor = 20;
/// The smaller widget size where 2 parts are about the same
export const GridMinor = GridFull - GridMajor;
/// Widget types. Key of the layout object
export const WidgetTypes: WidgetType[] = ["viewer", "editor", "map"];

/// Returns a new layout fitted to the grid
///
/// This will make sure the layout does not extend beyond the grid.
/// It will also try to make the layout visible if it is not (i.e. has 0 width or height)
///
/// This will return a new reference even if the layout is not modified.
export const fitLayoutToGrid = (layout: Layout): Layout => {
    return produce(layout, (layout) => {
        layout.viewer && fitLayoutDimToGrid(layout.viewer);
        layout.map && fitLayoutDimToGrid(layout.map);
        layout.editor && fitLayoutDimToGrid(layout.editor);

        if (layout.viewer?.w === 0 || layout.viewer?.h === 0) {
            delete layout.viewer;
        }
        if (layout.editor?.w === 0 || layout.editor?.h === 0) {
            delete layout.editor;
        }
        if (layout.map?.w === 0 || layout.map?.h === 0) {
            delete layout.map;
        }
    });
};

const fitLayoutDimToGrid = (layout: Draft<LayoutDim>) => {
    // Make sure the top of always visible
    if (layout.y < 0) {
        layout.y = 0;
    }
    // Make sure the bottom is always visible
    if (layout.y >= GridFull) {
        layout.y = GridFull;
    }
    if (layout.y + layout.h > GridFull) {
        layout.h = GridFull - layout.y;
    }
    // Make sure the left is always visible
    if (layout.x < 0) {
        layout.x = 0;
    }
    if (layout.x >= GridFull) {
        layout.x = GridFull;
    }
    // Make sure the right is always visible
    if (layout.x + layout.w > GridFull) {
        layout.w = GridFull - layout.x;
    }
    // If the widget has 0 width or height, try to make it visible
    if (layout.w <= 0) {
        layout.w = GridFull - layout.x;
    }
    // If the widget has 0 width or height, try to make it visible
    if (layout.h <= 0) {
        layout.h = GridFull - layout.h;
    }
};

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
};

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
};

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
            w: GridHalf,
            h: GridFull,
        },
        viewer: {
            x: GridHalf,
            y: 0,
            w: GridHalf,
            h: GridHalf,
        },
        map: {
            x: GridHalf,
            y: GridHalf,
            w: GridHalf,
            h: GridHalf,
        },
    };
};

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
};

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
};

const MobileWidthThreshold = 400;

/// Get the default layout for the given window size
export const getDefaultLayout = (
    windowWidth: number,
    windowHeight: number,
    stage: "view" | "edit",
): Layout => {
    const isEditor = stage === "edit";
    if (isEditor) {
        if (windowWidth > windowHeight) {
            return getDefaultLandscapeEditorLayout();
        }
        return getDefaultPortraitEditorLayout();
    } else {
        if (windowWidth <= MobileWidthThreshold) {
            return getDefaultMobileLayout();
        }
        if (windowWidth > windowHeight) {
            return getDefaultLandscapeViewerLayout();
        }
        return getDefaultPortraitViewerLayout();
    }
};

/// If the current layout is the default layout
export const isCurrentLayoutDefault = (state: LayoutSettingsState): boolean => {
    return (
        state.currentLayout < 0 ||
        state.currentLayout >= state.savedLayouts.length
    );
};

/// Get available toolbar locations for a layout
///
/// Returns empty array if layout is undefined
export const getAvailableToolbarLocations = (
    layout: Layout | undefined,
): WidgetType[] => {
    if (!layout) {
        return [];
    }
    return WidgetTypes.map((type) => {
        return layout[type] ? null : type;
    }).filter(Boolean) as WidgetType[];
};
