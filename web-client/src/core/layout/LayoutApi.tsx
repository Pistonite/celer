// import { Layout, LocalStore } from "data/store";
// import React from "react";
// import { useState } from "react";
// import ReactDOM from "react-dom/client";

// /// Gridster instance 
// ///
// /// Add methods here as needed :http://dsmorse.github.io/gridster.js/docs/classes/Gridster.html
// type GridsterInstance = {
//     /// Disables dragging
//     disable: () => void;
//     /// Disables resizeing
//     disable_resize: () => void;
//     /// Enables dragging
//     enable: () => void;
//     /// Enables resizeing
//     enable_resize: () => void;
//     /// Resizes widget dimensions
//     //resize_widget_dimensions: (options: GridsterOptions) => void;
//     ///
//     //update_widgets_dimensions: () => void;

//     /// Add new widget
//     add_widget: (html: string, size_x: number, size_y: number, col: number, row: number) => any
//     /// Removes all widgets
//     remove_all_widgets: () => void;
// }

// /// Gridster options
// ///
// /// Add options here as needed. Reference: http://dsmorse.github.io/gridster.js/docs/classes/Gridster.html
// type GridsterOptions = {
//     /// Widget selector
//     widget_selector?: string;
//     /// Margins between widgets
//     widget_margins?: [number, number];
//     /// Unit dimension (excluding margins) of widgets
//     widget_base_dimensions?: [number | "auto", number | "auto"];
//     /// Avoids overlapping widgets
//     avoid_overlapped_widgets?: boolean;
//     /// Resize options
//     resize?: {
//         /// Enables resizing API
//         /// disabling will not initializing the resize functionality, which will cause error when calling.
//         enabled?: boolean;
//     },
//     /// Maximum number of columns
//     max_cols?: number;
//     min_cols?: number;
//     min_rows?: number;
//     max_rows?: number;
//     autogenerate_stylesheet?: boolean;
// }

// /// ReactDOM root shim
// type ReactDOMRoot = {
//     render: (element: React.ReactElement) => void;
//     unmount: () => void;
// }

// /// React render shim
// type ReactRenderer = () => React.ReactElement;

// /// Layout runtime
// type LayoutRuntime = {
//     /// Gridster instance
//     gridster: GridsterInstance;
//     /// If is editing layout
//     editing: boolean;
//     /// Document root
//     document: ReactDOMRoot | null;
//     /// Document renderer
//     documentRenderer: ReactRenderer;
// }

// /// Gridster instance
// let LayoutRt: LayoutRuntime | null = null;
// /// Grid width in number of cells
// const GRID_WIDTH = 16;
// /// Grid height in number of cells
// const GRID_HEIGHT = 16;
// /// Margin between cells when editing
// const EDITING_MARGIN = 10;
// /// Maximum width of the window to be considered mobile
// const MOBILE_WIDTH_THRESHOLD = 400;
// /// Layout container ID
// const LAYOUT_CONTAINER_ID = "layout-container";

// /// Initializes the layout system, including the Gridster instance
// export const initializeLayout = (documentRenderer: ReactRenderer) => {
    
//     const options: GridsterOptions = {
//         widget_selector: "div",
//         widget_margins: [10, 10],
//         widget_base_dimensions: ["auto", 20],
//         resize: {
//             enabled: true // Need this to initialize resize functionality
//         },
//         autogenerate_stylesheet: true,
//         min_cols: 1,
//         max_cols: GRID_WIDTH,
//         min_rows: 1,
//         max_rows: GRID_HEIGHT,
//     };

//     $(() => {
//         let gridster: GridsterInstance = ($(`div#${LAYOUT_CONTAINER_ID}`) as any).gridster(options).data("gridster");
//         // gridster.disable();
//         // gridster.disable_resize();

//         //window.addEventListener("resize", onWindowResize);

//         LayoutRt = {
//             gridster,
//             document: null,
//             editing: false,
//             documentRenderer
//         };

//         mountCurrentLayout(LayoutRt);
//     });
// }

// const onWindowResize = (e: any) => {
//     //const {innerWidth, innerHeight} = (e?.currentTarget || window) as Window;
//     //const margin = LayoutRt?.editing ? EDITING_MARGIN : 0;
//     // LayoutRt?.gridster.resize_widget_dimensions({
//     //     widget_margins: [0, 0],
//     //     widget_base_dimensions: getWidgetSize(innerWidth, innerHeight, 0),
//     // });
//     //LayoutRt?.gridster.update_widgets_dimensions();

// }

