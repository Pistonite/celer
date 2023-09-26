//! Control for closing the project in the editor

import { forwardRef, useCallback } from "react";
import { useSelector } from "react-redux";
import { MenuItem, ToolbarButton, Tooltip } from "@fluentui/react-components";
import { Dismiss20Regular } from "@fluentui/react-icons";

import { useKernel } from "core/kernel";

import { viewSelector } from "core/store";
import { ToolbarControl } from "./util";

export const CloseProject: ToolbarControl = {
    ToolbarButton: forwardRef<HTMLButtonElement>((_, ref) => {
        const handler = useCloseProjectControl();
        return (
            <Tooltip content={"Close project"} relationship="label">
                <ToolbarButton
                    ref={ref}
                    icon={<Dismiss20Regular />}
                    disabled={!handler}
                    onClick={handler}
                />
            </Tooltip>
        );
    }),
    MenuItem: () => {
        const handler = useCloseProjectControl();
        return (
            <MenuItem
                icon={<Dismiss20Regular />}
                disabled={!handler}
                onClick={handler}
            >
                Close project
            </MenuItem>
        );
    },
};

const useCloseProjectControl = () => {
    const kernel = useKernel();
    const { rootPath } = useSelector(viewSelector);
    const handler = useCallback(async () => {
        const editor = kernel.getEditor();
        if (!editor) {
            return;
        }

        if (await editor.hasUnsavedChanges()) {
            const yes = await kernel.showAlert(
                "Unsaved changes",
                "There are unsaved changes in the editor. Continue closing will discard all changes. Are you sure you want to continue?",
                "Discard changes",
                "Cancel",
            );
            if (!yes) {
                return;
            }
        }

        await editor.reset();
    }, [kernel]);

    return rootPath ? handler : undefined;
};
