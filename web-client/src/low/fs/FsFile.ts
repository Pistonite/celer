import { allocErr, allocOk } from "low/utils";

import { FsPath } from "./FsPath";
import { FileSys } from "./FileSys";
import { FsResult, FsResultCodes } from "./FsResult";

/// A wrapper for the concept of a virtual, opened file.
///
/// The file is lazy-loaded. It's content will only be loaded when getContent is called.
export class FsFile {
    /// Reference to the file system so we can read/write
    private fs: FileSys;
    /// The path of the file
    private path: FsPath;
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

    /// The last modified time of the file on fs when last checked
    private lastModified: number | undefined;

    constructor(fs: FileSys, path: FsPath) {
        this.fs = fs;
        this.path = path;
        this.isText = false;
        this.buffer = undefined;
        this.isBufferDirty = false;
        this.content = undefined;
        this.isContentNewer = false;
        this.lastModified = undefined;
    }

    public getDisplayPath(): string {
        return this.path.path;
    }

    public isNewerThanFs(): boolean {
        return this.isBufferDirty || this.isContentNewer;
    }

    /// Get the text content of the file
    ///
    /// If the file is not loaded, it will load it.
    ///
    /// If the file is not a text file, it will return InvalidEncoding
    ///
    /// If clearChangedSinceLastCompile is true, it will clear the flag.
    public async getText(): Promise<FsResult<string>> {
        if (this.buffer === undefined) {
            const result = await this.load();
            if (result.isErr()) {
                return result;
            }
        }
        if (!this.isText) {
            return allocErr(FsResultCodes.InvalidEncoding);
        }
        return allocOk(this.content ?? "");
    }

    public async getBytes(): Promise<FsResult<Uint8Array>> {
        this.updateBuffer();
        if (this.buffer === undefined) {
            const result = await this.load();
            if (result.isErr()) {
                return result;
            }
        }
        if (this.buffer === undefined) {
            return allocErr(FsResultCodes.Fail);
        }
        return allocOk(this.buffer);

    }

    /// Set the content in memory. Does not save to FS.
    public setContent(content: string): void {
        if (this.content === content) {
            return;
        }
        this.content = content;
        this.isContentNewer = true;
    }

    /// Load the file's content if it's not newer than fs
    ///
    /// Returns Ok if the file is newer than fs
    public async loadIfNotDirty(): Promise<FsResult<void>> {
        if (this.isNewerThanFs()) {
            return allocOk();
        }
        return await this.load();
    }

    /// Load the file's content from FS.
    ///
    /// Overwrites any unsaved changes only if the file has been
    /// modified since it was last loaded.
    ///
    /// If it fails, the file's content will not be changed
    public async load(): Promise<FsResult<void>> {
        const result = await this.fs.readFile( this.path);

        if (result.isErr()) {
            return result;
        }

        const file = result.inner();
        // check if the file has been modified since last loaded
        if (this.lastModified !== undefined) {
            if (file.lastModified <= this.lastModified) {
                return result.makeOk(undefined);
            }
        }
        this.lastModified = file.lastModified;
        // load the buffer
        try {
            this.buffer = new Uint8Array(await file.arrayBuffer());
        } catch (e) {
            console.error(e);
            return result.makeErr(FsResultCodes.Fail);
        }
        this.isBufferDirty = false;
        // Try decoding the buffer as text
        try {
            this.content = new TextDecoder("utf-8", { fatal: true }).decode( this.buffer,);
            this.isText = true;
        } catch (e) {
            console.error(e);
            this.content = undefined;
            this.isText = false;
        }
        this.isContentNewer = false;
        return result.makeOk(undefined);
    }

    /// Save the file's content to FS if it is newer.
    ///
    /// If not dirty, returns Ok
    public async writeIfNewer(): Promise<FsResult<void>> {
        if (!this.isNewerThanFs()) {
            return allocOk();
        }
        return await this.write();
    }

    /// Write the content without checking if it's dirty. Overwrites the file currently on FS
    ///
    /// This is private - outside code should only use writeIfDirty
    private async write(): Promise<FsResult<void>> {
        this.updateBuffer();
        const buffer = this.buffer;
        if (this.content === undefined || buffer === undefined) {
            // file was never read or modified
            return allocOk();
        }
        const result = await this.fs.writeFile(this.path, buffer);
        if (result.isErr()) {
            return result;
        }
        this.isBufferDirty = false;
        return result.makeOk(undefined);
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
