async function __initWorker(HANDLERS) {
    await wasm_bindgen("/celerc/celercwasm_bg.wasm");

    const pendingFiles = {};

    function workerOnReady() {
        wasm_bindgen.init(
            self.location.origin,
            (x) => self.postMessage(["info_fn", undefined, x]),
            (x) => self.postMessage(["warn_fn", undefined, x]),
            (x) => self.postMessage(["error_fn", undefined, x]),
            (path, checkChanged) => {
                return new Promise((resolve, reject) => {
                    setTimeout(() => {
                        pendingFiles[path] = [resolve, reject];
                        self.postMessage(["load_file", undefined, [path, checkChanged]]);
                    }, 0);
                });
            },
            async (url) => {
                for (let i = 0; i < 3; i++) {
                    const response = await fetch(url);
                    if (!response.ok) {
                        await new Promise((resolve) => {
                            setTimeout(resolve, 1000);
                        });
                        continue;
                    }
                    const data = await response.arrayBuffer();
                    return new Uint8Array(data);
                }
                throw new Error(`failed to fetch ${url}`);
            }
        );
    }

    self.onmessage = async (event) => {
        const [msgId, funcId, args] = event.data;
        if (msgId === "ready") {
            // ["ready"]
            workerOnReady();
            self.postMessage(["info_fn", undefined, "compiler worker ready"]);
            self.postMessage(["ready"]);
            return;
        }
        if (msgId === "file") {
            // ["file", 0, path, [true, data]]
            // ["file", 0, path, [false]]
            // ["file", 1, path, FsError]
            if (!pendingFiles[args]) {
                return;
            }
            const handler = pendingFiles[args][funcId];
            delete pendingFiles[args];
            setTimeout(() => handler(event.data[3]), 0);
            return;
        }
        try {
            const handler = HANDLERS[funcId];
            const result = await handler(...args);
            self.postMessage([msgId, true, result]);
        } catch (e) {
            self.postMessage([msgId, false, e]);
        }
    };
}
