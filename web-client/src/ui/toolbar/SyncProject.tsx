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
import clsx from "clsx";

import { useKernel } from "core/kernel";
import { settingsSelector, viewActions, viewSelector } from "core/store";

import { useActions } from "low/store";
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
    const { incFileSysSerial } = useActions(viewActions);

    const isOpened = rootPath !== undefined;
    const enabled = isOpened && !loadInProgress && editorMode === "web";
    const icon = getIcon(lastLoadError);
    const tooltip = getTooltip(isOpened, loadInProgress, lastLoadError);

    const handler = useCallback(async () => {
        const editor = kernel.getEditor();
        if (!editor) {
            return;
        }

        editor.notifyActivity();
        const result = await editor.loadChangesFromFs();
        if (result.isErr()) {
            // failure could be due to project structure change. try again
            const result2 = await editor.loadChangesFromFs();
            if (result2.isErr()) {
                await kernel.getAlertMgr().show({
                    title: "Error",
                    message:
                        "Fail to load changes from file system. Please try again.",
                    okButton: "Close",
                });
            }
        }
        incFileSysSerial();
    }, [kernel, incFileSysSerial]);

    return { tooltip, enabled, icon, handler };
};

const getIcon = (lastLoadError: boolean) => {
    return (
        <FolderArrowUp20Regular
            className={clsx(lastLoadError && "color-error")}
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
