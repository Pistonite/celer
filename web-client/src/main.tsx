import React from "react";
import ReactDOM from "react-dom/client";
import { AppRoot } from "./AppRoot.tsx";
import { store } from "data/store";

import "./index.css";
import { Provider as ReduxProvider } from "react-redux";
import { ThemeProvider } from "theme.tsx";

window.addEventListener("popstate", (event) => {
    console.log(event);
});

const root = ReactDOM.createRoot(document.getElementById("react-root") as HTMLElement);

root.render(
    <React.StrictMode>
        <ThemeProvider>
            <ReduxProvider store={store}>
                <AppRoot />
            </ReduxProvider>

        </ThemeProvider>
    </React.StrictMode>,
);
