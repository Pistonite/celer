import { useState } from "react";
import clsx from "clsx";
import { Body2 } from "@fluentui/react-components";

import { FsResultCodes, FileSys, createFsFromDataTransferItem } from "low/fs";
import { useKernel } from "core/kernel";

type EditorDropZoneProps = {
    /// Callback when a FileSys is created from a project folder drop
    onFileSysCreate: (fileSys: FileSys) => void;
};

/// Shown when no project is loaded, for user to drag and drop a folder in
///
/// This the only way to open a project. There's no "open" button because
/// in Firefox, dialogs are not abled to use the File Entries API.
export const EditorDropZone: React.FC<EditorDropZoneProps> = ({
    onFileSysCreate,
}) => {
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
            onDrop={(e) => {
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
                createFsFromDataTransferItem(item).then(
                    async (fileSysResult) => {
                        setIsOpening(false);
                        if (fileSysResult.code === FsResultCodes.NotSupported) {
                            await kernel.showAlert(
                                "Not Supported",
                                "Your browser does not support this feature.",
                                "Close",
                                "",
                            );
                            return;
                        } else if (
                            fileSysResult.code === FsResultCodes.IsFile
                        ) {
                            await kernel.showAlert(
                                "Error",
                                "You dropped a file. Make sure you are dropping the project folder and not individual files.",
                                "Close",
                                "",
                            );
                            return;
                        } else if (fileSysResult.code !== FsResultCodes.Ok) {
                            await kernel.showAlert(
                                "Error",
                                "Cannot open the project. Make sure you have access to the folder or contact support.",
                                "Close",
                                "",
                            );
                            return;
                        }
                        const fileSys = fileSysResult.value;
                        if (!fileSys.isWritable()) {
                            const yes = await kernel.showAlert(
                                "Some feature not supported",
                                "Your browser does not support writing to file system. You will not be able to save changes made in the web editor! Do you want to open the project anyway?",
                                "Yes",
                                "No",
                            );
                            if (!yes) {
                                return;
                            }
                        }
                        onFileSysCreate(fileSys);
                    },
                );
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
