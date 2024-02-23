import { ResultHandle } from "../../result";
import { errstr } from "../../utils";
import { FsFile } from "../FsFile";
import { FsFileSystemInternal } from "../FsFileSystem";
import { FsErr, FsError, FsResult, fsErr, fsFail } from "../error";

/// Allocate a new file object
export function fsFile(fs: FsFileSystemInternal, path: string): FsFile {
    return new FsFileImpl(fs, path);
}

function errclosed(): FsError {
    return fsErr(FsErr.Closed, "File is closed");
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
    }

    public isDirty(): boolean {
        return this.isBufferDirty || this.isContentNewer;
    }

    public async getLastModified(r: ResultHandle): Promise<FsResult<number>> {
        if (this.closed) {
            return r.putErr(errclosed());
        }
        if (this.lastModified === undefined) {
            r.put(await this.loadIfNotDirty(r));
            if (r.isErr()) {
                return r.ret();
            }
        }
        return r.putOk(this.lastModified ?? 0);
    }

    public async getText(r: ResultHandle): Promise<FsResult<string>> {
        if (this.closed) {
            return r.putErr(errclosed());
        }
        if (this.buffer === undefined) {
            r.put(await this.load(r));
            if (r.isErr()) {
                return r.ret();
            }
        }
        if (!this.isText) {
            return r.putErr(fsErr(FsErr.InvalidEncoding, "File is not valid UTF-8"));
        }
        return r.putOk(this.content ?? "");
    }

    public async getBytes(r: ResultHandle): Promise<FsResult<Uint8Array>> {
        if (this.closed) {
            return r.putErr(errclosed());
        }
        this.updateBuffer();
        if (this.buffer === undefined) {
            r.put(await this.load(r));
            if (r.isErr()) {
                return r.ret();
            }
        }
        if (this.buffer === undefined) {
            return r.putErr(fsFail("Read was successful, but content was undefined"));
        }
        return r.putOk(this.buffer);
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

    public async loadIfNotDirty(r: ResultHandle): Promise<FsResult<void>> {
        if (this.closed) {
            return r.putErr(errclosed());
        }
        if (this.isDirty()) {
            return r.voidOk();
        }
        return await this.load(r);
    }

    public async load(r: ResultHandle): Promise<FsResult<void>> {
        if (this.closed) {
            return r.putErr(errclosed());
        }
        r.put(await this.fs.read(r, this.path));
        if (r.isErr()) {
            return r.ret();
        }

        const file = r.value;
        // check if the file has been modified since last loaded
        if (this.lastModified !== undefined) {
            if (file.lastModified <= this.lastModified) {
                return r.voidOk();
            }
        }
        this.lastModified = file.lastModified;
        // load the buffer
        r = r.erase();
        r.put(await r.tryCatchAsync(r, async () => {
            this.buffer = new Uint8Array(await file.arrayBuffer());
        }));
        if (r.isErr()) {
            const error = fsFail(errstr(r.error));
            return r.putErr(error);
        }
        this.isBufferDirty = false;
        // Try decoding the buffer as text
        this.decodeBuffer();
        this.isContentNewer = false;
        return r.ret();
    }

    public async writeIfNewer(r: ResultHandle): Promise<FsResult<void>> {
        if (this.closed) {
            return r.putErr(errclosed());
        }
        if (!this.isDirty()) {
            return r.voidOk();
        }
        return await this.write(r);
    }

    /// Write the content without checking if it's dirty. Overwrites the file currently on FS
    ///
    /// This is private - outside code should only use writeIfDirty
    private async write(r: ResultHandle): Promise<FsResult<void>> {
        this.updateBuffer();
        const buffer = this.buffer;
        if (this.content === undefined || buffer === undefined) {
            // file was never read or modified
            return r.voidOk();
        }
        r.put(await this.fs.write(r, this.path, buffer));
        if (r.isOk()) {
            this.isBufferDirty = false;
        }
        return r;
    }

    private decodeBuffer() {
        try {
            this.content = new TextDecoder("utf-8", { fatal: true }).decode(this.buffer);
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