// const mountCurrentLayout = (runtime: LayoutRuntime) => {
//     const layoutIndex = LocalStore.CurrentLayout;
//     const layout = layoutIndex && layoutIndex < LocalStore.SavedLayouts.length ? LocalStore.SavedLayouts[layoutIndex] : getDefaultViewerLayout(window.innerWidth, window.innerHeight);
//     mountLayout(runtime, layout);
// }

// const mountLayout = (runtime: LayoutRuntime, layout: Layout) => {
//     console.log(layout);
//     // unmount current layout
//     runtime.document?.unmount();
//     runtime.gridster.remove_all_widgets();
//     // create new layout
//     layout.widgets.forEach((widget) => {
//         switch(widget.type) {
//             case "document":
//                 runtime.gridster.add_widget(`<div><div id="react-document"></div></div>`, widget.width, widget.height, widget.x+1, widget.y+1);
//                 break;
//             case "map":
//                 runtime.gridster.add_widget(`<div><div id="leaflet-map"></div></div>`, widget.width, widget.height, widget.x+1, widget.y+1);
//                 break;
//         }
//     });

//     // runtime.gridster.disable();
//     // runtime.gridster.disable_resize();

//     // const resizeCallback = () => {
//     //     runtime.gridster.resize_widget_dimensions({
//     //         widget_margins: [0, 0],
//     //         widget_base_dimensions: getWidgetSize(window.innerWidth, window.innerHeight, 0),
//     //     });
//     //     runtime.gridster.update_widgets_dimensions();
//     // }
//     //setTimeout(resizeCallback, 0);

//     // mount components on new layout
//     const mountDocument = (tries: number) => {
//         if (tries > 10) {
//             console.log("documentRoot not found after 10 tries")
//             return;
//         }
//         const documentRoot = document.getElementById("react-document");
//         if (!documentRoot) {
//             console.log("documentRoot not found")
//             setTimeout(() => mountDocument(tries+1), 10);
//             return;
//         }
//         console.log("rendering document")

//         runtime.document = ReactDOM.createRoot(documentRoot);
//         runtime.document.render(runtime.documentRenderer());
//         //setTimeout(resizeCallback, 10);
//         //resizeCallback();
//     };
//     mountDocument(0);
//     onWindowResize(null);
    
// }

// /// Compute widget size based on window size and margins
// const getWidgetSize = (windowWidth: number, windowHeight: number, margin: number): [number, number] => {
//     // x cells means x+1 margins
//     const unitWidth = (windowWidth - margin * (GRID_WIDTH + 1)) / GRID_WIDTH;
//     const unitHeight = (windowHeight - margin * (GRID_HEIGHT + 1)) / GRID_HEIGHT;
//     console.log(unitWidth, unitHeight)
//     return [unitWidth, unitHeight];
// }

// export const setEditingEnabled = (enabled: boolean) => {
//     if(!LayoutRt) {
//         console.warn("setEditingEnabled called before Gridster is initialized");
//         return;
//     }
//     LayoutRt.editing = enabled;
//     onWindowResize(null);
//     // if (Gridster) {
//     //     console.log("destroy gridster", Gridster);
//     //     Gridster.disable();
//     //     Gridster.disable_resize();
//     //     Gridster.destroy();
//     //     Gridster=null;

//     // } 
//     // if (!Gridster) {
//     //     console.warn("setEditingEnabled called before Gridster is initialized");
//     //     return;
//     // }
//     //if (enabled) {
//         //LayoutRt.gridster.enable();
//         //LayoutRt.gridster.enable_resize();
//         // LayoutRt?.gridster.resize_widget_dimensions({
//         //     widget_margins: [10, 10],
//         //     widget_base_dimensions: getWidgetSize(window.innerWidth, window.innerHeight, 10),
//         // });
//         // Gridster.resize_widget_dimensions({
//         //     widget_margins: [10, 10],
//         //     widget_base_dimensions: [140, 140],

//         // });
//     //} else {
//         // LayoutRt?.gridster.resize_widget_dimensions({
//         //     widget_margins: [0, 0],
//         //     widget_base_dimensions: getWidgetSize(window.innerWidth, window.innerHeight, 0),
//         // });
//         //Gridster.options.widget_margins = [0, 0];
//         // Gridster.resize_widget_dimensions({
//         //     widget_margins: [0, 0],
//         //     widget_base_dimensions: [140, 140],

//         // });
//         //LayoutRt.gridster.disable();
//         //LayoutRt.gridster.disable_resize();
//         // Gridster = ($(".gridster ul") as any).gridster({
//         //     widget_margins: [0, 0],
//         //           widget_base_dimensions: [140, 140],
//         //           resize: {
//         //             enabled: false
//         //           }
//         //   }).data("gridster");
//         //   Gridster.disable();
//     //}
// }

// export const useLayoutApi = () => {
//     const [editing, setEditing] = useState(false);
// }

