//! Control for saving editor changes to the file system
//!
//! Scenarios:
//! | Project Opened | AutoSave | Saving  | Last Error | State    | Icon      |
//! |----------------|----------|---------|
//! | No                                  | No         | Disabled | Save |
//! | Yes            | Disabled | No      | No         | Enabled  | Save |
//! | Yes            | Disabled | Yes     |            | Disabled | SaveEdit |
//! | Yes            | Enabled  | No      | No         | Enabled  | SaveSync |
//! | Yes            | Enabled  | Yes     |            | Disabled | SaveEdit |
//! | Yes            | Inactive | No      | No         | Enabled  | Save |
//! | Yes            | Inactive | Yes     |            | Disabled | SaveEdit |
//! | Yes            |          | No      | Yes        |          | Save | red

import { forwardRef, useCallback, useEffect } from "react";
import { useSelector } from "react-redux";
import { MenuItem, ToolbarButton, Tooltip } from "@fluentui/react-components";
import {
    Save20Regular,
    SaveEdit20Regular,
    SaveSync20Regular,
} from "@fluentui/react-icons";
import clsx from "clsx";

import { useKernel } from "core/kernel";
import { settingsSelector, viewSelector } from "core/store";

import { ToolbarControl } from "./util";

export const SaveProject: ToolbarControl = {
    ToolbarButton: forwardRef<HTMLButtonElement>((_, ref) => {
        const { tooltip, enabled, icon, handler } = useSaveProjectControl();
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
        const { tooltip, enabled, icon, handler } = useSaveProjectControl();
        return (
            <Tooltip content={tooltip} relationship="label">
                <MenuItem icon={icon} disabled={!enabled} onClick={handler}>
                    Save
                </MenuItem>
            </Tooltip>
        );
    },
};

const useSaveProjectControl = () => {
    const kernel = useKernel();
    const { rootPath, supportsSave, saveInProgress, lastSaveError } =
        useSelector(viewSelector);
    const { autoSaveEnabled } = useSelector(settingsSelector);

    const isOpened = rootPath !== undefined;
    const enabled = isOpened && supportsSave && !saveInProgress;
    const icon = getIcon(
        isOpened,
        supportsSave,
        saveInProgress,
        lastSaveError,
        autoSaveEnabled,
    );
    const tooltip = getTooltip(
        isOpened,
        supportsSave,
        saveInProgress,
        lastSaveError,
        autoSaveEnabled,
    );

    const handler = useCallback(async () => {
        const editor = kernel.getEditor();
        if (!editor) {
            return;
        }

        const result = await editor.saveChangesToFs(true /* isUserAction */);
        if (result.isErr()) {
            await kernel.showAlert(
                "Error",
                "Fail to save changes to file system. Please try again.",
                "Close",
                "",
            );
        }
    }, [kernel]);

    useEffect(() => {
        const keyHandler = (e: KeyboardEvent) => {
            if (e.ctrlKey && e.key === "s") {
                e.preventDefault();
                handler();
            }
        };
        window.addEventListener("keydown", keyHandler);
        return () => {
            window.removeEventListener("keydown", keyHandler);
        };
    }, [handler]);

    return { tooltip, enabled, icon, handler };
};

const getIcon = (
    isOpened: boolean,
    supportsSave: boolean,
    saveInProgress: boolean,
    lastSaveError: boolean,
    autoSaveEnabled: boolean,
) => {
    if (!isOpened) {
        return <Save20Regular />;
    }
    if (saveInProgress) {
        return (
            <SaveEdit20Regular
                className={clsx(autoSaveEnabled && "color-success")}
            />
        );
    }
    if (lastSaveError) {
        return <Save20Regular className="color-error" />;
    }
    if (autoSaveEnabled) {
        return <SaveSync20Regular className="color-success" />;
    }
    return <Save20Regular className={clsx(!supportsSave && "color-error")} />;
};

const getTooltip = (
    isOpened: boolean,
    supportsSave: boolean,
    saveInProgress: boolean,
    lastSaveError: boolean,
    autoSaveEnabled: boolean,
) => {
    if (isOpened) {
        if (saveInProgress) {
            return "Saving changes to file system...";
        }
        if (lastSaveError) {
            return "There was an error saving to file system. Click to retry.";
        }
        if (autoSaveEnabled) {
            return "Auto-save is enabled. Any change you made in the editor will be saved automatically after a while. (Click to manually save)";
        }
    }
    if (!supportsSave) {
        return "Save is not supported by your browser";
    }
    return "Save changes to file system";
};
