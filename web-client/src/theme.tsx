import { FluentProvider, webDarkTheme, webLightTheme } from "@fluentui/react-components";
import React, { PropsWithChildren } from "react";

const DarkModeContext = React.createContext(false);

/// Theme provider
export const ThemeProvider: React.FC<PropsWithChildren> = ({children}) => {
    const isDarkMode = window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches;
    return (
        <FluentProvider theme={isDarkMode ? webDarkTheme : webLightTheme}>
            <DarkModeContext.Provider value={isDarkMode}>
                {children}
            </DarkModeContext.Provider>
        </FluentProvider>
    )
}

export const useIsDarkMode = () => React.useContext(DarkModeContext);