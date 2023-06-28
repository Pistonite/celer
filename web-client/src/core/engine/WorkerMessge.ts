/// Worker Message types

/// Message sent from worker to main thread for console output
export type WorkerConsoleMessage = {
    type: "console";
    payload: any[];
};

/// Message send from worker for finishing
export type WorkerDoneMessage = {
    type: "done";

};