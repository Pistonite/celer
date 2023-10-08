//! Header utility

import { MenuProps } from "@fluentui/react-components";

/// Type for defining the control groups and controls in the header
export type HeaderControlList = {
    /// The group priority in the overflow. Higher = overflow later
    priority: number;
    /// The controls in the group
    controls: ToolbarControl[];
}[];

/// Common toolbar control type
export type ToolbarControl = {
    /// The control component as toobar button
    ///
    /// This component needs to have forward ref to support the Overflow fluentui component
    ToolbarButton: React.ComponentType;
    /// The control component as a menu item
    ///
    /// Rendered when the toolbar overflows
    MenuItem: React.ComponentType;
    /// Priority of this control.
    /// 
    /// This is added on top of the group priority
    priority?: number;
};

/// Common toolbar control props
export type ControlComponentProps = {
    /// The element to render as the control (e.g. <ToolbarButton /> or <MenuItem />)
    children: React.ReactElement;
};

/// Fluentui menu checked value change function
export type OnMenuCheckedValueChangeFunction =
    MenuProps["onCheckedValueChange"];
