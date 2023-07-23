import "react-grid-layout/css/styles.css";
import "./AppRoot.css";

import clsx from "clsx";
import React, { Suspense, useCallback, useMemo } from "react";
import { useSelector } from "react-redux";
import ReactGridLayout from "react-grid-layout";
import { Header } from "ui/toolbar";
import { LoadScreen, useWindowSize } from "ui/shared";
import { settingsActions, viewSelector } from "core/store";
import { GridFull, Layout, WidgetType, WidgetTypes, getDefaultLayout, useCurrentUserLayout } from "core/layout";
import { useActions } from "low/store";
const Map: React.FC = React.lazy(() => import("ui/map"));
const Doc: React.FC = React.lazy(() => import("ui/doc"));

/// Margin to show when editing the layout
const LayoutEditingMargin = 5;

/// Root of the application
export const AppRoot: React.FC = () => {
    const { isEditingLayout } = useSelector(viewSelector);
    const { windowWidth, windowHeight } = useWindowSize();
    const { widgets, layout, setLayout } = useReactGridLayout(windowWidth, windowHeight);
    const margin = isEditingLayout ? LayoutEditingMargin : 0;

    // compute layout numbers
    const rowHeight = (windowHeight - (GridFull + 1) * margin) / GridFull;

    return (
        <ReactGridLayout
            className="layout-root"
            layout={widgets}
            cols={GridFull}
            width={windowWidth}
            rowHeight={rowHeight}
            isResizable={isEditingLayout}
            isDraggable={isEditingLayout}
            margin={[margin, margin]}
            onLayoutChange={setLayout}
        >
            {widgets.map((widget) => (
                <div
                    className={clsx(
                        "widget-container",
                        isEditingLayout && "editing",
                        `widget-toolbar-${layout.toolbarAnchor}`,
                    )}
                    key={widget.i}
                >
                    {layout.toolbar === widget.i && (
                        <Header toolbarAnchor={layout.toolbarAnchor} />
                    )}
                    <div className="widget">
                        {widget.i === "map" && (
                            <Suspense fallback={<LoadScreen color="green" />}>
                                <Map />
                            </Suspense>
                        )}
                        {widget.i === "viewer" && (
                            <Suspense fallback={<LoadScreen color="yellow" />}>
                                <Doc />
                            </Suspense>
                        )}
                        {widget.i === "editor" && <div>I am a editor</div>}
                    </div>
                </div>
            ))}
        </ReactGridLayout>
    );
};

/// Helper hook for converting between internal layout data and react-grid-layout data
const useReactGridLayout = (windowWidth: number, windowHeight: number) => {
    const userLayout = useCurrentUserLayout();
    const { setCurrentLayout } = useActions(settingsActions);

    // convert layout to ReactGridLayout
    const [layout, widgets] = useMemo(() => {
        const layout = userLayout || getDefaultLayout(windowWidth, windowHeight);
        const widgets = WidgetTypes.map((type) => {
            return layout[type] && { i: type, ...layout[type] };

        }).filter(Boolean) as ReactGridLayout.Layout[];
        return [layout, widgets];
    }, [
        userLayout,
        windowWidth,
        windowHeight,
    ]);

    const { toolbar, toolbarAnchor } = layout;

    const setLayout = useCallback(
        (widgets: ReactGridLayout.Layout[]) => {
            const layout: Layout = {
                toolbar,
                toolbarAnchor,
            };

            widgets.forEach((widget) => {
                const type = widget.i;
                if ((WidgetTypes as string[]).includes(type)) {
                    layout[type as WidgetType] = {
                        x: widget.x,
                        y: widget.y,
                        w: widget.w,
                        h: widget.h,
                    };
                }
            });

            setCurrentLayout(layout);
        },
        [toolbar, toolbarAnchor, setCurrentLayout],
    );

    return {
        layout,
        widgets,
        setLayout,
    };
};
