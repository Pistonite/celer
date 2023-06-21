//! Layout store types

import { Draft, produce } from "immer";

/// Layout settings. This is part of the SettingsStore
export type LayoutSettings = {
    /// Layouts saved by the user
    savedLayouts: Layout[];
    /// Curreent layout index
    ///
    /// 0-N where N is the length of savedLayouts would refer to a saved layout.
    /// Otherwise, this refers to the default layout.
    currentLayout: number;
}

/// App Layout
export type Layout = {
    /// The layout that has the toolbar
    toolbar: "viewer" | "editor" | "map";
    /// If toolbar is anchored to top or bottom
    toolbarAnchor: "top" | "bottom";
    /// Layout of the route document viewer
    viewer?: LayoutDim;
    /// Layout of the map
    map?: LayoutDim;
    /// Layout of the editor
    editor?: LayoutDim;
}

export type WidgetType = Layout["toolbar"];

/// Widget data
export type LayoutDim = {
    /// X position in the grid
    x: number;
    /// Y position in the grid
    y: number;
    /// Width in the grid
    w: number;
    /// Height in the grid
    h: number;
}

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

/// Default layout settings
export const initialLayoutSettings: LayoutSettings = {
    savedLayouts: [],
    currentLayout: -1
};

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
    });
}


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