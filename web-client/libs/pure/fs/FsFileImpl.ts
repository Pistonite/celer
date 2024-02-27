import { errstr } from "pure/utils";
import { tryAsync } from "pure/result";

import { FsFile } from "./FsFile.ts";
import { FsFileSystemInternal } from "./FsFileSystemInternal.ts";
import { FsErr, FsResult, FsVoid, fsErr, fsFail } from "./FsError.ts";

/// Allocate a new file object
export function fsFile(fs: FsFileSystemInternal, path: string): FsFile {
    return new FsFileImpl(fs, path);
}

function errclosed() {
    return { err: fsErr(FsErr.Closed, "File is closed") } as const;
}

class FsFileImpl implements FsFile {
    /// The path of the file
    public path: string;

    private closed: boolean;

    /// Reference to the file system so we can read/write
    private fs: FsFileSystemInternal;
    /// If the file is text
    private isText: boolean;
    /// Bytes of the file
    private buffer: Uint8Array | undefined;
    /// If the content in the buffer is different from the content on FS
    private isBufferDirty: boolean;
    /// The content string of the file
    private content: string | undefined;
    /// If the content string is newer than the bytes
    private isContentNewer: boolean;
    /// The last modified time of the file
    private lastModified: number | undefined;

    constructor(fs: FsFileSystemInternal, path: string) {
        this.closed = false;
        this.fs = fs;
        this.path = path;
        this.isText = false;
        this.buffer = undefined;
        this.isBufferDirty = false;
        this.content = undefined;
        this.isContentNewer = false;
        this.lastModified = undefined;
    }

    public close(): void {
        this.closed = true;
        this.fs.closeFile(this.path);
    }

    public isDirty(): boolean {
        return this.isBufferDirty || this.isContentNewer;
    }

    public async getLastModified(): Promise<FsResult<number>> {
        if (this.closed) {
            return errclosed();
        }
        if (this.lastModified === undefined) {
            const r = await this.loadIfNotDirty();
            if (r.err) {
                return r;
            }
        }
        return { val: this.lastModified ?? 0 };
    }

    public async getText(): Promise<FsResult<string>> {
        if (this.closed) {
            return errclosed();
        }
        if (this.buffer === undefined) {
            const r = await this.load();
            if (r.err) {
                return r;
            }
        }
        if (!this.isText) {
            const err = fsFail("File is not valid UTF-8");
            return { err };
        }
        return { val: this.content ?? "" };
    }

    public async getBytes(): Promise<FsResult<Uint8Array>> {
        if (this.closed) {
            return errclosed();
        }
        this.updateBuffer();
        if (this.buffer === undefined) {
            const r = await this.load();
            if (r.err) {
                return r;
            }
        }
        if (this.buffer === undefined) {
            const err = fsFail(
                "Read was successful, but content was undefined",
            );
            return { err };
        }
        return { val: this.buffer };
    }

    public setText(content: string): void {
        if (this.closed) {
            return;
        }
        if (this.content === content) {
            return;
        }
        this.content = content;
        this.isContentNewer = true;
        this.lastModified = new Date().getTime();
    }

    public setBytes(content: Uint8Array): void {
        if (this.closed) {
            return;
        }
        this.buffer = content;
        this.isBufferDirty = true;
        this.decodeBuffer();
        this.isContentNewer = true;
        this.lastModified = new Date().getTime();
    }

    public async loadIfNotDirty(): Promise<FsVoid> {
        if (this.closed) {
            return errclosed();
        }
        if (this.isDirty()) {
            return {};
        }
        return await this.load();
    }

    public async load(): Promise<FsVoid> {
        if (this.closed) {
            return errclosed();
        }
        const { val: file, err } = await this.fs.read(this.path);
        if (err) {
            return { err };
        }

        // check if the file has been modified since last loaded
        if (this.lastModified !== undefined) {
            if (file.lastModified <= this.lastModified) {
                return {};
            }
        }
        this.lastModified = file.lastModified;
        // load the buffer
        const buffer = await tryAsync(
            async () => new Uint8Array(await file.arrayBuffer()),
        );
        if ("err" in buffer) {
            const err = fsFail(errstr(buffer.err));
            return { err };
        }
        this.buffer = buffer.val;
        this.isBufferDirty = false;
        // Try decoding the buffer as text
        this.decodeBuffer();
        this.isContentNewer = false;
        return {};
    }

    public async writeIfNewer(): Promise<FsVoid> {
        if (this.closed) {
            return errclosed();
        }
        if (!this.isDirty()) {
            return {};
        }
        return await this.write();
    }

    /// Write the content without checking if it's dirty. Overwrites the file currently on FS
    ///
    /// This is private - outside code should only use writeIfDirty
    private async write(): Promise<FsVoid> {
        this.updateBuffer();
        const buffer = this.buffer;
        if (this.content === undefined || buffer === undefined) {
            // file was never read or modified
            return {};
        }
        const result = await this.fs.write(this.path, buffer);
        if (result.err) {
            return result;
        }
        this.isBufferDirty = false;
        return {};
    }

    private decodeBuffer() {
        try {
            this.content = new TextDecoder("utf-8", { fatal: true }).decode(
                this.buffer,
            );
            this.isText = true;
        } catch (_) {
            this.content = undefined;
            this.isText = false;
        }
    }

    /// Encode the content to buffer if it is newer
    private updateBuffer() {
        if (!this.isContentNewer || this.content === undefined) {
            return;
        }
        const encoder = new TextEncoder();
        this.buffer = encoder.encode(this.content);
        this.isBufferDirty = true;
        this.isContentNewer = false;
    }
}
