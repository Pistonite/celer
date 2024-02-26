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

import { CommonStyles, useCommonStyles } from "ui/shared";
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
                    Save to file system
                </MenuItem>
            </Tooltip>
        );
    },
};

const useSaveProjectControl = () => {
    const kernel = useKernel();
    const { rootPath, saveInProgress, lastSaveError } =
        useSelector(viewSelector);
    const { autoSaveEnabled, editorMode } = useSelector(settingsSelector);

    const styles = useCommonStyles();
    const isOpened = rootPath !== undefined;
    const enabled = isOpened && editorMode === "web" && !saveInProgress;
    const icon = getIcon(
        styles,
        isOpened,
        saveInProgress,
        lastSaveError,
        autoSaveEnabled,
    );
    const tooltip = getTooltip(
        isOpened,
        saveInProgress,
        lastSaveError,
        autoSaveEnabled,
    );

    const handler = useCallback(async () => {
        const editor = kernel.getEditor();
        if (!editor) {
            return;
        }

        editor.notifyActivity();
        await editor.saveToFs();
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
    styles: CommonStyles,
    isOpened: boolean,
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
                className={autoSaveEnabled ? styles.colorSuccess : ""}
            />
        );
    }
    if (lastSaveError) {
        return <Save20Regular className={styles.colorError} />;
    }
    if (autoSaveEnabled) {
        return <SaveSync20Regular className={styles.colorSuccess} />;
    }
    return <Save20Regular />;
};

const getTooltip = (
    isOpened: boolean,
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
    return "Save changes to file system";
};
