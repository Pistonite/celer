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

import { fsOpenReadWrite } from "pure/fs";

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
            const editor = kernel.asEdit().getEditor();
            if (!editor) {
                return;
            }

            if (await editor.hasUnsavedChanges()) {
                const yes = await kernel.alertMgr.show({
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

            await kernel.asEdit().closeProjectFileSystem();
        } else {
            // open
            // only import editor when needed, since
            // header controls are initialized in view mode as well
            const { createRetryOpenHandler } = await import("core/editor");
            const retryHandler = createRetryOpenHandler(kernel.alertMgr);
            const fs = await fsOpenReadWrite(retryHandler);
            if (fs.err) {
                return;
            }
            await kernel.asEdit().openProjectFileSystem(fs.val);
        }
    }, [kernel, rootPath]);

    return {
        icon: rootPath ? <Dismiss20Regular /> : <FolderOpen20Regular />,
        text: rootPath ? "Close project" : "Open Project",
        handler,
    };
};
