import React, { Suspense, useEffect, useMemo, useState } from "react";
import reactLogo from "./assets/react.svg";
import viteLogo from "/vite.svg";
import ReactGridLayout from "react-grid-layout";
import "react-grid-layout/css/styles.css";
import { useLayout, useWindowSize } from "core/utils";
import { Layout, settingsSelector, GridFull, getDefaultMobileLayout, getDefaultLandscapeViewerLayout, getDefaultPortraitViewerLayout, settingsActions, useActions, LayoutDim, WidgetType, toolbarSelector } from "data/store";
import { Loading } from "Loading";
import { useSelector } from "react-redux";
import { Header } from "ui/surfaces";
import "./layout.css";
import clsx from "clsx";
import { Card } from "@fluentui/react-components";

const Map: React.FC = React.lazy(() => import("ui/map"));


//import "./App.css";

// const ToolBar: React.FC = () => {
//     const [editingLayout, setEditingLayout] = useState(false);
//     const [currentLayout, setCurrentLayout] = useLocalStore("CurrentLayout");

//     return (
//         <div>I am a toolbar
//             current layout {currentLayout}
//             <button onClick={() => {
//                 console.log(editingLayout);
//                 setEditingEnabled(!editingLayout);
//                 setEditingLayout(e => !e);
//             }}>Edit Current Layout</button>
//             <button>Switch Layout</button>
//             <button>Delete Current Layout</button>
//         </div>
//     )
// };

/// Margin to show when editing the layout
const LayoutEditingMargin = 5;

/// Root of the application
///
/// This handles things like layout and routing
export const AppRoot: React.FC = () => {
    const { isEditingLayout } = useSelector(toolbarSelector);
    const {
        widgets,
        layout,
        setLayout,
    } = useLayout();
    const { windowWidth, windowHeight } = useWindowSize();
    const margin = isEditingLayout ? LayoutEditingMargin : 0;
    //const { setCurrentLayout } = useActions(settingsActions);

    // compute layout numbers
    const rowHeight = (windowHeight - (GridFull + 1) * margin) / GridFull;




    // console.log({windowWidth, windowHeight})
    // console.log((windowHeight - (GRID_SIZE+1)*EDITING_MARGIN)/GRID_SIZE);
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
            {
                widgets.map((widget) =>
                    <div className={clsx(
                        "widget-container",
                        isEditingLayout && "editing",
                        `widget-toolbar-${layout.toolbarAnchor}`
                    )} key={widget.i}>
                        {
                            layout.toolbar === widget.i && <Header />
                        }
                        <div className="widget">
                            {
                                widget.i === "map" && <Suspense fallback={<Loading color="green"/>}>
                                    <Map />
                                    </Suspense>
                                    
                            }
                            {
                                widget.i === "viewer" && <div>I am a viewer</div>
                            }
                            {
                                widget.i === "editor" && <div>I am a editor</div>
                            }
                            
                        </div>
                    </div>

                )
            }
        </ReactGridLayout>
    );
}



function App() {
    const [count, setCount] = useState(0);

    useEffect(() => {
        if (count > 1) {
            const load = async () => {
                const module = await import("./core/engine/poc");
                const result = await module.testExec();
                console.log("worker responsed: " + result);
            };
            load();

        }
    }, [count]);

    return (
        <>
            <div>
                <a href="https://vitejs.dev" target="_blank">
                    <img src={viteLogo} className="logo" alt="Vite logo" />
                </a>
                <a href="https://react.dev" target="_blank">
                    <img src={reactLogo} className="logo react" alt="React logo" />
                </a>
            </div>
            <h1>Vite + React</h1>
            {/* <ToolBar /> */}
            <div className="card">
                <button onClick={() => setCount((count) => count + 1)}>
                    count is {count}
                </button>
                <p>
                    Edit <code>src/App.tsx</code> and save to test HMR
                </p>
            </div>
            <p className="read-the-docs">
                Click on the Vite and React logos to learn more
            </p>
            <div style={{
                width: "100%",
                height: "20vh"
            }}>
                <Loading color="yellow" />
            </div>
        </>
    );
}

//export default App;
