//! useWindowSize Hook
//!
//! This hook gets the window size and listens for changes.
//! It is backed by a context and provider so that components share the same reference,
//! and rerenders when the window size changes.
import React, { useContext } from "react";

/// Window size type
export type WindowSize = {
    windowWidth: number;
    windowHeight: number;
};

/// Context for window size
export const WindowSizeContext = React.createContext<WindowSize>({
    windowWidth: window.innerWidth - 1,
    windowHeight: window.innerHeight - 1,
});

/// Hook for using the WindowSizeContext
export const useWindowSize = () => useContext(WindowSizeContext);
