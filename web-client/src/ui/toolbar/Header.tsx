//! The header for the app
//!
//! Header consistents of a title and a toolbar.
//! The title displays the current document name (or other page titles).
//! The toolbar contains a list of controls available to the user.
//!
//! The toolbar can be below or on top of the title depending on the layout.

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
    mergeClasses,
} from "@fluentui/react-components";
import { MoreHorizontal20Filled } from "@fluentui/react-icons";
import React, { PropsWithChildren, useMemo } from "react";
import { useSelector } from "react-redux";

import { documentSelector, settingsSelector, viewSelector } from "core/store";
import type { ExecDoc } from "low/celerc";

import { getHeaderControls } from "./getHeaderControls";
import { HeaderControlList } from "./util";
import { useHeaderStyles } from "./styles";

/// The header props
type HeaderProps = {
    /// Position of the toolbar
    toolbarAnchor: "top" | "bottom";
};

/// The header component
export const Header: React.FC<HeaderProps> = ({ toolbarAnchor }) => {
    const { document } = useSelector(documentSelector);
    const { stageMode, compileInProgress } = useSelector(viewSelector);
    const { editorMode } = useSelector(settingsSelector);
    const title = useTitle(stageMode, document, compileInProgress);

    const headerControls = useMemo(() => {
        return getHeaderControls(stageMode, editorMode);
    }, [stageMode, editorMode]);

    const styles = useHeaderStyles();

    return (
        <header className={mergeClasses(styles.header, styles[toolbarAnchor])}>
            <Card
                size="small"
                appearance="filled-alternative"
                className={styles.title}
            >
                <CardHeader
                    image={
                        <img
                            src="/static/celer-3.svg"
                            className={styles.logo}
                        />
                    }
                />
                <Subtitle1 as="h1" className={styles.titleText}>
                    {title}
                </Subtitle1>
            </Card>

            <Overflow padding={130} minimumVisible={1}>
                <Toolbar className={styles.toolbar}>
                    {headerControls.map((group, i) => (
                        <React.Fragment key={i}>
                            {group.controls.map((Control, j) => (
                                <OverflowItem
                                    priority={
                                        (Control.priority || 0) + group.priority
                                    }
                                    id={toItemId(i, j)}
                                    groupId={i.toString()}
                                    key={j}
                                >
                                    <Control.ToolbarButton />
                                </OverflowItem>
                            ))}
                            {i < headerControls.length - 1 && (
                                <ToolbarOverflowDivider
                                    groupId={i.toString()}
                                />
                            )}
                        </React.Fragment>
                    ))}
                    <OverflowMenu controls={headerControls} />
                </Toolbar>
            </Overflow>
        </header>
    );
};

function useTitle(stageMode: string, document: ExecDoc | undefined, compileInProgress: boolean) {
    if (document) {
        // if document is loaded, return the document title
        return document?.project.title;
    }
    if (stageMode ==="edit"){
        // if in edit mode, return the editor title
        return "Celer Editor";
    }
    // viewer
    if (compileInProgress) {
        return "Loading...";
    }
    // if in view mode, but is not loading (e.g. user cancelled the loading)
    // return the viewer title
    return "Celer Viewer";
}

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
const OverflowMenu: React.FC<{ controls: HeaderControlList }> = ({
    controls,
}) => {
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
                    {controls.map((group, i) => (
                        <React.Fragment key={i}>
                            {group.controls.map((Control, j) => (
                                <ShowIfOverflown
                                    itemId={toItemId(i, j)}
                                    key={j}
                                >
                                    <Control.MenuItem />
                                </ShowIfOverflown>
                            ))}
                            {i < controls.length - 1 && (
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
    if (groupVisibleState === "visible") {
        return null;
    }
    return <>{children}</>;
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
