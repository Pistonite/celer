//! Control for compiling the project

import { forwardRef, useCallback } from "react";
import { useSelector } from "react-redux";
import { MenuItem, ToolbarButton, Tooltip } from "@fluentui/react-components";
import { ArrowSync20Regular } from "@fluentui/react-icons";

import { useCommonStyles } from "ui/shared";
import { useKernel } from "core/kernel";
import { viewSelector } from "core/store";

import { ToolbarControl } from "./util";

export const ReloadDocument: ToolbarControl = {
    ToolbarButton: forwardRef<HTMLButtonElement>((_, ref) => {
        const { handler, disabled, icon, tooltip } = useReloadDocumentControl();
        return (
            <Tooltip content={tooltip} relationship="label">
                <ToolbarButton
                    ref={ref}
                    icon={icon}
                    disabled={disabled}
                    onClick={handler}
                />
            </Tooltip>
        );
    }),
    MenuItem: () => {
        const { handler, disabled, icon, tooltip } = useReloadDocumentControl();
        return (
            <Tooltip content={tooltip} relationship="label">
                <MenuItem icon={icon} disabled={disabled} onClick={handler}>
                    Compile project
                </MenuItem>
            </Tooltip>
        );
    },
};

function useReloadDocumentControl() {
    const kernel = useKernel();
    const { stageMode, rootPath, compileInProgress, compilerReady } =
        useSelector(viewSelector);
    const handler = useCallback(() => {
        kernel.reloadDocument();
    }, [kernel]);

    const styles = useCommonStyles();

    const icon = (
        <ArrowSync20Regular
            className={compileInProgress ? styles.spinningInfinite : ""}
        />
    );
    let tooltip;
    let disabled;
    if (stageMode === "edit") {
        tooltip = getEditorTooltip(!!rootPath, compileInProgress);
        disabled = !rootPath || compileInProgress || !compilerReady;
    } else {
        tooltip = getViewerTooltip(compileInProgress);
        disabled = compileInProgress;
    }

    return {
        handler,
        disabled,
        icon,
        tooltip,
    };
}

function getViewerTooltip(compileInProgress: boolean) {
        if (compileInProgress) {
            return "Loading...";
        }
        return "Reload Document";
}

function getEditorTooltip(isOpened: boolean, compileInProgress: boolean) {
    if (isOpened) {
        if (compileInProgress) {
            return "Compiling...";
        }
        return "Click to compile the project";
    }
    return "Compile project";
}
