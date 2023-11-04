import { Logger } from "./Logger";

let worker: Worker;
/* eslint-disable @typescript-eslint/no-explicit-any */
const specialHandlers: { [key: string]: (data: any) => any } = {};
const workerHandlers: { [key: number]: [(x: any) => void, (x: any) => void] } = {};
let nextId = 0;

export type LoadFileFn = (path: string) => Promise<Uint8Array>;

/// Set the worker and post the "ready" message
export function setWorker(w: Worker, logger: Logger, loadFile: LoadFileFn ) {
    if (worker) {
        worker.terminate();
    }
    worker = w;
    specialHandlers["info_fn"] = logger.info.bind(logger);
    specialHandlers["warn_fn"] = logger.warn.bind(logger);
    specialHandlers["error_fn"] = logger.error.bind(logger);
    specialHandlers["load_file"] = async (x: string) => {
        try {
            const result = await loadFile(x);
            worker.postMessage(["file", 0, [x, result]]);
        } catch (e) {
            worker.postMessage(["file", 1, [x, e]]);
        }
    };
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
            const [resolve, reject] = workerHandlers[handleId];
            delete workerHandlers[handleId];
            if (ok) {
                resolve(result);
            } else {
                reject(result);
            }
        }
    }
    worker.onerror = (e) => {
        console.error(e);
    }
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
            handle = setTimeout(postReady, 100);
        }
        postReady();
    });
}

/// Call a worker function
export function callWorker<T>(funcId: number, args: any[]): Promise<T> {
    return new Promise((resolve, reject) => {
        workerHandlers[nextId] = [resolve, reject];
        worker.postMessage([nextId++, funcId, args]);
    });
}
