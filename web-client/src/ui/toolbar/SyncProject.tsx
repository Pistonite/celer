//! Control for syncing file system changes to the editor
//!
//! Scenarios:
//! | Project Opened | AutoLoad | Loading | Last Error | State    | Icon      |
//! |----------------|----------|---------|
//! | No                                  | No         | Disabled | ArrowSync |
//! | Yes            | Disabled | No      | No         | Enabled  | ArrowSync |
//! | Yes            | Disabled | Yes     |            | Disabled | ArrowSync | Rotating
//! | Yes            | Enabled  | No      | No         | Enabled  | ArrowSyncCheckmark |
//! | Yes            | Enabled  | Yes     |            | Disabled | ArrowSync | Rotating
//! | Yes            | Inactive | No      | No         | Enabled  | ArrowSync |
//! | Yes            | Inactive | Yes     |            | Disabled | ArrowSync | Rotating
//! | Yes            |          | No      | Yes        |          | ArrowSyncDismiss |

import { forwardRef, useCallback } from "react";
import { useSelector } from "react-redux";
import { MenuItem, ToolbarButton, Tooltip } from "@fluentui/react-components";
import {
    ArrowSync20Regular,
    ArrowSyncCheckmark20Regular,
    ArrowSyncDismiss20Filled,
} from "@fluentui/react-icons";
import clsx from "clsx";

import { useKernel } from "core/kernel";
import { settingsSelector, viewSelector } from "core/store";

import { ToolbarControl } from "./util";

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
                    Reload file system
                </MenuItem>
            </Tooltip>
        );
    },
};

const useSyncProjectControl = () => {
    const kernel = useKernel();
    const { rootPath, autoLoadActive, loadInProgress, lastLoadError } =
        useSelector(viewSelector);
    const { autoLoadEnabled } = useSelector(settingsSelector);

    const isOpened = rootPath !== undefined;
    const enabled = isOpened && !loadInProgress;
    const icon = getIcon(
        isOpened,
        loadInProgress,
        lastLoadError,
        autoLoadEnabled,
        autoLoadActive,
    );
    const tooltip = getTooltip(
        isOpened,
        loadInProgress,
        lastLoadError,
        autoLoadEnabled,
        autoLoadActive,
    );

    const handler = useCallback(async () => {
        const editor = kernel.getEditor();
        if (!editor) {
            return;
        }

        const result = await editor.loadChangesFromFs(true /* isUserAction */);
        const { FsResultCodes } = await import("low/fs");
        if (result !== FsResultCodes.Ok) {
            // failure could be due to project structure change. try again
            const result2 = await editor.loadChangesFromFs(false);
            if (result2 !== FsResultCodes.Ok) {
                await kernel.showAlert(
                    "Error",
                    "Fail to load changes from file system. Please try again.",
                    "Close",
                    "",
                );
            }
        }
    }, [kernel]);

    return { tooltip, enabled, icon, handler };
};

const getIcon = (
    isOpened: boolean,
    loadInProgress: boolean,
    lastLoadError: boolean,
    autoLoadEnabled: boolean,
    autoLoadActive: boolean,
) => {
    if (!isOpened) {
        return <ArrowSync20Regular />;
    }
    if (loadInProgress) {
        return (
            <ArrowSync20Regular
                className={clsx(
                    "spinning-infinite",
                    autoLoadEnabled && autoLoadActive && "color-success",
                )}
            />
        );
    }
    if (lastLoadError) {
        return <ArrowSyncDismiss20Filled className="color-error" />;
    }
    if (autoLoadEnabled && autoLoadActive) {
        return <ArrowSyncCheckmark20Regular className="color-success" />;
    }
    return <ArrowSync20Regular />;
};

const getTooltip = (
    isOpened: boolean,
    loadInProgress: boolean,
    lastLoadError: boolean,
    autoLoadEnabled: boolean,
    autoLoadActive: boolean,
) => {
    if (isOpened) {
        if (loadInProgress) {
            return "Loading from file system in progress...";
        }
        if (lastLoadError) {
            return "There was an error loading from file system. Click to retry.";
        }
        if (autoLoadEnabled) {
            if (autoLoadActive) {
                return "Auto-load is enabled and active. Any change you made in the file system will be loaded automatically after a while. (Click to manually reload)";
            }
            return "Auto-load has been deactivated due to inactivity. Click to activate.";
        }
    }
    return "Reload files from file system";
};
