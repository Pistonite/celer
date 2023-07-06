//! Control for switching toolbar location
//!
//! The toolbar location is part of the layout.
//!
//! Toolbar can be attached to any widget and on top of bottom.
//! Moreover, since the default layout cannot be edited, the toolbar
//! location cannot be changed in the default layout.

import React from "react";
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

import { useLayout } from "core/utils";
import { Layout, WidgetType, settingsActions, useActions } from "data/store";

import {
    ControlComponentProps,
    OnMenuCheckedValueChangeFunction,
    ToolbarControl,
} from "./util";

/// Switch toolbar location control
export const SwitchToolbarLocation: ToolbarControl = {
    ToolbarButton: React.forwardRef<HTMLButtonElement>((_, ref) => {
        const { layout, availableToolbarLocations, isDefaultLayout } =
            useLayout();
        return (
            <SwitchToolbarLocationInternal
                layout={layout}
                locations={availableToolbarLocations}
            >
                <Tooltip
                    content={
                        isDefaultLayout
                            ? "You cannot change the toolbar location in the default layout"
                            : "Change toolbar location"
                    }
                    relationship="label"
                >
                    <ToolbarButton
                        disabled={isDefaultLayout}
                        icon={<Window20Regular />}
                        ref={ref}
                    />
                </Tooltip>
            </SwitchToolbarLocationInternal>
        );
    }),
    MenuItem: () => {
        const { layout, availableToolbarLocations, isDefaultLayout } =
            useLayout();
        return (
            <SwitchToolbarLocationInternal
                layout={layout}
                locations={availableToolbarLocations}
            >
                <MenuItem disabled={isDefaultLayout} icon={<Window20Regular />}>
                    Toolbar Location
                </MenuItem>
            </SwitchToolbarLocationInternal>
        );
    },
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
    /// The current layout
    layout: Layout;
    /// The available toolbar locations
    locations: WidgetType[];
};

/// Internal switch toolbar location control logic
const SwitchToolbarLocationInternal: React.FC<
    SwitchToolbarLocationInternalProps
> = ({ children, layout, locations }) => {
    // settings store
    const { setCurrentLayout } = useActions(settingsActions);

    // compute which menu items should show as checked
    const toolbarMenuCheckedItems = {
        [ToolbarLocationRadioName]: [layout.toolbar],
        [ToolbarAnchorRadioName]: [layout.toolbarAnchor],
    };

    // callback to update current layout with new toolbar location
    const onChangeToolbarMenuCheckedItems: OnMenuCheckedValueChangeFunction = (
        _,
        { name, checkedItems },
    ) => {
        const newLayout = { ...layout };
        switch (name) {
            case ToolbarLocationRadioName:
                newLayout.toolbar = checkedItems[0] as WidgetType;
                break;
            case ToolbarAnchorRadioName:
                newLayout.toolbarAnchor = checkedItems[0] as "top" | "bottom";
                break;
        }
        setCurrentLayout(newLayout);
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
