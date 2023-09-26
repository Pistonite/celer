//! Control for switching map layer
//!
//! The map layer control allows the user to switch between
//! different map layers defined in the document.
//!
import React, { forwardRef } from "react";
import { useSelector } from "react-redux";
import {
    Menu,
    MenuItem,
    MenuItemRadio,
    MenuList,
    MenuPopover,
    MenuTrigger,
    ToolbarButton,
    Tooltip,
} from "@fluentui/react-components";
import { Layer20Regular } from "@fluentui/react-icons";
import { documentSelector, viewActions, viewSelector } from "core/store";
import { useActions } from "low/store";

import {
    ControlComponentProps,
    OnMenuCheckedValueChangeFunction,
    ToolbarControl,
} from "./util";

/// The map layer switch control
export const SwitchMapLayer: ToolbarControl = {
    ToolbarButton: forwardRef<HTMLButtonElement>((_, ref) => {
        const layerNames = useMapLayerNames();
        return (
            <SwitchMapLayerInternal layerNames={layerNames}>
                <Tooltip content="Map layer" relationship="label">
                    <ToolbarButton
                        ref={ref}
                        icon={<Layer20Regular />}
                        disabled={layerNames.length < 2}
                    />
                </Tooltip>
            </SwitchMapLayerInternal>
        );
    }),
    MenuItem: () => {
        const layerNames = useMapLayerNames();
        return (
            <SwitchMapLayerInternal layerNames={layerNames}>
                <MenuItem
                    icon={<Layer20Regular />}
                    disabled={layerNames.length < 2}
                >
                    Map layers
                </MenuItem>
            </SwitchMapLayerInternal>
        );
    },
};

/// Internal convenience hook to get the layer names
const useMapLayerNames = () => {
    const { document } = useSelector(documentSelector);
    if (!document) {
        return [];
    }
    return document.project.map.layers.map(
        (layer) => layer.name || "(Unnamed layer)",
    );
};

/// The control is disabled if there are less than 2 layers.
const LayerRadioName = "Select a layer";

/// Internal props for the control
type SwitchMapLayerInternalProps = ControlComponentProps & {
    /// The layer names
    layerNames: string[];
};

const SwitchMapLayerInternal: React.FC<SwitchMapLayerInternalProps> = ({
    layerNames,
    children,
}) => {
    const { currentMapLayer } = useSelector(viewSelector);
    const { setMapLayer } = useActions(viewActions);

    const layerMenuCheckedItems = {
        [LayerRadioName]: [`${currentMapLayer}`],
    };

    const onChangeLayerMenuCheckedItems: OnMenuCheckedValueChangeFunction = (
        _,
        { checkedItems },
    ) => {
        setMapLayer(parseInt(checkedItems[0] as string));
    };
    return (
        <Menu
            checkedValues={layerMenuCheckedItems}
            onCheckedValueChange={onChangeLayerMenuCheckedItems}
        >
            <MenuTrigger>{children}</MenuTrigger>
            <MenuPopover>
                <MenuList>
                    {layerNames.map((layerName, i) => (
                        <MenuItemRadio
                            name={LayerRadioName}
                            value={`${i}`}
                            key={i}
                        >
                            {layerName}
                        </MenuItemRadio>
                    ))}
                </MenuList>
            </MenuPopover>
        </Menu>
    );
};
