import { FsErr, FsFile, FsVoid, fsErr } from "pure/fs";

/// Track if file was modified since last time it was accessed
export class ModifyTimeTracker {
    /// Track the last modified time of a file
    private modifiedTimeWhenLastAccessed: { [path: string]: number } = {};

    /// Check if the file should be considered modified
    /// since the last time this method was called with the same path
    ///
    /// Returns the NotModified error code if the file was not modified
    public async checkModifiedSinceLastAccess(file: FsFile): Promise<FsVoid> {
        const path = file.path;
        const modifiedTimeCurrent = await file.getLastModified();
        if (modifiedTimeCurrent.err) {
            return modifiedTimeCurrent;
        }

        const modifiedTimeLast = this.modifiedTimeWhenLastAccessed[path];
        if (!modifiedTimeLast) {
            // will be undefined if we have never seen this file before
            // so consider it modified
            return {};
        }
        if (modifiedTimeLast >= modifiedTimeCurrent.val) {
            // file was not modified since last access
            return notModified();
        }
        return {};
    }
}

function notModified(): FsVoid {
    return { err: fsErr(FsErr.NotModified, "Not modified") };
}
