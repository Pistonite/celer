//! Layout store states

/// Layout view state
///
/// This is part of ViewState and not persisted
export type LayoutViewState = {
    /// If the user is currently editing the layout
    isEditingLayout: boolean;
};

/// Default layout view state
export const initialLayoutViewState: LayoutViewState = {
    isEditingLayout: false,
};

/// Layout settings.
///
/// This is part of SettingsState and persisted in localStorage.
export type LayoutSettingsState = {
    /// Layouts saved by the user
    savedLayouts: Layout[];
    /// Curreent layout index
    ///
    /// 0-N where N is the length of savedLayouts would refer to a saved layout.
    /// Otherwise, this refers to the default layout.
    currentLayout: number;
};

/// Default layout settings
export const initialLayoutSettingsState: LayoutSettingsState = {
    savedLayouts: [],
    currentLayout: -1,
};

/// Layout of the application
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
};

export type WidgetType = Layout["toolbar"];

/// Dimension of a widget
export type LayoutDim = {
    /// X position in the grid
    x: number;
    /// Y position in the grid
    y: number;
    /// Width in the grid
    w: number;
    /// Height in the grid
    h: number;
};
