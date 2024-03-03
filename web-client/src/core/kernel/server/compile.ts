//! Calls the /compile endpoint

import type { ExpoContext, PluginOptionsRaw } from "low/celerc";
import { consoleKernel as console } from "low/utils";
import { fetchAsJson, getApiUrl } from "low/fetch";

import { DocRef, encodeObjectAsBase64, parseDocRef } from "./utils.ts";

export type LoadDocumentResult =
    | {
          type: "success";
          data: ExpoContext;
      }
    | {
          type: "failure";
          data: string;
      };

function createLoadError(data: string): LoadDocumentResult {
    return { type: "failure", data };
}

/// Load the document based on the current URL (window.location.pathname)
///
/// The path should be /view/{owner}/{repo}/{path}:{reference}
export async function loadDocument(
    pluginOptions: PluginOptionsRaw | undefined,
): Promise<LoadDocumentResult> {
    const docRef = parseDocRef(window.location.pathname);
    if (!docRef) {
        return createLoadError(
            "Invalid document reference. Please double check you have the correct URL.",
        );
    }
    return await loadDocumentForRef(docRef, pluginOptions);
}

async function loadDocumentForRef(
    docRef: DocRef,
    pluginOptions: PluginOptionsRaw | undefined,
): Promise<LoadDocumentResult> {
    const { owner, repo, ref, path } = docRef;
    console.info(`loading document: ${owner}/${repo}/${ref} ${path}`);
    const startTime = performance.now();
    let url = `/compile/${owner}/${repo}/${ref}`;
    if (path) {
        url += `/${path}`;
    }

    const headers: Record<string, string> = {};
    if (pluginOptions) {
        const optionsValue = encodeObjectAsBase64(pluginOptions);
        if ("err" in optionsValue) {
            console.error(optionsValue.err);
            return createLoadError("Failed to encode plugin options");
        }
        headers["Celer-Plugin-Options"] = optionsValue.val;
    }
    const result = await fetchAsJson<LoadDocumentResult>(getApiUrl(url), {
        headers,
    });
    if ("err" in result) {
        const err = result.err;
        console.error(err);
        return createLoadError(
            "There was an error loading the document from the server.",
        );
    }
    const response = result.val;

    const elapsed = Math.round(performance.now() - startTime);
    console.info(`received resposne in ${elapsed}ms`);
    if (response.type === "success") {
        injectLoadTime(response.data, elapsed);
    }

    return response;
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
