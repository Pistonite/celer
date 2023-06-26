//! The header for the app
//!
//! Header consistents of a title and a toolbar.
//! The title displays the current document name (or other page titles).
//! The toolbar contains a list of controls available to the user.
//!
//! The toolbar can be below or on top of the title depending on the layout.
import "./Header.css";
import { Card, CardHeader, MenuButton, Title1, Toolbar, Text, ToolbarDivider, Title3, Subtitle2, Subtitle1, ToolbarButton, Tooltip, Menu, MenuTrigger, MenuPopover, MenuList, MenuItem, MenuDivider, MenuItemRadio, MenuGroup, MenuGroupHeader, MenuProps, Overflow, OverflowItem, useIsOverflowItemVisible, useOverflowMenu, useIsOverflowGroupVisible } from "@fluentui/react-components";
import { 
    Layer20Regular, 
    MoreHorizontal20Filled,
    Settings20Regular,
    Window20Regular,
    Edit20Regular,
    Grid20Regular,
    Save20Regular,
    Copy20Regular,
    Delete20Regular,
} from "@fluentui/react-icons";
import clsx from "clsx";
import { useLayout } from "core/utils";
import { WidgetType, documentSelector, settingsActions, settingsSelector, toolbarActions, toolbarSelector, useActions } from "data/store";
import React from "react";
import { useMemo } from "react";
import { useSelector } from "react-redux";


const LayoutRadioName = "Select a layout";
const ToolbarLocationRadioName = "Select a toolbar location";
const ToolbarAnchorRadioName = "Select a toolbar anchor";
const LayerRadioName = "Select a layer";
const ToolbarLocations: Record<WidgetType, string> = {
    "viewer": "Document",
    "map": "Map",
    "editor": "Editor"
};

