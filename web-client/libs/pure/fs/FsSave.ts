import { saveAs } from "file-saver";

/// Save (download) a file using Blob
export function fsSave(content: string | Uint8Array, filename: string) {
    const blob = new Blob([content], {
        // maybe lying, but should be fine
        type: "text/plain;charset=utf-8",
    });
    saveAs(blob, filename);
}
