import clsx from "clsx";
import React, { Suspense } from "react";
import { useSelector } from "react-redux";
import ReactGridLayout from "react-grid-layout";

import "react-grid-layout/css/styles.css";
import "./layout.css";

import { Header } from "ui/toolbar";
const Map: React.FC = React.lazy(() => import("ui/map"));
const Doc: React.FC = React.lazy(() => import("ui/doc"));
import { LoadScreen } from "ui/shared";

import { useLayout, useWindowSize } from "core/utils";

import { GridFull, viewSelector } from "data/store";

/// Margin to show when editing the layout
const LayoutEditingMargin = 5;

/// Root of the application
///
/// This handles things like layout and routing
export const AppRoot: React.FC = () => {
    const { isEditingLayout } = useSelector(viewSelector);
    const { widgets, layout, setLayout } = useLayout();
    const { windowWidth, windowHeight } = useWindowSize();
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
