//! Control for opening and closing the project in the editor
//!
//! Open project is for external editor workflow only, because the directory picker
//! does not support saving files to the file system in any browser.
//! In the web editor workflow, it shows a dialog that lets the user
//! turn on the external editor workflow.

import { forwardRef, useCallback } from "react";
import { useSelector } from "react-redux";
import { MenuItem, ToolbarButton, Tooltip } from "@fluentui/react-components";
import { Dismiss20Regular, FolderOpen20Regular } from "@fluentui/react-icons";

import { useKernel } from "core/kernel";
import { viewSelector } from "core/store";

import { ToolbarControl } from "./util";

export const OpenCloseProject: ToolbarControl = {
    ToolbarButton: forwardRef<HTMLButtonElement>((_, ref) => {
        const { icon, text, handler } = useOpenCloseProjectControl();
        return (
            <Tooltip content={text} relationship="label">
                <ToolbarButton ref={ref} icon={icon} onClick={handler} />
            </Tooltip>
        );
    }),
    MenuItem: () => {
        const { icon, text, handler } = useOpenCloseProjectControl();
        return (
            <MenuItem icon={icon} onClick={handler}>
                {text}
            </MenuItem>
        );
    },
};

const useOpenCloseProjectControl = () => {
    const kernel = useKernel();
    const { rootPath } = useSelector(viewSelector);
    const handler = useCallback(async () => {
        if (rootPath) {
            // close
            const editor = kernel.getEditor();
            if (!editor) {
                return;
            }

            if (await editor.hasUnsavedChanges()) {
                const yes = await kernel.getAlertMgr().show({
                    title: "Unsaved changes",
                    message:
                        "There are unsaved changes in the editor. Continue closing will discard all changes. Are you sure you want to continue?",
                    okButton: "Discard changes",
                    cancelButton: "Cancel",
                });
                if (!yes) {
                    return;
                }
            }

            await kernel.closeFileSys();
        } else {
            const { showDirectoryPicker } = await import("low/fs");
            const result = await showDirectoryPicker();
            await kernel.handleOpenFileSysResult(result);
        }
    }, [kernel, rootPath]);

    return {
        icon: rootPath ? <Dismiss20Regular /> : <FolderOpen20Regular />,
        text: rootPath ? "Close project" : "Open Project",
        handler,
    };
};
