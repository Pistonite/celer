import React from "react";
import ReactDOM from "react-dom/client";
import { AppRoot } from "./AppRoot.tsx";
import { store } from "data/store";

import "./index.css";
import { Provider as ReduxProvider } from "react-redux";
import { WindowSizeProvider } from "core/utils";
import { prefersDarkMode } from "data/util";
import { FluentProvider, webDarkTheme, webLightTheme } from "@fluentui/react-components";

window.addEventListener("popstate", (event) => {
    console.log(event);
});

const root = ReactDOM.createRoot(
    document.getElementById("react-root") as HTMLElement,
);

const isDarkMode = prefersDarkMode();

root.render(
    <React.StrictMode>
        <ReduxProvider store={store}>
            <WindowSizeProvider>
        <FluentProvider id="style-root" theme={isDarkMode ? webDarkTheme : webLightTheme} >
                    <AppRoot />
        </FluentProvider>
            </WindowSizeProvider>
        </ReduxProvider>
    </React.StrictMode>,
);
