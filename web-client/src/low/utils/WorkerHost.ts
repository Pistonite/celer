import type { Logger } from "@pistonite/pure/log";

let worker: Worker;
/* eslint-disable @typescript-eslint/no-explicit-any */
const specialHandlers: { [key: string]: (data: any) => any } = {};
// map of [resolve, reject, timeoutHandle]
let workerHandlers: {
    [key: number]: [(x: any) => void, (x: any) => void, any];
} = {};
let nextId = 0;

export type LoadFileFn = (path: string) => Promise<Uint8Array>;

export function registerWorkerHandler(
    name: string,
    handler: (data: any) => any,
) {
    specialHandlers[name] = handler;
}

/// Set the worker and post the "ready" message
export function setWorker(w: Worker, logger: Logger) {
    if (worker) {
        logger.info("terminating existing worker");
        worker.terminate();
    }
    worker = w;
    specialHandlers["info_fn"] = logger.info.bind(logger);
    specialHandlers["warn_fn"] = logger.warn.bind(logger);
    specialHandlers["error_fn"] = logger.error.bind(logger);
    worker.onmessage = (e) => {
        const [handleId, ok, result] = e.data;
        if (typeof handleId === "string") {
            // Special handler
            const handler = specialHandlers[handleId];
            if (handler) {
                handler(result);
            }
        } else {
            // Event handler
            const handler = workerHandlers[handleId];
            if (!handler) {
                logger.warn(
                    `no worker handler for handleId=${handleId}. This could possibly be due to a previous panic from the worker.`,
                );
                return;
            }
            const [resolve, reject, timeoutHandle] = handler;
            clearTimeout(timeoutHandle);
            delete workerHandlers[handleId];
            if (ok) {
                resolve(result);
            } else {
                reject(result);
            }
        }
    };
    worker.onerror = (e) => {
        logger.error(e);
        // we don't know where the error comes from, so we reject all handlers
        const handlers = workerHandlers;
        workerHandlers = {};
        Object.values(handlers).forEach(([, reject, timeoutHandle]) => {
            clearTimeout(timeoutHandle);
            reject(e);
        });
    };
    return new Promise((resolve) => {
        let handle: any;
        specialHandlers["ready"] = () => {
            clearTimeout(handle);
            resolve(undefined);
        };
        function postReady() {
            logger.info("waiting for worker to be ready...");
            worker.postMessage(["ready"]);
            clearTimeout(handle);
            handle = setTimeout(postReady, 500);
        }
        postReady();
    });
}

/// Call a worker function
export function callWorker<T>(funcId: number, args: any[]): Promise<T> {
    return new Promise((resolve, reject) => {
        // To prevent memory leak from infinitely stuck promises
        // we set a timeout of 5 minutes.
        const timeoutHandle = setTimeout(
            () =>
                reject(`worker call timed out (msg=${nextId}, func=${funcId})`),
            300000,
        );
        workerHandlers[nextId] = [resolve, reject, timeoutHandle];
        worker.postMessage([nextId++, funcId, args]);
    });
}
