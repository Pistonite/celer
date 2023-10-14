import { console, sleep } from "./utils";

export const fetchAsBytes = (url: string): Promise<Uint8Array> => {
    return doFetch(url, async (response) => {
        const buffer = await response.arrayBuffer();
        return new Uint8Array(buffer);
    });
};

export const fetchAsString = (url: string): Promise<string> => {
    return doFetch(url, (response) => {
        return response.text();
    });
};

const API_PREFIX = "/api/v1";
export const getApiUrl = (path: string) => {
    return API_PREFIX + path;
};

const doFetch = async <T>(
    url: string,
    handler: (response: Response) => Promise<T>,
): Promise<T> => {
    const RETRY_COUNT = 3;
    let error: unknown;
    for (let i = 0; i < RETRY_COUNT; i++) {
        try {
            const response = await fetch(url);
            if (response.ok) {
                return await handler(response);
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
};
