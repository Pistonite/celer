//! ui/app
//!
//! Root of the react ui app

import React from "react";
import ReactDOM from "react-dom/client";
import { Provider as ReduxProvider } from "react-redux";

import { ErrorBoundary } from "ui/shared";
import type { AppStore } from "core/store";
import type { Kernel } from "core/kernel";
import { KernelContext } from "core/kernel";
import { console } from "low/utils";

import { AppRoot } from "./AppRoot";
import { AppErrorBoundary } from "./AppErrorBoundary";
import { FluentProviderWrapper } from "./FluentProviderWrapper";
import { ReactRootDiv } from "./dom";

/// Mount the react app root
///
/// Returns a function to unmount the ui
export const initAppRoot = (
    /// The kernel
    kernel: Kernel,
    /// The redux store
    store: AppStore,
) => {
    const rootDiv = ReactRootDiv.get();
    if (!rootDiv) {
        console.error("Root div not found");
        return () => undefined;
    }
    const root = ReactDOM.createRoot(rootDiv);
    root.render(
        <React.StrictMode>
            <AppErrorBoundary>
                <KernelContext.Provider value={kernel}>
                    <ReduxProvider store={store}>
                        <FluentProviderWrapper>
                            <ErrorBoundary>
                                <AppRoot />
                            </ErrorBoundary>
                        </FluentProviderWrapper>
                    </ReduxProvider>
                </KernelContext.Provider>
            </AppErrorBoundary>
        </React.StrictMode>,
    );

    return () => root.unmount();
};