/// The header component
export const Header: React.FC = () => {
    const title = "My awesome route whose title is really really really long long"; // TODO: StageStore

    const { layout, availableToolbarLocations, isDefaultLayout } = useLayout();
    const { currentLayout, savedLayouts } = useSelector(settingsSelector);
    const { isEditingLayout, currentMapLayer } = useSelector(toolbarSelector);
    const { setIsEditingLayout, setCurrentMapLayer } = useActions(toolbarActions);
    const { setCurrentLayout, addAndSwitchLayout, deleteCurrentLayout, switchLayout } = useActions(settingsActions);

    const { document } = useSelector(documentSelector);
    const layerNames = document.project.map.layers.map(layer => layer.name);

    const layerMenuCheckedItems = {
        [LayerRadioName]: [`${currentMapLayer}`],
    };

    const onChangeLayerMenuCheckedItems: MenuProps["onCheckedValueChange"] = (_, { name, checkedItems }) => {
        setCurrentMapLayer(
            parseInt(checkedItems[0] as string)
        );
    };



    const layoutMenuCheckedItems = {
        [LayoutRadioName]: [`${isDefaultLayout ? -1 : currentLayout}`],
    };

    const onChangeLayoutMenuCheckedItems: MenuProps["onCheckedValueChange"] = (_, { name, checkedItems }) => {
        console.log(checkedItems);
        switchLayout({ index: parseInt(checkedItems[0] as string) });
    }


    const toolbarMenuCheckedItems = useMemo(() => {
        return {
            [ToolbarLocationRadioName]: [layout.toolbar],
            [ToolbarAnchorRadioName]: [layout.toolbarAnchor],
        }
    }, [layout]);

    const onChangeToolbarMenuCheckedItems: MenuProps["onCheckedValueChange"] = (_, { name, checkedItems }) => {
        const newLayout = { ...layout };
        switch (name) {
            case ToolbarLocationRadioName:
                newLayout.toolbar = checkedItems[0] as WidgetType;
                break;
            case ToolbarAnchorRadioName:
                newLayout.toolbarAnchor = checkedItems[0] as "top" | "bottom";
                break;
        };
        setCurrentLayout({
            layout: newLayout
        });
    };


    return (
        <header className={clsx("celer-header", layout.toolbarAnchor)}>
            <Card size="small" appearance="filled-alternative" className="celer-title">
                <CardHeader
                    image={<img src="/static/celer-3.svg" className="celer-logo" />}
                >

                </CardHeader>

                <Subtitle1 as="h1">{title}</Subtitle1>


            </Card>

            <Overflow padding={90} minimumVisible={1}>
                <Toolbar className="celer-toolbar">
                    <OverflowItem id="layout-1" groupId="1">
                        <span>

                            <Menu checkedValues={layoutMenuCheckedItems} onCheckedValueChange={onChangeLayoutMenuCheckedItems}>
                                <MenuTrigger>
                                    <Tooltip content="Layout" relationship="label">
                                        <ToolbarButton icon={<Grid20Regular />} />
                                    </Tooltip>
                                </MenuTrigger>
                                <MenuPopover>
                                    <MenuList>
                                        <MenuItemRadio name={LayoutRadioName} value="-1">Default Layout</MenuItemRadio>
                                        {
                                            savedLayouts.map((_, i) => (
                                                <MenuItemRadio name={LayoutRadioName} value={`${i}`} key={i} >Custom {i + 1}</MenuItemRadio>
                                            ))
                                        }


                                        <MenuDivider />
                                        {
                                            isEditingLayout ? (
                                                <Tooltip content="Finish editing the current layout" relationship="label">
                                                    <MenuItem icon={<Save20Regular />} onClick={() => setIsEditingLayout(false)}>
                                                        Finish
                                                    </MenuItem>
                                                </Tooltip>
                                            ) : (
                                                <Tooltip content={
                                                    isDefaultLayout ? "Cannot edit the default layout" : "Edit the current layout"
                                                } relationship="label">
                                                    <MenuItem disabled={isDefaultLayout} icon={<Edit20Regular />} onClick={() => setIsEditingLayout(true)}>
                                                        Edit
                                                    </MenuItem>
                                                </Tooltip>
                                            )
                                        }

                                        <MenuItem icon={<Copy20Regular />} onClick={() => {
                                            addAndSwitchLayout({
                                                layout: layout
                                            });
                                        }}>Duplicate</MenuItem>
                                        <MenuItem icon={<Delete20Regular />} onClick={() => {
                                            deleteCurrentLayout();
                                        }}>Delete</MenuItem>


                                    </MenuList>
                                </MenuPopover>
                            </Menu>
                        </span>
                    </OverflowItem>
                    <OverflowItem id="toolbar-1" groupId="1">
                        <span>
                            <Menu checkedValues={toolbarMenuCheckedItems} onCheckedValueChange={onChangeToolbarMenuCheckedItems}>
                                <MenuTrigger>
                                    <Tooltip content={isDefaultLayout ? "You cannot change the toolbar location in the default layout" : "Change toolbar location"} relationship="label">
                                        <ToolbarButton disabled={isDefaultLayout} icon={<Window20Regular />}>

                                        </ToolbarButton>

                                    </Tooltip>
                                </MenuTrigger>
                                <MenuPopover>
                                    <MenuList>

                                        <MenuItemRadio name={ToolbarLocationRadioName} value="viewer">Document</MenuItemRadio>
                                        <MenuItemRadio name={ToolbarLocationRadioName} value="map">Map</MenuItemRadio>
                                        <MenuItemRadio name={ToolbarLocationRadioName} value="editor">Editor</MenuItemRadio>

                                        <MenuDivider />
                                        <MenuItemRadio name={ToolbarAnchorRadioName} value="top">Top</MenuItemRadio>
                                        <MenuItemRadio name={ToolbarAnchorRadioName} value="bottom">Bottom</MenuItemRadio>


                                    </MenuList>
                                </MenuPopover>
                            </Menu>
                        </span>
                    </OverflowItem>
                    <ToolbarOverflowDivider groupId="1" />
                    <OverflowItem id="layer-2" groupId="2">
                        <span>
                            <Menu checkedValues={layerMenuCheckedItems} onCheckedValueChange={onChangeLayerMenuCheckedItems}>
                                <MenuTrigger>
                            <MenuButton icon={<Layer20Regular />} >
                                {layerNames[currentMapLayer] || "Unknown" }
                            </MenuButton>
                            </MenuTrigger>
                            <MenuPopover>
                                <MenuList>
                                        {
                                            layerNames.map((layerName, i) => (
                                                <MenuItemRadio name={LayerRadioName} value={`${i}`} key={i} >{layerName}</MenuItemRadio>
                                            ))
                                        }
                                </MenuList>
                            </MenuPopover>
                            </Menu>
                        </span>
                        
                    </OverflowItem>
                    <ToolbarOverflowDivider groupId="2" />
                    <OverflowItem id="settings-999" groupId="999">
                        <ToolbarButton icon={<Settings20Regular />} />
                    </OverflowItem>
                    <OverflowMenu itemIds={[
                        ["layout-1", "toolbar-1"],
                        ["layer-2"],
                        ["settings-999"]
                    ]} />

                    {/* <div> */}

                    {/* <div className="celer-toolbar-main"> */}
                    {/* </div> */}
                    {/* </div> */}
                    {/* <ToolbarDivider /> */}
                </Toolbar>
            </Overflow>
            {/* </Card> */}
            {/* <Toolbar size="medium">
            <Title1>{title}</Title1>
            </Toolbar> */}

        </header>
    );

}
type ToolbarOverflowDividerProps = {
    groupId: string;
};
const ToolbarOverflowDivider: React.FC<ToolbarOverflowDividerProps> = ({ groupId }) => {
    const groupVisibleState = useIsOverflowGroupVisible(groupId);

    if (groupVisibleState !== "hidden") {
        return <ToolbarDivider />;
    }

    return null;
};

const OverflowMenuItem1: React.FC = () => {
    const isVisible = useIsOverflowItemVisible("1");

    if (isVisible) {
        return null;
    }

    // As an union between button props and div props may be conflicting, casting is required
    return <MenuItem>Item 1</MenuItem>;
};

const OverflowMenuItem2: React.FC = () => {
    const isVisible = useIsOverflowItemVisible("2");

    if (isVisible) {
        return null;
    }

    // As an union between button props and div props may be conflicting, casting is required
    return <MenuItem>Item 2</MenuItem>;
};

const OverflowMenu: React.FC<{ itemIds: string[][] }> = ({ itemIds }) => {
    const { ref, overflowCount, isOverflowing } =
        useOverflowMenu<HTMLButtonElement>();


    if (!isOverflowing) {
        return null;
    }

    return (
        <Menu>
            <MenuTrigger disableButtonEnhancement>
                <ToolbarButton ref={ref} icon={<MoreHorizontal20Filled />}></ToolbarButton>
            </MenuTrigger>

            <MenuPopover>
                <MenuList>
                    <OverflowMenuItem1 />
                    <OverflowMenuItem2 />
                </MenuList>
            </MenuPopover>
        </Menu>
    );
};
