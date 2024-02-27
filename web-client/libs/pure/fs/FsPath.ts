//! Path utilities
//!
//! The library has the following path standard:
//! - All paths are relative (without leading /) to the root
//!   of the file system (i.e. the uploaded directory)
//! - Paths are always separated by /
//! - Empty string denotes root
//! - Paths cannot lead outside of root

import { FsErr, FsResult, fsErr } from "./FsError.ts";

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
export function fsGetBase(p: string): FsResult<string> {
    if (fsIsRoot(p)) {
        const err = fsErr(FsErr.InvalidPath, "Trying to get the base of root");
        return { err };
    }
    const i = Math.max(p.lastIndexOf("/"), p.lastIndexOf("\\"));
    if (i < 0) {
        return { val: fsRoot() };
    }
    return { val: p.substring(0, i) };
}

/// Get the name of a path (i.e. the last component)
///
/// Returns the last component of the path.
/// Does not include leading or trailing slashes.
///
/// If this path is the root directory, return IsRoot.
export function fsGetName(p: string): FsResult<string> {
    p = stripTrailingSlashes(p);
    if (fsIsRoot(p)) {
        const err = fsErr(FsErr.IsRoot, "Root directory has no name");
        return { err };
    }
    const i = Math.max(p.lastIndexOf("/"), p.lastIndexOf("\\"));
    if (i < 0) {
        return { val: p };
    }
    return { val: p.substring(i + 1) };
}

/// Normalize .. and . in a path
///
/// Returns InvalidPath if the path tries to escape the root directory.
export function fsNormalize(p: string): FsResult<string> {
    let s = fsRoot();
    for (const comp of fsComponents(p)) {
        if (comp === "..") {
            const base = fsGetBase(s);
            if (base.err) {
                return base;
            }
            s = base.val;
            continue;
        }
        s = fsJoin(s, comp);
    }
    return { val: s };
}

/// Join two paths
export function fsJoin(p1: string, p2: string): string {
    if (fsIsRoot(p1)) {
        return p2;
    }
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
