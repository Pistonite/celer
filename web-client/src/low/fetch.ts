import { sleep } from "./utils";

export const fetchAsBytes = async (url: string): Promise<Uint8Array> => {
    const RETRY_COUNT = 3;
    let error: unknown;
    for (let i = 0; i < RETRY_COUNT; i++) {
        try {
            const response = await fetch(url, {
                cache: "reload",
            });
            if (response.ok) {
                const buffer = await response.arrayBuffer();
                return new Uint8Array(buffer);
            }
        } catch (e) {
            console.error(e);
            error = e;
            await sleep(50);
        }
    }
    if (error) {
        throw error;
    }
    throw new Error("unknown error");
}
