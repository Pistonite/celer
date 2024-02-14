//! Control for compiling the project

import { forwardRef, useCallback } from "react";
import { useSelector } from "react-redux";
import { MenuItem, ToolbarButton, Tooltip } from "@fluentui/react-components";
import { ArrowSync20Regular } from "@fluentui/react-icons";

import { useCommonStyles } from "ui/shared";
import { useKernel } from "core/kernel";
import { viewSelector } from "core/store";

import { ToolbarControl } from "./util";

export const CompileProject: ToolbarControl = {
    ToolbarButton: forwardRef<HTMLButtonElement>((_, ref) => {
        const { handler, disabled, icon, tooltip } = useCompileProjectControl();
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
        const { handler, disabled, icon, tooltip } = useCompileProjectControl();
        return (
            <Tooltip content={tooltip} relationship="label">
                <MenuItem icon={icon} disabled={disabled} onClick={handler}>
                    Compile project
                </MenuItem>
            </Tooltip>
        );
    },
};

const useCompileProjectControl = () => {
    const kernel = useKernel();
    const { rootPath, compileInProgress, compilerReady } =
        useSelector(viewSelector);
    const handler = useCallback(() => {
        kernel.compile();
    }, [kernel]);

    const styles = useCommonStyles();

    const icon = (
        <ArrowSync20Regular
            className={compileInProgress ? styles.spinningInfinite : ""}
        />
    );
    const tooltip = getTooltip(!!rootPath, compileInProgress);

    return {
        handler,
        disabled: !rootPath || compileInProgress || !compilerReady,
        icon,
        tooltip,
    };
};

const getTooltip = (isOpened: boolean, compileInProgress: boolean) => {
    if (isOpened) {
        if (compileInProgress) {
            return "Compiling...";
        }
        return "Click to compile the project";
    }
    return "Compile project";
};
