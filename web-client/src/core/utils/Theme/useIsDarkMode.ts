//! Dark mode context and hook

import React from "react";

/// The context
export const DarkModeContext = React.createContext(false);
/// The hook
export const useIsDarkMode = () => React.useContext(DarkModeContext);

