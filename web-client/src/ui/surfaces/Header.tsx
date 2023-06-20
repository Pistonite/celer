//! The header for the app
//!
//! Header consistents of a title and a toolbar.
//! The title displays the current document name (or other page titles).
//! The toolbar contains a list of controls available to the user.
//!
//! The toolbar can be below or on top of the title depending on the layout.
import "./Header.css";
import { Card, CardHeader, MenuButton, Title1, Toolbar, Text, ToolbarDivider, Title3, Subtitle2, Subtitle1, ToolbarButton, Tooltip, Menu, MenuTrigger, MenuPopover, MenuList, MenuItem, MenuDivider, MenuItemRadio, MenuGroup, MenuGroupHeader, MenuProps } from "@fluentui/react-components";
import { Window20Regular, Edit20Regular, Grid20Regular, Save20Regular, Copy20Regular, Delete20Regular } from "@fluentui/react-icons";
import { useLayout } from "core/utils";
import { WidgetType, settingsActions, settingsSelector, toolbarActions, toolbarSelector, useActions } from "data/store";
import { useMemo } from "react";
import { useSelector } from "react-redux";
/// Props for the header
export type HeaderProps = {
    /// if the toolbar is below the title
    isToolbarBelowTitle: boolean;
}

const LayoutRadioName = "Select a layout";
const ToolbarLocationRadioName = "Select a toolbar location";
const ToolbarAnchorRadioName = "Select a toolbar anchor";
const ToolbarLocations: Record<WidgetType, string> = {
    "viewer": "Document",
    "map": "Map",
    "editor": "Editor"
};

/// The header component
export const Header: React.FC<HeaderProps> = ({isToolbarBelowTitle}) => {
    const title = "Test Title"; // TODO: StageStore

    const { layout, availableToolbarLocations, isDefaultLayout } = useLayout();
    const { currentLayout, savedLayouts } = useSelector(settingsSelector);
    const { isEditingLayout } = useSelector(toolbarSelector);
    const { setIsEditingLayout } = useActions(toolbarActions);
    const { setCurrentLayout } = useActions(settingsActions);

    const layoutMenuCheckedItems = {
        [LayoutRadioName]: [`${isDefaultLayout ? -1 : currentLayout}`],
    };

    const toolbarMenuCheckedItems = useMemo(() => {
        return {
            [ToolbarLocationRadioName]: [layout.toolbar],
            [ToolbarAnchorRadioName]: [layout.toolbarAnchor],
        }
    }, [layout]);

    const onChangeToolbarMenuCheckedItems: MenuProps["onCheckedValueChange"] = (_, { name, checkedItems}) => {
        const newLayout = { ...layout };
        switch(name) {
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

    // const [layoutCheckedItems, setLayoutCheckedItems] = useState<Record<string, string[]>>({
    //     [LayoutRadioName]: ["-1"],
    // });
    return (
        <header className="celer-header">
            {/* <Card> */}
                {/* <CardHeader
                    header={}
                >
                    
                </CardHeader> */}
                <Toolbar>
                    <img src="/static/celer-3.svg" className="celer-logo" />
                <Subtitle1 as="h1">{title}</Subtitle1>
                <ToolbarDivider />
                <Menu checkedValues={layoutMenuCheckedItems}>
                    <MenuTrigger>
                        <Tooltip content="Layout" relationship="label">
                            
                            <ToolbarButton icon={<Grid20Regular />}>

                            </ToolbarButton>
                        </Tooltip> 

                    </MenuTrigger>
                    <MenuPopover>
                        <MenuList>
                            <MenuItemRadio name={LayoutRadioName} value="-1">Default Layout</MenuItemRadio>
                            {
                                savedLayouts.map((_, i) => (
                                    <MenuItemRadio name={LayoutRadioName} value={`${i}`} key={i}>Custom {i+1}</MenuItemRadio>
                                ))
                            }
                            
                                <MenuItemRadio name={LayoutRadioName} value="0">Layout 1</MenuItemRadio>
                                <MenuItemRadio name={LayoutRadioName} value="1">Layout 2</MenuItemRadio>
                            
                            <MenuDivider />
                            {
                                isEditingLayout ? (
                                    <Tooltip content="Finish editing the current layout" relationship="label">
                                        <MenuItem icon={<Save20Regular />} onClick={()=>setIsEditingLayout({ value: false })}>
                                            Finish
                                        </MenuItem>
                                    </Tooltip>
                                ) : (
                                    <Tooltip content={
                                        isDefaultLayout ? "Cannot edit the default layout" : "Edit the current layout"
                                    } relationship="label">
                                        <MenuItem disabled={isDefaultLayout} icon={<Edit20Regular />} onClick={()=>setIsEditingLayout({ value: true })}>
                                            Edit
                                        </MenuItem>
                                    </Tooltip>
                                )
                            }
                            
                            <MenuItem icon={<Copy20Regular />}>Duplicate</MenuItem>
                            <MenuItem icon={<Delete20Regular />}>Delete</MenuItem>

                        </MenuList>
                    </MenuPopover>
                </Menu>
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
                <ToolbarDivider />
                <MenuButton>
                    Layout
                </MenuButton>
                <MenuButton>
                    Layout
                </MenuButton>
                <MenuButton>
                    Layout
                </MenuButton>
            </Toolbar>
            {/* </Card> */}
            {/* <Toolbar size="medium">
            <Title1>{title}</Title1>
            </Toolbar> */}
            
        </header>
    );
    
}
// const a = [

// ]

// export const Default = () => {
//     const styles = useStyles();
  
//     const itemIds = new Array(8).fill(0).map((_, i) => i.toString());
  
//     return (
//       <Overflow>
//         <div className={mergeClasses(styles.container, styles.resizableArea)}>
//           {itemIds.map((i) => (
//             <OverflowItem key={i} id={i}>
//               <Button>Item {i}</Button>
//             </OverflowItem>
//           ))}
//           <OverflowMenu itemIds={itemIds} />
//         </div>
//       </Overflow>
//     );
//   };
  
//   const OverflowMenuItem: React.FC<Pick<OverflowItemProps, "id">> = (props) => {
//     const { id } = props;
//     const isVisible = useIsOverflowItemVisible(id);
  
//     if (isVisible) {
//       return null;
//     }
  
//     // As an union between button props and div props may be conflicting, casting is required
//     return <MenuItem>Item {id}</MenuItem>;
//   };
  
//   const OverflowMenu: React.FC<{ itemIds: string[] }> = ({ itemIds }) => {
//     const { ref, overflowCount, isOverflowing } =
//       useOverflowMenu<HTMLButtonElement>();
  
//     if (!isOverflowing) {
//       return null;
//     }
  
//     return (
//       <Menu>
//         <MenuTrigger disableButtonEnhancement>
//           <MenuButton ref={ref}>+{overflowCount} items</MenuButton>
//         </MenuTrigger>
  
//         <MenuPopover>
//           <MenuList>
//             {itemIds.map((i) => {
//               return <OverflowMenuItem key={i} id={i} />;
//             })}
//           </MenuList>
//         </MenuPopover>
//       </Menu>
//     );
//   };