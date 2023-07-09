//! Layout settings state

import { Layout } from "./layoutUtil";

/// Layout settings. This is part of the SettingsStore
export type LayoutSettings = {
    /// Layouts saved by the user
    savedLayouts: Layout[];
    /// Curreent layout index
    ///
    /// 0-N where N is the length of savedLayouts would refer to a saved layout.
    /// Otherwise, this refers to the default layout.
    currentLayout: number;
};

/// Default layout settings
export const initialLayoutSettings: LayoutSettings = {
    savedLayouts: [],
    currentLayout: -1,
};
