//! Path utilities
//!
//! The library has the following path standard:
//! - All paths are relative (without leading /) to the root 
//!   of the file system (i.e. the uploaded directory)
//! - Paths are always separated by /
//! - Empty string denotes root
//! - Paths cannot lead outside of root

import { ResultHandle } from "../result";
import { FsErr, FsResult, fsErr } from "./error";

/// Get the root path. Current implementation is empty string.
export function fsRoot(): string {
    return "";
}

/// Check if a path is the root directory, also handles badly formatted paths like ".///../"
export function fsIsRoot(p: string): boolean {
    if (!p) {
        return true;
    }
    for (let i = 0; i < p.length; i++) {
        if (p[i] !== "/" || p[i] !== "." || p[i] !== "\\") {
            return false;
        }
    }
    return true;
}

/// Get the base name of a path (i.e. remove the last component)
///
/// If this path is the root directory, return InvalidPath.
export function fsGetBase(r: ResultHandle, p: string): FsResult<string> {
    if (fsIsRoot(p)) {
        return r.putErr(fsErr(FsErr.InvalidPath, "Trying to get the parent of root"));
    }
    const i = Math.max(p.lastIndexOf("/"), p.lastIndexOf("\\"));
    if (i < 0) {
        return r.putOk(fsRoot());
    }
    return r.putOk(p.substring(0, i));
}

/// Get the name of a path (i.e. the last component)
///
/// Returns the last component of the path.
/// Does not include leading or trailing slashes.
///
/// If this path is the root directory, return IsRoot.
export function fsGetName(r: ResultHandle, p: string): FsResult<string> {
    p = stripTrailingSlashes(p);
    if (fsIsRoot(p)) {
        return r.putErr(fsErr(FsErr.IsRoot, "Root directory has no name"));
    }
    const i = Math.max(p.lastIndexOf("/"), p.lastIndexOf("\\"));
    if (i < 0) {
        return r.putOk(p);
    }
    return r.putOk(p.substring(i + 1));
}

/// Normalize .. and . in a path
///
/// Returns InvalidPath if the path tries to escape the root directory.
export function fsNormalize(r: ResultHandle, p: string): FsResult<string> {
    let s = fsRoot();
    for (const comp of fsComponents(p)) {
        if (comp === "..") {
            r.put(fsGetBase(r, s));
            if (r.isErr()) {
                return r;
            }
            s = r.value;
            continue;
        }
        s = fsJoin(s, comp);
    }
    return r.putOk(s);
}

/// Join two paths
export function fsJoin(p1: string, p2: string): string {
    return p1 + "/" + p2;
}

/// Iterate through the components of a path. Empty components and . are skipped
export function* fsComponents(p: string): Iterable<string> {
    let i = 0;
    while (i < p.length) {
        let nextSlash = p.indexOf("/", i);
        if (nextSlash < 0) {
            nextSlash = p.length;
        }
        let nextBackslash = p.indexOf("\\", i);
        if (nextBackslash < 0) {
            nextBackslash = p.length;
        }
        let j = Math.min(nextSlash, nextBackslash);
        if (j < 0) {
            j = p.length;
        }
        const c = p.substring(i, j);
        if (c && c !== ".") {
            yield c;
        }
        i = j + 1;
    }
}

/// Remove trailing slashes from a path
function stripTrailingSlashes(p: string): string {
    let i = p.length - 1;
    for (; i >= 0; i--) {
        if (p[i] !== "/" && p[i] !== "\\") {
            break;
        }
    }
    if (i === p.length - 1) {
        return p;
    }
    return p.substring(0, i + 1);
}
