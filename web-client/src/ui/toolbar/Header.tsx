//! The header for the app
//!
//! Header consistents of a title and a toolbar.
//! The title displays the current document name (or other page titles).
//! The toolbar contains a list of controls available to the user.
//!
//! The toolbar can be below or on top of the title depending on the layout.
import "./Header.css";

import {
    Card,
    CardHeader,
    Toolbar,
    ToolbarDivider,
    Subtitle1,
    ToolbarButton,
    Menu,
    MenuTrigger,
    MenuPopover,
    MenuList,
    MenuDivider,
    Overflow,
    OverflowItem,
    useIsOverflowItemVisible,
    useOverflowMenu,
    useIsOverflowGroupVisible,
} from "@fluentui/react-components";
import { MoreHorizontal20Filled } from "@fluentui/react-icons";
import clsx from "clsx";
import React, { PropsWithChildren } from "react";

import { HeaderControlList } from "./util";
import { SwitchToolbarLocation } from "./SwitchToolbarLocation";
import { SwitchLayout } from "./SwitchLayout";
import { SwitchMapLayer } from "./SwitchMapLayer";
import { Settings } from "./Settings";
import { ZoomIn, ZoomOut } from "./Zoom";
import { ViewDiagnostics } from "./ViewDiagnostics";

/// Header controls.
///
/// The controls are defined in groups. Each control is a ToolbarControl that defines its apperances in the toolbar and in the overflow menu
const HeaderControls: HeaderControlList = [
    // UI Controls
    {
        priority: 0,
        controls: [SwitchLayout, SwitchToolbarLocation],
    },
    // Map Controls
    {
        priority: 0,
        controls: [SwitchMapLayer, ZoomIn, ZoomOut],
    },
    // Diagnostic/editor
    {
        priority: 0,
        controls: [ViewDiagnostics],
    },
    // Setting
    {
        priority: 99,
        controls: [Settings],
    },
];

/// The header props
type HeaderProps = {
    /// Position of the toolbar
    toolbarAnchor: "top" | "bottom";
};

/// The header component
export const Header: React.FC<HeaderProps> = ({ toolbarAnchor }) => {
    const title =
        "My awesome route whose title is really really really long long"; // TODO: StageStore

    return (
        <header className={clsx("celer-header", toolbarAnchor)}>
            <Card
                size="small"
                appearance="filled-alternative"
                className="celer-title"
            >
                <CardHeader
                    image={
                        <img src="/static/celer-3.svg" className="celer-logo" />
                    }
                />
                <Subtitle1 as="h1">{title}</Subtitle1>
            </Card>

            <Overflow padding={90} minimumVisible={1}>
                <Toolbar className="celer-toolbar">
                    {HeaderControls.map((group, i) => (
                        <React.Fragment key={i}>
                            {group.controls.map((Control, j) => (
                                <OverflowItem
                                    priority={group.priority}
                                    id={toItemId(i, j)}
                                    groupId={i.toString()}
                                    key={j}
                                >
                                    <Control.ToolbarButton />
                                </OverflowItem>
                            ))}
                            {i < HeaderControls.length - 1 && (
                                <ToolbarOverflowDivider
                                    groupId={i.toString()}
                                />
                            )}
                        </React.Fragment>
                    ))}
                    <OverflowMenu />
                </Toolbar>
            </Overflow>
        </header>
    );
};

/// Wrapper for ToolbarDivider in the overflow
///
/// The divider is only visible when the group is visible
const ToolbarOverflowDivider: React.FC<{ groupId: string }> = ({ groupId }) => {
    const groupVisibleState = useIsOverflowGroupVisible(groupId);

    if (groupVisibleState !== "hidden") {
        return <ToolbarDivider />;
    }

    return null;
};

/// The overflow menu
///
/// Controls that cannot fit in the toolbar will be moved to here
const OverflowMenu: React.FC = () => {
    const { ref, isOverflowing } = useOverflowMenu<HTMLButtonElement>();

    if (!isOverflowing) {
        return null;
    }

    return (
        <Menu>
            <MenuTrigger disableButtonEnhancement>
                <ToolbarButton
                    ref={ref}
                    icon={<MoreHorizontal20Filled />}
                ></ToolbarButton>
            </MenuTrigger>
            <MenuPopover>
                <MenuList>
                    {HeaderControls.map((group, i) => (
                        <React.Fragment key={i}>
                            {group.controls.map((Control, j) => (
                                <ShowIfOverflown
                                    itemId={toItemId(i, j)}
                                    key={j}
                                >
                                    <Control.MenuItem />
                                </ShowIfOverflown>
                            ))}
                            {i < HeaderControls.length - 1 && (
                                <ShowIfGroupOverflown groupId={`${i}`}>
                                    <MenuDivider />
                                </ShowIfGroupOverflown>
                            )}
                        </React.Fragment>
                    ))}
                </MenuList>
            </MenuPopover>
        </Menu>
    );
};
/// Show children if the toolbar group is overflown
const ShowIfGroupOverflown: React.FC<
    PropsWithChildren<{ groupId: string }>
> = ({ children, groupId }) => {
    const groupVisibleState = useIsOverflowGroupVisible(groupId);
    if (groupVisibleState !== "hidden") {
        return <>{children}</>;
    }
    return null;
};

/// Show children if the toolbar item is overflown
const ShowIfOverflown: React.FC<PropsWithChildren<{ itemId: string }>> = ({
    children,
    itemId,
}) => {
    const isVisible = useIsOverflowItemVisible(itemId);
    if (isVisible) {
        return null;
    }
    return <>{children}</>;
};

/// Convert group id and index to item id
const toItemId = (groupId: number, index: number) => `${groupId}-${index}`;
