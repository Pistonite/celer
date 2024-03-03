import { Buffer } from "buffer/";

import { Result, tryCatch } from "pure/result";

import { consoleKernel as console } from "low/utils";

export type DocRef = {
    owner: string;
    repo: string;
    ref: string;
    path: string;
};

/// Parse document ref from a path like /view/{owner}/{repo}[/{path}]:{ref}
/// Return undefined if parse fail
export function parseDocRef(pathname: string): DocRef | undefined {
    if (!pathname.startsWith("/view")) {
        return undefined;
    }
    const parts = pathname.substring(6).split("/").filter(Boolean);
    // parts[0] is owner
    // parts[1] is repo
    // parts[2:] is path
    // last is path:reference
    if (parts.length < 2) {
        return undefined;
    }

    const [owner, repoTemp, ...rest] = parts;
    if (!owner || !repoTemp) {
        return undefined;
    }
    let ref = "main";
    let repo = repoTemp;
    if (rest.length > 0) {
        const [last, refTemp] = rest[rest.length - 1].split(":", 2);
        rest[rest.length - 1] = last;
        if (refTemp) {
            ref = refTemp;
        }
    } else {
        // :reference might be in repo
        const [last, refTemp] = repo.split(":", 2);
        repo = last;
        if (refTemp) {
            ref = refTemp;
        }
    }
    const path = rest.join("/");

    return { owner, repo, ref, path };
}

/// Encode a JSON object as base64
export function encodeObjectAsBase64(obj: unknown): Result<string, unknown> {
    return tryCatch(() => {
        const json = JSON.stringify(obj);
        const bytes = new TextEncoder().encode(json);
        return Buffer.from(bytes).toString("base64");
    });
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
