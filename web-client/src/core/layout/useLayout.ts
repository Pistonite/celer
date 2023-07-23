//! Hooks for managing the layout of the application
import { useSelector } from "react-redux";
import { settingsSelector } from "core/store";

import { Layout, WidgetType } from "./state";
import { isCurrentLayoutDefault } from "./utils";

/// Return type of useLayout hook
export type UseLayout = {
    /// The layout
    layout: Layout;
    /// Widgets in the layout
    widgets: ReactGridLayout.Layout[];
    /// Available locations for the toolbar
    availableToolbarLocations: WidgetType[];
    /// Callback to set the layout from a ReactGridLayout
    setLayout: (widgets: ReactGridLayout.Layout[]) => void;
    /// If the layout is the default layout
    isDefaultLayout: boolean;
};

/// Hook for getting the current layout, or undefined if the layout is the default layout
export const useCurrentUserLayout = (): Layout | undefined => {
    const settings = useSelector(settingsSelector);
    if (isCurrentLayoutDefault(settings)) {
        return undefined;
    }
    const { currentLayout, savedLayouts } = settings;
    return savedLayouts[currentLayout];
};
