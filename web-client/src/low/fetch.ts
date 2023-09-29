import { sleep } from "./utils";

export const fetchAsBytes = async (url: string): Promise<Uint8Array> => {
    const RETRY_COUNT = 3;
    let error: unknown;
    for (let i = 0; i < RETRY_COUNT; i++) {
        try {
            const response = await fetch(url);
            if (response.ok) {
                return new Uint8Array(await response.arrayBuffer());
            }
        } catch (e) {
            console.error(e);
            error = e;
            await sleep(50);
        }
    }
    throw error;
}
