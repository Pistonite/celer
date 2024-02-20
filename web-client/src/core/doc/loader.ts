//! Utilities for loading/requesting document from server

import type { ExpoContext } from "low/celerc";
import { fetchAsJson, getApiUrl } from "low/fetch";
import { console, wrapAsync } from "low/utils";

export type LoadDocumentResult = {
    type: "success";
    data: ExpoContext;
} | {
    type: "failure";
    data: string;
    help?: string;
}

const HELP_URL = "/docs/route/publish#viewing-the-route-on-celer";

/// Load the document based on the current URL (window.location.pathname)
///
/// The path should be /view/{owner}/{repo}/{path}:{reference}
export async function loadDocumentFromCurrentUrl(): Promise<LoadDocumentResult> {
    const pathname = window.location.pathname;
    if (!pathname.startsWith("/view")) {
        return createLoadError("Invalid document URL. Please double check you have the correct URL.", HELP_URL);
    }
    const parts = pathname.substring(6).split("/").filter(Boolean);
    // parts[0] is owner
    // parts[1] is repo
    // parts[2:] is path
    // last is path:reference
    if (parts.length < 2) {
        return createLoadError("Invalid document reference. Please double check you have the correct URL.", HELP_URL);
    }

    const [owner, repo, ...rest] = parts;
    if (!owner || !repo) {
        return createLoadError("Invalid document reference. Please double check you have the correct URL.", HELP_URL);
    }
    let reference = "main";
    let realRepo = repo;
    if (rest.length > 0) {
        const [last, ref] = rest[rest.length - 1].split(":", 2);
        rest[rest.length - 1] = last;
        if (ref) {
            reference = ref;
        }
    } else {
        // :reference might be in repo
        const [last, ref] = repo.split(":", 2);
        realRepo = last;
        if (ref) {
            reference = ref;
        }
    }
    const path = rest.join("/");
    return await loadDocument(owner, realRepo, reference, path);
}

function createLoadError(message: string, help: string | undefined): LoadDocumentResult {
    return {
        type: "failure",
        data: message,
        help
    };
}

export async function loadDocument(
    owner: string, 
    repo: string, 
    reference: string, 
    path: string | undefined
): Promise<LoadDocumentResult> {
    console.info(`loading document: ${owner}/${repo}/${reference} ${path}`);
    const startTime = performance.now();
    let url = `/compile/${owner}/${repo}/${reference}`;
    if (path) {
        url += `/${path}`;
    }
    const result = await wrapAsync(async () => fetchAsJson<LoadDocumentResult>(getApiUrl(url)));
    if (result.isErr()) {
        return createLoadError("There was an error loading the document from the server.", undefined);
    }
    const response = result.inner();
    const elapsed = performance.now() - startTime;
    console.info(`received resposne in ${elapsed}ms`);
    if (response.type === "success") {
        injectLoadTime(response.data, elapsed);
    }
    
    return result.inner();
}

function injectLoadTime(doc: ExpoContext, ms: number) {
    // in case the response from server is invalid, we don't want to crash the app
    try {
        doc.execDoc.project.stats["Loaded In"] = `${ms}ms`;
    } catch (e) {
        console.info("failed to inject load time");
        console.error(e);
    }
}
