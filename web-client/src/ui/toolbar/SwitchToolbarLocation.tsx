//! Control for switching toolbar location
//!
//! The toolbar location is part of the layout.
//!
//! Toolbar can be attached to any widget and on top of bottom.
//! Moreover, since the default layout cannot be edited, the toolbar
//! location cannot be changed in the default layout.

import React, { forwardRef } from "react";
import {
    Menu,
    MenuDivider,
    MenuItem,
    MenuItemRadio,
    MenuList,
    MenuPopover,
    MenuTrigger,
    ToolbarButton,
    Tooltip,
} from "@fluentui/react-components";
import { Window20Regular } from "@fluentui/react-icons";
import type { WidgetType } from "core/layout";
import {
    getAvailableToolbarLocations,
    useCurrentUserLayout,
} from "core/layout";
import { useActions } from "low/store";
import { settingsActions } from "core/store";

import type {
    ControlComponentProps,
    OnMenuCheckedValueChangeFunction,
    ToolbarControl,
} from "./util";

/// Switch toolbar location control
export const SwitchToolbarLocation: ToolbarControl = {
    ToolbarButton: forwardRef<HTMLButtonElement>((_, ref) => {
        const { disabled, props } = useControlPropsInternal();
        return (
            <SwitchToolbarLocationInternal {...props}>
                <Tooltip
                    content={
                        disabled
                            ? "You cannot change the toolbar location in the default layout"
                            : "Change toolbar location"
                    }
                    relationship="label"
                >
                    <ToolbarButton
                        disabled={disabled}
                        icon={<Window20Regular />}
                        ref={ref}
                    />
                </Tooltip>
            </SwitchToolbarLocationInternal>
        );
    }),
    MenuItem: () => {
        const { disabled, props } = useControlPropsInternal();
        return (
            <SwitchToolbarLocationInternal {...props}>
                <MenuItem disabled={disabled} icon={<Window20Regular />}>
                    Toolbar Location
                </MenuItem>
            </SwitchToolbarLocationInternal>
        );
    },
};

/// Helper to compute props
const useControlPropsInternal = () => {
    const userLayout = useCurrentUserLayout();
    return {
        disabled: !userLayout, // disable changing toolbar in default layout
        props: {
            locations: getAvailableToolbarLocations(userLayout),
            location: userLayout?.toolbar,
            anchor: userLayout?.toolbarAnchor,
        },
    };
};

/// Mapping for widget type to display name
const ToolbarLocations: Record<WidgetType, string> = {
    viewer: "Document",
    map: "Map",
    editor: "Editor",
};

const ToolbarLocationRadioName = "Select a toolbar location";
const ToolbarAnchorRadioName = "Select a toolbar anchor";

/// Internal props for switch toolbar location control
type SwitchToolbarLocationInternalProps = ControlComponentProps & {
    /// The current location
    location?: WidgetType;
    /// The current anchor
    anchor?: "top" | "bottom";
    /// The available toolbar locations
    locations: WidgetType[];
};

/// Internal switch toolbar location control logic
const SwitchToolbarLocationInternal: React.FC<
    SwitchToolbarLocationInternalProps
> = ({ children, location, anchor, locations }) => {
    const { setCurrentLayoutToolbarAnchor, setCurrentLayoutToolbarLocation } =
        useActions(settingsActions);

    // compute which menu items should show as checked
    const toolbarMenuCheckedItems = {
        [ToolbarLocationRadioName]: [location || ""],
        [ToolbarAnchorRadioName]: [anchor || ""],
    };

    // callback to update current layout with new toolbar location
    const onChangeToolbarMenuCheckedItems: OnMenuCheckedValueChangeFunction = (
        _,
        { name, checkedItems },
    ) => {
        switch (name) {
            case ToolbarLocationRadioName:
                setCurrentLayoutToolbarLocation(checkedItems[0] as WidgetType);
                break;
            case ToolbarAnchorRadioName:
                setCurrentLayoutToolbarAnchor(
                    checkedItems[0] as "top" | "bottom",
                );
                break;
        }
    };
    return (
        <Menu
            checkedValues={toolbarMenuCheckedItems}
            onCheckedValueChange={onChangeToolbarMenuCheckedItems}
        >
            <MenuTrigger>{children}</MenuTrigger>
            <MenuPopover>
                <MenuList>
                    {locations.map((location) => (
                        <MenuItemRadio
                            key={location}
                            name={ToolbarLocationRadioName}
                            value={location}
                        >
                            {ToolbarLocations[location]}
                        </MenuItemRadio>
                    ))}

                    <MenuDivider />
                    <MenuItemRadio name={ToolbarAnchorRadioName} value="top">
                        Top
                    </MenuItemRadio>
                    <MenuItemRadio name={ToolbarAnchorRadioName} value="bottom">
                        Bottom
                    </MenuItemRadio>
                </MenuList>
            </MenuPopover>
        </Menu>
    );
};
