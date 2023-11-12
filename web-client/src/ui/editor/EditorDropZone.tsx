import { useState } from "react";
import clsx from "clsx";
import { Body2 } from "@fluentui/react-components";

import { createFsFromDataTransferItem } from "low/fs";
import { useKernel } from "core/kernel";

/// Shown when no project is loaded, for user to drag and drop a folder in
///
/// This the only way to open a project. There's no "open" button because
/// in Firefox, dialogs are not abled to use the File Entries API.
export const EditorDropZone: React.FC = () => {
    const [isDragging, setIsDragging] = useState(false);
    const [isOpening, setIsOpening] = useState(false);
    const kernel = useKernel();
    return (
        <div
            id="editor-drop-zone"
            className={clsx(isDragging && "editor-drop-zone-dragging")}
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
                const item = e.dataTransfer?.items[0];

                if (!item) {
                    kernel.showAlert(
                        "Error",
                        "Cannot open the project. Make sure you are dropping the correct folder and try again.",
                        "Close",
                        "",
                    );
                    return;
                }
                const fileSysResult = await createFsFromDataTransferItem(item);
                await kernel.handleOpenFileSysResult(fileSysResult);
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
