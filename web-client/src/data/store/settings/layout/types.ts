//! Layout store types

/// Layout settings. This is part of the SettingsStore
export type LayoutSettings = {
    /// Viewer layouts saved by the user
    savedViewerLayouts: Layout[];
    /// Editor layouts saved by the user
    savedEditorLayouts: Layout[];
    /// The current viewer layout index
    currentViewerLayout: number;
    /// The current editor layout index
    currentEditorLayout: number;
}

/// App Layout
export type Layout = {
    /// Widgets in the layout
    widgets: Widget[];
    /// Which widget is the toolbar displaying
    toolbar: number;
    /// If toolbar is anchored to top or bottom
    toolbarAnchor: "top" | "bottom";
}

/// Type of a widget
export type WidgetType = "document" | "map" | "editor";

/// Widget data
export type Widget = {
    /// Type of the widget
    type: WidgetType;
    /// X position in the grid
    x: number;
    /// Y position in the grid
    y: number;
    /// Width in the grid
    w: number;
    /// Height in the grid
    h: number;
}

/// Default layout settings
export const initialLayoutSettings: LayoutSettings = {
    savedViewerLayouts: [],
    savedEditorLayouts: [],
    currentViewerLayout: 0,
    currentEditorLayout: 0,
};