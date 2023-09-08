
import { useState } from "react";
import clsx from "clsx";
import { Body2 } from "@fluentui/react-components";

import { FileSys, createFsFromDataTransferItem } from "low/fs";
import { FsResultCode } from "low/fs";
import { useKernel } from "core/kernel";

type EditorDropZoneProps = {
    /// Callback when a FileSys is created from a project folder drop
    onFileSysCreate: (fileSys: FileSys) => void;
}

/// Shown when no project is loaded, for user to drag and drop a folder in
///
/// This the only way to open a project. There's no "open" button because
/// in Firefox, dialogs are not abled to use the File Entries API.
export const EditorDropZone: React.FC<EditorDropZoneProps> = ({onFileSysCreate}) => {
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
                e.preventDefault();
                if (e.dataTransfer) {
                    // setting this will allow dropping
                    e.dataTransfer.dropEffect = "link";
                }
            }}

            onDrop={(e) => {
                e.preventDefault();
                const item = e.dataTransfer?.items[0];
                setIsDragging(false);
                setIsOpening(true);
                const doCreateFileSys = async () => {
                    if (!item) {
                        await kernel.showAlert(
                            "Error",
                            "Cannot open the project. Make sure you are dropping the correct folder and try again.",
                            "Close",
                            "",
                        );
                        return;
                    }
                    const fileSysResult = await createFsFromDataTransferItem(item);
                    if (fileSysResult.code === FsResultCode.NotSupported) {
                        await kernel.showAlert(
                            "Not Supported",
                            "Your browser does not support this feature.",
                            "Close",
                            "",
                        );
                        return;
                    } else if (fileSysResult.code !== FsResultCode.Ok) {
                        await kernel.showAlert(
                            "Error",
                            "Cannot open the project. Make sure you are dropping the correct folder (not individual files).",
                            "Close",
                            "",
                        );
                        return;
                    }
                    const fileSys = fileSysResult.value;
                    onFileSysCreate(fileSys);
                };
                setTimeout(() => {
                    doCreateFileSys();
                    setIsOpening(false);
                }, 0);
            }}
        >
            <Body2 align="center">
                {
                    isOpening ? "Opening..." : "Drag and drop a project folder here to open it"
                }
            </Body2>
        </div>
    );
};
