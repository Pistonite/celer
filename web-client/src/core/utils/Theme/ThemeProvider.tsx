//! ThemeProvider component
//!
//! Wraps FluentProvider and provides dark mode context
import React, { PropsWithChildren } from "react";
import {
    FluentProvider,
    webDarkTheme,
    webLightTheme,
} from "@fluentui/react-components";

import { DarkModeContext } from "./useIsDarkMode";

/// Theme provider
export const ThemeProvider: React.FC<PropsWithChildren> = ({ children }) => {
    const isDarkMode =
        window.matchMedia &&
        window.matchMedia("(prefers-color-scheme: dark)").matches;
    return (
        <FluentProvider
            id="style-root"
            theme={isDarkMode ? webDarkTheme : webLightTheme}
        >
            <DarkModeContext.Provider value={isDarkMode}>
                {children}
            </DarkModeContext.Provider>
        </FluentProvider>
    );
};
