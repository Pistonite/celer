//! Widget component
//!
//! A widget component is a component with an optional toolbar and the content
//! The toolbar can be anchored to the top or bottom of the widget

import { PropsWithChildren } from "react";

/// Widget component props
export type WidgetProps = {
    /// Position of the toolbar. Undefined for no toolbar
    toolbarAnchor?: "top" | "bottom";
};

/// Widget component
export const Widget: React.FC<PropsWithChildren<WidgetProps>> = ({}) => {
};