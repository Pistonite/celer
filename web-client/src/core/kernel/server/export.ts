//! Calls the /export endpoint

import type { ExpoDoc, ExportRequest, PluginOptions } from "low/celerc";
import { fetchAsJson, getApiUrl } from "low/fetch";
import { consoleKernel as console } from "low/utils";

import { DocRef, encodeObjectAsBase64, parseDocRef } from "./utils.ts";

function createExportError(error: string): ExpoDoc {
    return { error };
}

export async function sendExportRequest(
    pluginOptions: PluginOptions | undefined,
    request: ExportRequest,
): Promise<ExpoDoc> {
    const docRef = parseDocRef(window.location.pathname);
    if (!docRef) {
        return createExportError("Invalid document reference for export.");
    }
    return await sendExportRequestForRef(docRef, pluginOptions, request);
}

async function sendExportRequestForRef(
    docRef: DocRef,
    pluginOptions: PluginOptions | undefined,
    request: ExportRequest,
): Promise<ExpoDoc> {
    const { owner, repo, ref, path } = docRef;
    console.info(`export document: ${owner}/${repo}/${ref} ${path}`);
    const startTime = performance.now();
    let url = `/export/${owner}/${repo}/${ref}`;
    if (path) {
        url += `/${path}`;
    }
    const headers: Record<string, string> = {};
    if (pluginOptions) {
        const optionsValue = encodeObjectAsBase64(pluginOptions);
        if ("err" in optionsValue) {
            console.error(optionsValue.err);
            return createExportError("Failed to encode plugin options");
        }
        headers["Celer-Plugin-Options"] = optionsValue.val;
    }
    const requestValue = encodeObjectAsBase64(request);
    if ("err" in requestValue) {
        console.error(requestValue.err);
        return createExportError("Failed to encode export request");
    }
    headers["Celer-Export-Request"] = requestValue.val;

    const result = await fetchAsJson<ExpoDoc>(getApiUrl(url), { headers });
    if ("err" in result) {
        const err = result.err;
        console.error(err);
        return createExportError(
            "There was an error sending export request to the server.",
        );
    }
    const doc = result.val;

    const elapsed = Math.round(performance.now() - startTime);
    console.info(`received resposne in ${elapsed}ms`);
    return doc;
}
