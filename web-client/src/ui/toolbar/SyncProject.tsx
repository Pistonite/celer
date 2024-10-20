//! Control for syncing file system changes to the editor
//!
//! Scenarios:
//! | Project Opened | Loading | Last Error | State    | Icon      |
//! |----------------|---------|------------|----------|-----------|
//! | No                       | No         | Disabled | FolderArrowUp |
//! | Yes            | No      | No         | Enabled  | FolderArrowUp |
//! | Yes            | Yes     |            | Disabled | FolderArrowUp |
//! | Yes            | No      | Yes        | Enabled  | FolderArrowUp | Red

import { forwardRef, useCallback } from "react";
import { useSelector } from "react-redux";
import { MenuItem, ToolbarButton, Tooltip } from "@fluentui/react-components";
import { FolderArrowUp20Regular } from "@fluentui/react-icons";

import type { CommonStyles } from "ui/shared";
import { useCommonStyles } from "ui/shared";
import { useKernel } from "core/kernel";
import { settingsSelector, viewSelector } from "core/store";

import type { ToolbarControl } from "./util";

export const SyncProject: ToolbarControl = {
    ToolbarButton: forwardRef<HTMLButtonElement>((_, ref) => {
        const { tooltip, enabled, icon, handler } = useSyncProjectControl();
        return (
            <Tooltip content={tooltip} relationship="label">
                <ToolbarButton
                    ref={ref}
                    icon={icon}
                    disabled={!enabled}
                    onClick={handler}
                />
            </Tooltip>
        );
    }),
    MenuItem: () => {
        const { tooltip, enabled, icon, handler } = useSyncProjectControl();
        return (
            <Tooltip content={tooltip} relationship="label">
                <MenuItem icon={icon} disabled={!enabled} onClick={handler}>
                    Load from file system
                </MenuItem>
            </Tooltip>
        );
    },
};

const useSyncProjectControl = () => {
    const kernel = useKernel();
    const { rootPath, loadInProgress, lastLoadError } =
        useSelector(viewSelector);
    const { editorMode } = useSelector(settingsSelector);

    const styles = useCommonStyles();

    const isOpened = rootPath !== undefined;
    const enabled = isOpened && !loadInProgress && editorMode === "web";
    const icon = getIcon(styles, lastLoadError);
    const tooltip = getTooltip(isOpened, loadInProgress, lastLoadError);

    const handler = useCallback(async () => {
        const editor = kernel.asEdit().getEditor();
        if (!editor) {
            return;
        }

        editor.notifyActivity();
        await editor.loadFromFs();
    }, [kernel]);

    return { tooltip, enabled, icon, handler };
};

const getIcon = (styles: CommonStyles, lastLoadError: boolean) => {
    return (
        <FolderArrowUp20Regular
            className={lastLoadError ? styles.colorError : ""}
        />
    );
};

const getTooltip = (
    isOpened: boolean,
    loadInProgress: boolean,
    lastLoadError: boolean,
) => {
    if (isOpened) {
        if (loadInProgress) {
            return "Loading from file system in progress...";
        }
        if (lastLoadError) {
            return "There was an error loading from file system. Click to retry.";
        }
    }
    return "Load from file system";
};
