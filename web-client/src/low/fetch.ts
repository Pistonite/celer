import type { Result } from "@pistonite/pure/result";

import { sleep } from "./utils";

export function fetchAsBytes(
    url: string,
    options?: RequestInit,
): Promise<Result<Uint8Array, unknown>> {
    return doFetch(url, options, async (response) => {
        const buffer = await response.arrayBuffer();
        return new Uint8Array(buffer);
    });
}

export function fetchAsString(
    url: string,
    options?: RequestInit,
): Promise<Result<string, unknown>> {
    return doFetch(url, options, (response) => {
        return response.text();
    });
}

export const fetchAsJson = <T>(
    url: string,
    options?: RequestInit,
): Promise<Result<T, unknown>> => {
    return doFetch(url, options, (response) => {
        return response.json();
    });
};

const API_PREFIX = "/api/v1";
export const getApiUrl = (path: string) => {
    return API_PREFIX + path;
};

async function doFetch<T>(
    url: string,
    options: RequestInit | undefined,
    handler: (response: Response) => Promise<T>,
): Promise<Result<T, unknown>> {
    const RETRY_COUNT = 3;
    let error: unknown;
    for (let i = 0; i < RETRY_COUNT; i++) {
        try {
            const response = await fetch(url, options);
            if (response.ok) {
                const val = await handler(response);
                return { val };
            }
        } catch (e) {
            error = e;
            await sleep(50);
        }
    }
    if (error) {
        return { err: error };
    }
    return { err: new Error("unknown error") };
}
