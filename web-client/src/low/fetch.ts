import { console, sleep } from "./utils";

export function fetchAsBytes(
    url: string,
    options?: RequestInit,
): Promise<Uint8Array> {
    return doFetch(url, options, async (response) => {
        const buffer = await response.arrayBuffer();
        return new Uint8Array(buffer);
    });
}

export function fetchAsString(
    url: string,
    options?: RequestInit,
): Promise<string> {
    return doFetch(url, options, (response) => {
        return response.text();
    });
}

export const fetchAsJson = <T>(
    url: string,
    options?: RequestInit,
): Promise<T> => {
    return doFetch(url, options, (response) => {
        return response.json();
    });
};

const API_PREFIX = "/api/v1";
export const getApiUrl = (path: string) => {
    return API_PREFIX + path;
};

const doFetch = async <T>(
    url: string,
    options: RequestInit | undefined,
    handler: (response: Response) => Promise<T>,
): Promise<T> => {
    const RETRY_COUNT = 3;
    let error: unknown;
    for (let i = 0; i < RETRY_COUNT; i++) {
        try {
            const response = await fetch(url, options);
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
