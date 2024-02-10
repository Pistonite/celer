// @ts-expect-error no types for this library
import FileSaverFunction from "./FileSaver";

export const saveAs = (
    content: string | Uint8Array,
    filename: string,
): void => {
    const blob = new Blob([content], {
        // maybe lying, but should be fine
        type: "text/plain;charset=utf-8",
    });
    FileSaverFunction(blob, filename);
};
