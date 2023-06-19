import { useEffect, useState } from "react";
import reactLogo from "./assets/react.svg";
import viteLogo from "/vite.svg";
import GridLayout from "react-grid-layout";
import { useWindowSize } from "core/utils";
import { WidgetType } from "data/store";


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

// const adjustHeight = (layout: Widget) => {
//     // Make sure the top of always visible
//     if (layout.y < 0) {
//         layout.y = 0;
//     }
//     // Make sure the bottom is always visible
//     if (layout.y >= GRID_SIZE) {
//         layout.y = GRID_SIZE;
//     }
//     if (layout.y + layout.h > GRID_SIZE) {
//         layout.h = GRID_SIZE - layout.y;
//     }
//     // Make sure the left is always visible
//     if (layout.x < 0) {
//         layout.x = 0;
//     }
//     if (layout.x >= GRID_SIZE) {
//         layout.x = GRID_SIZE;
//     }
//     // Make sure the right is always visible
//     if (layout.x + layout.w > GRID_SIZE) {
//         layout.w = GRID_SIZE - layout.x;
//     }
//     // If the widget has 0 width or height, try to make it visible
//     if (layout.w <= 0) {
//         layout.w = GRID_SIZE - layout.x;
//     }
//     // If the widget has 0 width or height, try to make it visible
//     if (layout.h <= 0) {
//         layout.h = GRID_SIZE - layout.h;
//     }
//     return layout;
// };

/// Root of the application
///
/// This handles things like layout and routing
export const AppRoot: React.FC = () => {
    // const { currentLayout, isEditingLayout, saveEditingLayout } = useLayoutApi();
    // const {windowWidth, windowHeight} = useWindowSize();
    // const margin = isEditingLayout ? EDITING_MARGIN : 0;
    // console.log({windowWidth, windowHeight})
    // console.log((windowHeight - (GRID_SIZE+1)*EDITING_MARGIN)/GRID_SIZE);
    // return (
    //     <div style={{height: "100vh"}}>
    //                 <GridLayout
    //                     className="layout"
    //                     layout={currentLayout.widgets.map((widget) => ({i: widget.type, ...widget}))}
    //                     cols={GRID_SIZE}
    //                     width={windowWidth}
            
    //                     rowHeight={(windowHeight - (GRID_SIZE+1)*margin)/GRID_SIZE}
    //                     isResizable={isEditingLayout}
    //                     isDraggable={isEditingLayout}
    //                     margin={[margin,margin]}
    //                     onLayoutChange={(widgets) => {
    //                         saveEditingLayout({
    //                             ...currentLayout,
    //                             widgets: widgets.map((widget) => ({...widget, type: widget.i as WidgetType}))
    //                         });
    //                     }}

    //                 >
    //                     {
    //                         currentLayout.widgets.map((widget, i)=>
    //                             <div className="widget" key={widget.type}>
    //                                 I am a {widget.type} widget
    //                             </div>

    //                         )
    //                     }
    //                 </GridLayout>
    //             </div>
    
    return App();
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
            <div className="meter">
    <span style={{width: "25%"}}></span>
  </div>
        </>
    );
}

//export default App;
