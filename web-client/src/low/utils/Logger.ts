//! Client side log util

import Denque from "denque";
import { saveAs } from "./FileSaver";
/// Global log queue
const LogQueue = new Denque<string>();
const pushLog = (msg: string) => {
    if (LogQueue.length > 500) {
        LogQueue.shift();
    }
    LogQueue.push(`[${new Date().toISOString()}]${msg}`);
};

/// Get the current log
export const getLog = () => {
    return LogQueue.toArray();
};

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
    const log = getLog().join("\n");
    const filename = `celer_web-client_${new Date().toISOString()}.log`;
    saveAs(log, filename);
};

/// A general-purpose client side logger
///
/// We are not collecting telemetry on server side,
/// so we want to have some logs/diagonistics on the client side
export class Logger {
    /// The prefix of the logger
    private prefix: string;

    constructor(prefix: string) {
        this.prefix = prefix;
    }

    /// Log an info message
    public info(msg: string) {
        const msgWithPrefix = `[${this.prefix}] ${msg}`;
        window.console.info(msgWithPrefix);
        pushLog(msgWithPrefix);
    }

    /// Log a warning message
    public warn(msg: string) {
        const msgWithPrefix = `[${this.prefix}] ${msg}`;
        window.console.warn(msgWithPrefix);
        pushLog(msgWithPrefix);
    }

    /// Log an error message
    public error(msg: any) { // eslint-disable-line @typescript-eslint/no-explicit-any
        const msgWithPrefix = `[${this.prefix}] ${msg}`;
        window.console.error(msgWithPrefix);
        window.console.error(msg);
        pushLog(msgWithPrefix);
    }
}
export const console = new Logger("low");
