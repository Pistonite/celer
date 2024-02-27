import { FsErr, FsFile, FsVoid, fsErr } from "pure/fs";

import { Yielder, consoleEditor as console, createYielder } from "low/utils";

/// Track if file was modified since last time it was accessed
export interface ChangeTracker {
    /// Check if the file should be considered modified
    /// since the last time this method was called with the same path
    ///
    /// Returns the NotModified error code if the file was not modified
    checkModifiedSinceLastAccess(file: FsFile): Promise<FsVoid>
}

export function newModifyTimeBasedTracker(): ChangeTracker {
    return new ModifyTimeTracker();
}

export function newHashBasedTracker(): ChangeTracker {
    if (window.crypto.subtle) {
        return new HashTracker();
    }
    console.warn("hash based tracker requested but not supported by browser");
    return new NoopTracker();
}

class ModifyTimeTracker implements ChangeTracker {
    /// Track the last modified time of a file
    private modifiedTimeWhenLastAccessed: { [path: string]: number } = {};

    public async checkModifiedSinceLastAccess(file: FsFile): Promise<FsVoid> {
        const path = file.path;
        const modifiedTimeCurrent = await file.getLastModified();
        if (modifiedTimeCurrent.err) {
            return modifiedTimeCurrent;
        }

        const modifiedTimeLast = this.modifiedTimeWhenLastAccessed[path];
        this.modifiedTimeWhenLastAccessed[path] = modifiedTimeCurrent.val;
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

/// Track modified time against a static time stamp
export class StaticTimeTracker implements ChangeTracker {
    /// Track the last modified time of a file
    private lastTime: number;
    constructor() {
        this.lastTime = 0;
    }
    public setLastTime(time: number): void {
        this.lastTime = time;
    }

    public async checkModifiedSinceLastAccess(file: FsFile): Promise<FsVoid> {
        const modifiedTimeCurrent = await file.getLastModified();
        if (modifiedTimeCurrent.err) {
            return modifiedTimeCurrent;
        }

        if (this.lastTime > modifiedTimeCurrent.val) {
            // file was not modified since last access
            return notModified();
        }
        return {};
    }
}

class HashTracker implements ChangeTracker {
    private hashYield: Yielder;
    private hashWhenLastAccessed: { [path: string]: Uint32Array } = {};

    constructor() {
        // yield after digesting 10KB of data
        this.hashYield = createYielder(10240);
    }

    public async checkModifiedSinceLastAccess(file: FsFile): Promise<FsVoid> {
        const bytes = await file.getBytes();
        if (bytes.err) {
            return bytes;
        }
        await this.hashYield(bytes.val.length);
        const hashLast = this.hashWhenLastAccessed[file.path];
        const hashCurrent = new Uint32Array(await crypto.subtle.digest("SHA-256", bytes.val));
        this.hashWhenLastAccessed[file.path] = hashCurrent;
        if (!hashLast) {
            return {};
        }

        for (let i = 0; i < hashCurrent.length; i++) {
            if (hashCurrent[i] !== hashLast[i]) {
                console.info("file hash changed: " + file.path);
                return {};
            }
        }
        return notModified();
    }

}

/// A stub tracker that doesn't know
class NoopTracker implements ChangeTracker {
    public async checkModifiedSinceLastAccess(): Promise<FsVoid> {
        return notModified();
    }
}

function notModified(): FsVoid {
    return { err: fsErr(FsErr.NotModified, "Not modified") };
}
