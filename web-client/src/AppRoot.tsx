import clsx from "clsx";
import React, { Suspense } from "react";
import { useSelector } from "react-redux";
import ReactGridLayout from "react-grid-layout";

import "react-grid-layout/css/styles.css";
import "./layout.css";

import { Header } from "ui/surfaces";
const Map: React.FC = React.lazy(() => import("ui/map"));
import { DocLine, LoadScreen } from "ui/components";

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
                        {widget.i === "viewer" && <div>
                            <DocLine 
                                selected={true}
                                mode="normal"
                                lineColor="red"
                                iconUrl="https://icons.pistonite.org/icon/shrine.shrine.none.69a2d5.c1fefe.69a2d5.c1fefe.69a2d5.c1fefe.png"
                                text={[{
                                    text: "Kaya Wan",
                                }]}
                                counter={{
                                    style: {
                                        background: "cyan",
                                        color: "black",
                                    },
                                    text: "9999",
                                }}
                            />
                            <DocLine 
                                selected={false}
                                mode="normal"
                                lineColor="red"
                                iconUrl="https://icons.pistonite.org/icon/shrine.shrine.none.69a2d5.c1fefe.69a2d5.c1fefe.69a2d5.c1fefe.png"
                                text={[{
                                    text: "Kaya Wan",
                                }]}
                                counter={{
                                    style: {
                                        background: "orange",
                                        color: "inherit",
                                    },
                                    text: "split",
                                }}
                            />
                            <DocLine 
                                selected={false}
                                mode="normal"
                                lineColor="red"
                                iconUrl="https://icons.pistonite.org/icon/shrine.shrine.none.69a2d5.c1fefe.69a2d5.c1fefe.69a2d5.c1fefe.png"
                                text={[{
                                    text: "Kaya Wan",
                                }]}
                                secondaryText={[{
                                    text: "do this then do ",
                                }, {
                                    text: "that",
                                    tag: {
                                        bold: true,
                                        italic: true,
                                        }
                                }, {
                                    text: " and do this",
                                        tag: {
                                            color: "red",
                                        }
                                }]}
                            />
                            <DocLine 
                                selected={true}
                                mode="normal"
                                lineColor="#00ff00"
                                text={[{
                                    text: "Kaya Wan",
                                }]}
                                secondaryText={[{
                                    text: "Core inside short short speak speak talk talkt asdfasd asd lgkjalsd",
                                }]}
                            />
                            <DocLine 
                                selected={false}
                                mode="normal"
                                lineColor="red"
                                text={[{
                                    text: "Hello long long long long long long long long long long long ",
                                    tag: {
                                        bold: false,
                                        italic: false,
                                        underline: false,
                                        strikethrough: false,
                                        // color: "black",
                                        // backgroundColor: "white",
                                        link: undefined,
                                    },
                                }]}
                            />
                        </div>}
                        {widget.i === "editor" && <div>I am a editor</div>}
                    </div>
                </div>
            ))}
        </ReactGridLayout>
    );
};
