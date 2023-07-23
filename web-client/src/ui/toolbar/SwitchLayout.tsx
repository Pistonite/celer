//! Layout switching control
//!
//! The layout switching control offers ability to
//! switch between different layouts, edit current layout, duplicate current layout
//! and delete current layout.
//!
//! User can switch between the "default" layout and a set of saved layouts.
//! The default layout cannot be edited or deleted, while the saved layouts can.

import React from "react";
import { useSelector } from "react-redux";
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
import {
    DataTreemap20Regular,
    Save20Regular,
    Delete20Regular,
    Copy20Regular,
    Edit20Regular,
} from "@fluentui/react-icons";
import { isCurrentLayoutDefault } from "core/layout";
import { settingsActions, settingsSelector, viewActions, viewSelector } from "core/store";
import { useActions } from "low/store";

import {
    ControlComponentProps,
    OnMenuCheckedValueChangeFunction,
    ToolbarControl,
} from "./util";

/// Switch layout control
export const SwitchLayout: ToolbarControl = {
    ToolbarButton: React.forwardRef<HTMLButtonElement>((_, ref) => {
        return (
            <SwitchLayoutInternal>
                <Tooltip content="Layout" relationship="label">
                    <ToolbarButton ref={ref} icon={<DataTreemap20Regular />} />
                </Tooltip>
            </SwitchLayoutInternal>
        );
    }),
    MenuItem: () => {
        return (
            <SwitchLayoutInternal>
                <MenuItem icon={<DataTreemap20Regular />}>Layouts</MenuItem>
            </SwitchLayoutInternal>
        );
    },
};

/// Layout radio button group name
const LayoutRadioName = "Select a layout";

/// Internal switch layout control logic
const SwitchLayoutInternal: React.FC<ControlComponentProps> = ({
    children,
}) => {
    // settings store
    const settings = useSelector(settingsSelector);
    const isDefaultLayout = isCurrentLayoutDefault(settings);
    const { currentLayout, savedLayouts } = settings;
    const { duplicateLayout, deleteCurrentLayout, switchLayout } =
        useActions(settingsActions);
    // view store
    const { isEditingLayout } = useSelector(viewSelector);
    const { setIsEditingLayout } = useActions(viewActions);

    // compute which menu items should show as checked
    const layoutMenuCheckedItems = {
        [LayoutRadioName]: [`${isDefaultLayout ? -1 : currentLayout}`],
    };

    // callback to set the layer
    const onChangeLayoutMenuCheckedItems: OnMenuCheckedValueChangeFunction = (
        _,
        { checkedItems },
    ) => {
        switchLayout(parseInt(checkedItems[0] as string));
    };
    return (
        <Menu
            checkedValues={layoutMenuCheckedItems}
            onCheckedValueChange={onChangeLayoutMenuCheckedItems}
        >
            <MenuTrigger>{children}</MenuTrigger>
            <MenuPopover>
                <MenuList>
                    <MenuItemRadio name={LayoutRadioName} value="-1">
                        Default Layout
                    </MenuItemRadio>
                    {savedLayouts.map((_, i) => (
                        <MenuItemRadio
                            name={LayoutRadioName}
                            value={`${i}`}
                            key={i}
                        >
                            Custom {i + 1}
                        </MenuItemRadio>
                    ))}

                    <MenuDivider />
                    {isEditingLayout ? (
                        <Tooltip
                            content="Finish editing the current layout"
                            relationship="label"
                        >
                            <MenuItem
                                icon={<Save20Regular />}
                                onClick={() => setIsEditingLayout(false)}
                            >
                                Finish
                            </MenuItem>
                        </Tooltip>
                    ) : (
                        <Tooltip
                            content={
                                isDefaultLayout
                                    ? "Cannot edit the default layout"
                                    : "Edit the current layout"
                            }
                            relationship="label"
                        >
                            <MenuItem
                                disabled={isDefaultLayout}
                                icon={<Edit20Regular />}
                                onClick={() => setIsEditingLayout(true)}
                            >
                                Edit
                            </MenuItem>
                        </Tooltip>
                    )}

                    <MenuItem
                        icon={<Copy20Regular />}
                        onClick={duplicateLayout}
                    >
                        Duplicate
                    </MenuItem>

                    <MenuItem
                        icon={<Delete20Regular />}
                        onClick={deleteCurrentLayout}
                    >
                        Delete
                    </MenuItem>
                </MenuList>
            </MenuPopover>
        </Menu>
    );
};
