//! Client side log util

import Denque from "denque";
import { errstr } from "pure/utils";

const LIMIT = 500;

/// Global log queue
const LogQueue = new Denque<string>();
function pushLog(msg: string) {
    if (LogQueue.length > LIMIT) {
        LogQueue.shift();
    }
    LogQueue.push(`[${new Date().toISOString()}]${msg}`);
}

/// Get the current log
export function getLogLines() {
    return LogQueue.toArray();
}

/// A general-purpose client side logger
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
    public error(
        msg: any /* eslint-disable-line @typescript-eslint/no-explicit-any */,
    ) {
        const msgWithPrefix = `[${this.prefix}] ${errstr(msg)}`;
        window.console.error(msgWithPrefix);
        window.console.error(msg);
        pushLog(msgWithPrefix);
    }
}
