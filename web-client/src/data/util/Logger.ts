//! Client side log util

import Denque from "denque";

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
        console.info(msgWithPrefix);
        pushLog(msgWithPrefix);
    }

    /// Log a warning message
    public warn(msg: string) {
        const msgWithPrefix = `[${this.prefix}] ${msg}`;
        console.warn(msgWithPrefix);
        pushLog(msgWithPrefix);
    }

    /// Log an error message
    public error(msg: string) {
        const msgWithPrefix = `[${this.prefix}] ${msg}`;
        console.error(msgWithPrefix);
        pushLog(msgWithPrefix);
    }
}
