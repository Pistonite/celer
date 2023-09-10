//! ui/app
//!
//! Root of the react ui app

import React from "react";
import ReactDOM from "react-dom/client";
import {
    FluentProvider,
    webDarkTheme,
    webLightTheme,
} from "@fluentui/react-components";
import { Provider as ReduxProvider } from "react-redux";

import { ErrorBoundary } from "ui/shared";
import type { AppStore } from "core/store";
import { Kernel, KernelContext } from "core/kernel";

import { AppRoot } from "./AppRoot";
import { AppErrorBoundary } from "./AppErrorBoundary";

/// Mount the react app root
///
/// Returns a function to unmount the ui
export const initAppRoot = (
    /// The kernel
    kernel: Kernel,
    /// The redux store
    store: AppStore,
    /// Whether the ui should render in dark mode
    isDarkMode: boolean,
) => {
    const root = ReactDOM.createRoot(
        document.getElementById("react-root") as HTMLElement,
    );
    root.render(
        <React.StrictMode>
            <AppErrorBoundary>
                <KernelContext.Provider value={kernel}>
                    <ReduxProvider store={store}>
                        <FluentProvider
                            id="style-root"
                            theme={isDarkMode ? webDarkTheme : webLightTheme}
                        >
                            <ErrorBoundary>
                                <AppRoot />
                            </ErrorBoundary>
                        </FluentProvider>
                    </ReduxProvider>
                </KernelContext.Provider>
            </AppErrorBoundary>
        </React.StrictMode>,
    );

    return () => root.unmount();
};
