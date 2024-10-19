import { useState } from "react";
import { Body2, mergeClasses } from "@fluentui/react-components";

import { fsOpenReadWriteFrom } from "@pistonite/pure/fs";

import { useKernel } from "core/kernel";
import { createRetryOpenHandler } from "core/editor";

import { useEditorStyles } from "./styles";

/// Shown when no project is loaded, for user to drag and drop a folder in
///
/// This the only way to open a project. There's no "open" button because
/// in Firefox, dialogs are not abled to use the File Entries API.
export const EditorDropZone: React.FC = () => {
    const [isDragging, setIsDragging] = useState(false);
    const [isOpening, setIsOpening] = useState(false);
    const kernel = useKernel();
    const styles = useEditorStyles();
    return (
        <div
            className={mergeClasses(
                styles.editorDropZone,
                isDragging && styles.editorDropZoneDragging,
            )}
            onDragEnter={() => {
                setIsDragging(true);
            }}
            onDragLeave={() => {
                setIsDragging(false);
            }}
            onDragOver={(e) => {
                setIsDragging(true);
                e.preventDefault();
                if (e.dataTransfer) {
                    // setting this will allow dropping
                    e.dataTransfer.dropEffect = "link";
                }
            }}
            onDrop={async (e) => {
                e.preventDefault();
                setIsDragging(false);
                setIsOpening(true);
                const { alertMgr } = kernel;
                const item = e.dataTransfer?.items[0];

                if (!item) {
                    await alertMgr.show({
                        title: "Error",
                        message:
                            "Cannot open the project. Make sure you are dropping the correct folder and try again.",
                        okButton: "Close",
                    });
                    return;
                }

                // create the retry handle to show error messages,
                // and ask user if they want to retry
                const retryHandler = createRetryOpenHandler(alertMgr);
                const fs = await fsOpenReadWriteFrom(item, retryHandler);
                if (fs.err) {
                    // ignore the error, because it has been alerted to the user
                    return;
                }

                await kernel.asEdit().openProjectFileSystem(fs.val);
                setIsOpening(false);
            }}
        >
            <Body2 align="center">
                {isOpening
                    ? "Opening..."
                    : "Drag and drop a project folder here to open it"}
            </Body2>
        </div>
    );
};
