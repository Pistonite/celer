import "react-grid-layout/css/styles.css";

import React, { Suspense, useCallback, useMemo } from "react";
import { useSelector } from "react-redux";
import ReactGridLayout from "react-grid-layout";
import { mergeClasses } from "@fluentui/react-components";

import { Header } from "ui/toolbar";
import { LoadScreen, useWindowSize } from "ui/shared";
import { settingsActions, settingsSelector, viewSelector } from "core/store";
import {
    GridFull,
    Layout,
    WidgetType,
    WidgetTypes,
    getDefaultLayout,
    useCurrentUserLayout,
} from "core/layout";
import { useActions } from "low/store";

import { AppAlert } from "./AppAlert";
import { useAppStyles } from "./styles";

const Map: React.FC = React.lazy(() => import("ui/map"));
const Doc: React.FC = React.lazy(() => import("ui/doc"));
const Editor: React.FC = React.lazy(() => import("ui/editor"));

/// Margin to show when editing the layout
const LayoutEditingMargin = 5;

/// Root of the application
export const AppRoot: React.FC = () => {
    const { isEditingLayout } = useSelector(viewSelector);
    const { windowWidth, windowHeight } = useWindowSize();
    const { widgets, layout, setLayout } = useReactGridLayout(
        windowWidth,
        windowHeight,
    );
    const margin = isEditingLayout ? LayoutEditingMargin : 0;

    // compute layout numbers
    const rowHeight = (windowHeight - (GridFull + 1) * margin) / GridFull;

    const styles = useAppStyles();

    return (
        <>
            <ReactGridLayout
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
                        className={mergeClasses(
                            styles.widgetContainer,
                            // "widget-container",
                            isEditingLayout && styles.widgetContainerEditing, //"editing",
                            layout.toolbarAnchor === "top"
                                ? styles.widgetToolbarTop
                                : styles.widgetToolbarBottom,
                            // `widget-toolbar-${layout.toolbarAnchor}`,
                        )}
                        key={widget.i}
                    >
                        {layout.toolbar === widget.i && (
                            <Header toolbarAnchor={layout.toolbarAnchor} />
                        )}
                        <div className={styles.widget}>
                            {widget.i === "map" && (
                                <Suspense
                                    fallback={<LoadScreen color="green" />}
                                >
                                    <Map />
                                </Suspense>
                            )}
                            {widget.i === "viewer" && (
                                <Suspense
                                    fallback={<LoadScreen color="yellow" />}
                                >
                                    <Doc />
                                </Suspense>
                            )}
                            {widget.i === "editor" && (
                                <Suspense
                                    fallback={<LoadScreen color="blue" />}
                                >
                                    <Editor />
                                </Suspense>
                            )}
                        </div>
                    </div>
                ))}
            </ReactGridLayout>
            <AppAlert />
        </>
    );
};

/// Helper hook for converting between internal layout data and react-grid-layout data
const useReactGridLayout = (windowWidth: number, windowHeight: number) => {
    const userLayout = useCurrentUserLayout();
    const { setCurrentLayout } = useActions(settingsActions);
    const { rootPath, stageMode } = useSelector(viewSelector);
    const { editorMode } = useSelector(settingsSelector);

    // show editor layout if:
    // - in edit mode, and
    // - one of:
    //   - using web editor
    //   - using external editor, but no project opened yet
    //     (this is because in Firefox, only drag and drop can create a DirectoryEntry
    const isDefaultLayoutEditor =
        stageMode === "edit" &&
        (editorMode === "web" || rootPath === undefined);

    // convert layout to ReactGridLayout
    const [layout, widgets] = useMemo(() => {
        const layout =
            userLayout ||
            getDefaultLayout(windowWidth, windowHeight, isDefaultLayoutEditor);
        const widgets = WidgetTypes.map((type) => {
            return layout[type] && { i: type, ...layout[type] };
        }).filter(Boolean) as ReactGridLayout.Layout[];
        return [layout, widgets];
    }, [userLayout, windowWidth, windowHeight, isDefaultLayoutEditor]);

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
