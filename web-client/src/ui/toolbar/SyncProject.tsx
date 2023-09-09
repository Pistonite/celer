//! Control for syncing file system changes to the editor
//!
//! Scenarios:
//! | Project Opened | AutoLoad | Loading | Last Error | State    | Icon      |
//! |----------------|----------|---------|
//! | No                                  | No         | Disabled | ArrowSync |
//! | Yes            | Disabled | No      | No         | Enabled  | ArrowSync |
//! | Yes            | Disabled | Yes     |            | Disabled | ArrowSync | Rotating
//! | Yes            | Enabled  | No      | No         | Disabled | ArrowSyncCheckmark |
//! | Yes            | Enabled  | Yes     |            | Disabled | ArrowSync | Rotating
//! | Yes            | Inactive | No      | No         | Enabled  | ArrowSync |
//! | Yes            | Inactive | Yes     |            | Disabled | ArrowSync | Rotating
//! | Yes            |          | No      | Yes        |          | ArrowSyncDismiss |

import { forwardRef, useCallback } from "react";
import { useSelector } from "react-redux";
import { MenuItem, ToolbarButton, Tooltip } from "@fluentui/react-components";
import { ArrowSync20Regular, ArrowSyncCheckmark20Regular, ArrowSyncDismiss20Regular } from "@fluentui/react-icons";

import { useKernel } from "core/kernel";
import { settingsSelector, viewSelector } from "core/store";

import { ToolbarControl } from "./util";

export const SyncProject: ToolbarControl = {
    ToolbarButton: forwardRef<HTMLButtonElement>((_, ref) => {
        const {text, enabled, icon, handler} = useSyncProjectControl();
        return (
                <Tooltip content={text} relationship="label">
                    <ToolbarButton
                        ref={ref}
                        icon={icon }
                        disabled={!enabled}
                        onClick={handler}
                    />
                </Tooltip>

        );
    }),
    MenuItem: () => {
        const {text, enabled, icon, handler} = useSyncProjectControl();
        return (
            <MenuItem
                        icon={icon }
                disabled={!enabled}
                onClick={handler}
            >
                {text}
            </MenuItem>
        );

    }
};

const useSyncProjectControl = () => {
    const kernel = useKernel();
    const { rootPath, autoLoadActive, loadInProgress, lastLoadError } = useSelector(viewSelector);
    const { autoLoadEnabled } = useSelector(settingsSelector);

    const enabled = rootPath !== undefined 
        && !loadInProgress && (!autoLoadEnabled || !autoLoadActive);
    const icon = getIcon(rootPath !== undefined, loadInProgress, lastLoadError, autoLoadEnabled, autoLoadActive);
    const text = getText(loadInProgress, lastLoadError, autoLoadEnabled, autoLoadActive);
    
    const handler = useCallback(async () => {
        const editor = kernel.getEditor();
        if (!editor) {
            return;
        }

        await editor.loadChangesFromFs();
    }, [kernel]);

    return {text, enabled, icon, handler};
};

const getIcon = (
    isOpened: boolean, 
    loadInProgress: boolean, 
    lastLoadError: boolean, 
    autoLoadEnabled: boolean, 
    autoLoadActive: boolean
) => {
    if (!isOpened) {
        return <ArrowSync20Regular />;
    }
    if (loadInProgress) {
        return <ArrowSync20Regular className="spinning-infinite" />;
    }
    if (lastLoadError) {
        return <ArrowSyncDismiss20Regular />;
    }
    if (autoLoadEnabled && autoLoadActive) {
        return <ArrowSyncCheckmark20Regular />;
    }
        return <ArrowSync20Regular />;
};

const getText = (
    loadInProgress: boolean, 
    lastLoadError: boolean, 
    autoLoadEnabled: boolean, 
    autoLoadActive: boolean
) => {
    if (loadInProgress) {
        return "Syncing filesystem...";
    }
    if (lastLoadError) {
        return "Retry sync filesystem";
    }
    if (autoLoadEnabled ) {
        if (autoLoadActive) {
            return "(Auto-load enabled)";
        } 
        return "Activate auto-load";
    }
    return "Sync filesystem";
};
