import { fsSave } from "pure/fs";
import { Logger, getLogLines } from "pure/log";

export const console = new Logger("log");
export const consoleKernel = new Logger("krn");
export const consoleDoc = new Logger("doc");
export const consoleMap = new Logger("map");
export const consoleEditor = new Logger("edt");
export const consoleCompiler = new Logger("com");

/// Save the current log to a file
export const saveLog = () => {
    const result =
        confirm(`You are about to download the client-side application log to a file.

Celer does not automatically collect any user data. However, the client-side log may contain sensitive information such as the name of the files loaded in the application.

Please make sure sensitive information are removed before sharing it with developers or others for diagonistics.

Do you want to continue?

`);
    if (!result) {
        return;
    }
    const log = getLogLines().join("\n");
    const filename = `celera_${new Date().toISOString()}.log`;
    fsSave(log, filename);
};
