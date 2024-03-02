//! Utilities for interaction in VIEW mode

import { Buffer } from "buffer/";

import { tryAsync } from "pure/result";

import type { ExpoContext, PluginOptionsRaw } from "low/celerc";
import { fetchAsJson, getApiUrl } from "low/fetch";
import { consoleDoc as console } from "low/utils";

export type LoadDocumentResult =
    | {
          type: "success";
          data: ExpoContext;
      }
    | {
          type: "failure";
          data: string;
          help?: string;
      };

const HELP_URL = "/docs/route/publish#viewing-the-route-on-celer";

/// Load the document based on the current URL (window.location.pathname)
///
/// The path should be /view/{owner}/{repo}/{path}:{reference}
export async function loadDocumentFromCurrentUrl(
    pluginOptions: PluginOptionsRaw | undefined,
): Promise<LoadDocumentResult> {
    const pathname = window.location.pathname;
    if (!pathname.startsWith("/view")) {
        return createLoadError(
            "Invalid document URL. Please double check you have the correct URL.",
            HELP_URL,
        );
    }
    const parts = pathname.substring(6).split("/").filter(Boolean);
    // parts[0] is owner
    // parts[1] is repo
    // parts[2:] is path
    // last is path:reference
    if (parts.length < 2) {
        return createLoadError(
            "Invalid document reference. Please double check you have the correct URL.",
            HELP_URL,
        );
    }

    const [owner, repo, ...rest] = parts;
    if (!owner || !repo) {
        return createLoadError(
            "Invalid document reference. Please double check you have the correct URL.",
            HELP_URL,
        );
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
    return await loadDocument(owner, realRepo, reference, path, pluginOptions);
}

function createLoadError(
    message: string,
    help: string | undefined,
): LoadDocumentResult {
    return {
        type: "failure",
        data: message,
        help,
    };
}

export async function loadDocument(
    owner: string,
    repo: string,
    reference: string,
    path: string | undefined,
    pluginOptions: PluginOptionsRaw | undefined,
): Promise<LoadDocumentResult> {
    console.info(`loading document: ${owner}/${repo}/${reference} ${path}`);
    const startTime = performance.now();
    let url = `/compile/${owner}/${repo}/${reference}`;
    if (path) {
        url += `/${path}`;
    }
    const headers: Record<string, string> = {};
    if (pluginOptions) {
        try {
            const optionsJson = JSON.stringify(pluginOptions);
            const optionsBytes = new TextEncoder().encode(optionsJson);
            const optionsBase64 = Buffer.from(optionsBytes).toString("base64");
            headers["Celer-Plugin-Options"] = optionsBase64;
        } catch (e) {
            console.error(e);
            console.error("failed to encode plugin options");
            return createLoadError(
                "Failed to encode plugin options",
                undefined,
            );
        }
    }
    const result = await tryAsync(() =>
        fetchAsJson<LoadDocumentResult>(getApiUrl(url), { headers }),
    );
    if ("err" in result) {
        const err = result.err;
        console.error(err);
        return createLoadError(
            "There was an error loading the document from the server.",
            undefined,
        );
    }
    const response = result.val;
    const elapsed = Math.round(performance.now() - startTime);
    console.info(`received resposne in ${elapsed}ms`);
    if (response.type === "success") {
        injectLoadTime(response.data, elapsed);
    } else {
        if (!response.help) {
            response.help = HELP_URL;
        }
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

/// Get the server provided document title, for configuring the load request
export function getPreloadedDocumentTitle(): string | undefined {
    const meta = document.querySelector("meta[name='preload-title']");
    if (!meta) {
        if (window.location.hash) {
            const title = decodeURIComponent(window.location.hash.substring(1));
            console.info(`using preloaded title from URL hash: ${title}`);
            return title || undefined;
        } else {
            console.warn(
                "cannot find preloaded document title. This is a bug if you are in production env",
            );
            console.info(
                "for dev environment, append the URL encoded title after # in the URL",
            );
            return undefined;
        }
    }

    const title = meta.getAttribute("content") || undefined;
    console.info(`using preloaded title from server: ${title}`);
    return title;
}
