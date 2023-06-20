//! useLayout hook
//!
//! This binds the layout store and layout editing/switching UI

import { useCallback, useMemo } from "react";
import { useSelector } from "react-redux";
import {
    Layout,
    WidgetType,
    getDefaultLandscapeViewerLayout,
    getDefaultMobileLayout,
    getDefaultPortraitViewerLayout,
    settingsActions,
    settingsSelector,
    useActions
} from "data/store";
import { useWindowSize } from "./useWindowSize";

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
}

const WidgetTypes: WidgetType[] = ["viewer", "editor", "map"];

export const useLayout = (): UseLayout => {
    const { currentLayout, savedLayouts } = useSelector(settingsSelector);
    const { windowWidth, windowHeight } = useWindowSize();
    const { setCurrentLayout } = useActions(settingsActions);
    const isDefaultLayout = currentLayout < 0 || currentLayout >= savedLayouts.length;

    // convert layout to ReactGridLayout
    const [layout, widgets, availableToolbarLocations] = useMemo(() => {
        const theLayout = isDefaultLayout ? getDefaultLayout(windowWidth, windowHeight) : savedLayouts[currentLayout];
        const widgets: ReactGridLayout.Layout[] = [];
        const locations: WidgetType[] = []
        WidgetTypes.forEach((type) => {
            const dim = theLayout[type];
            if (dim) {
                widgets.push({ i: type, ...dim });
                locations.push(type);
            }
        });
        return [theLayout, widgets, locations];
    }, [currentLayout, savedLayouts, windowWidth, windowHeight]);

    const { toolbar, toolbarAnchor } = layout;

    const setLayout = useCallback((widgets: ReactGridLayout.Layout[]) => {
        const layout: Layout = {
            toolbar,
            toolbarAnchor,
        };
    
        widgets.forEach((widget) => {
            const type = widget.i;
            if ((WidgetTypes as string[]).includes(type)) {
                layout[type as WidgetType] = { x: widget.x, y: widget.y, w: widget.w, h: widget.h }
            }
        });

        setCurrentLayout({layout});
    }, [toolbar, toolbarAnchor]);


    return {
        layout,
        widgets,
        availableToolbarLocations,
        setLayout,
        isDefaultLayout
    }
}

const MobileWidthThreshold = 400;


const getDefaultLayout = (windowWidth: number, windowHeight: number): Layout => {
    // TODO: check editor mode StageStore
    if (windowWidth <= MobileWidthThreshold) {
        return getDefaultMobileLayout();
    }
    if (windowWidth > windowHeight) {
        return getDefaultLandscapeViewerLayout();
    }
    return getDefaultPortraitViewerLayout();
}
