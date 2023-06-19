import WorkerSource from "./worker.min.js?raw";
import ConsoleSource from "./console.min.js?raw";

console.log("Loading engine");

// type EnginePlugin = {
//     /// display name of the engine plugin
//     name: string;
//     /// Callback that will be executed when the engine is initialized.
//     init?: () => void;
//     /// Callback that will be executed when the document is finished
//     done?: () => void;
//     /// Called when the engine is about to process a line of input.
//     onLine?: (line: string) => string;
// }

// type EngineWindow = {
//     console: {
//         log: (...message: any) => void;
//     }
// }

const testengine = `{
    "name": "testengine",
    "init": () => {
        console.log(syntax error);
    }
}`;

export const testExec = async (): Promise<string> => {
    const plugin = testengine; // this should be checked plugin source code
    // construct and start web worker
    const constructPluginExpr = (name: string, plugin: string) => {
        const body = `var __plugin_name__=${JSON.stringify(name)};${ConsoleSource};return (${plugin});`;
        return `((function(){${body}})())`;
    }
    const magic = "(window._INJECTED_PLUGINS);";
    if (!WorkerSource.endsWith(magic)) {
        throw new Error("WorkerSource does not end with magic");
    }

    const startMagic = "\"use strict\";";
    if (!WorkerSource.startsWith(startMagic)) {
        throw new Error("WorkerSource does not start with magic");
    }

    const WorkerSourceWithoutMagic = WorkerSource.slice(startMagic.length, -magic.length);
    const WorkerSourceWithPlugin = "\"use strict\";var exports={};" + WorkerSourceWithoutMagic + "([" + constructPluginExpr("testengine1", plugin) + "]);";
    const blob = new Blob([WorkerSourceWithPlugin], { type: "application/javascript" });
    const url = URL.createObjectURL(blob);
    let worker = new Worker(url);

    const promise = new Promise<string>((resolve, reject) => {
        worker.addEventListener("message", (event) => {
            console.log("Worker message:", event.data);
            if (event.data.type === "done") {
                console.log("Worker done")
                resolve(event.data);
            }
        });
        worker.addEventListener("error", (event) => {
            console.error("Worker error:", event);
            reject(event);
        });
        worker.postMessage("start");
        setTimeout(function(){
            console.log("working timeout")
            worker.terminate();
            reject("timeout")
            //worker = null as any;
        }, 1000);
    });

    return await promise;
}

"(window._INJECTED_PLUGINS);"