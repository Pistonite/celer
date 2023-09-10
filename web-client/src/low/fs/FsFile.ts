import { FsPath } from "./FsPath";
import { FileSys } from "./FileSys";
import { FsResult, FsResultCode, FsResultCodes } from "./FsResult";

/// A wrapper for the concept of a virtual, opened file.
///
/// The file is lazy-loaded. It's content will only be loaded when getContent is called.
export class FsFile {
    /// Reference to the file system so we can read/write
    private fs: FileSys;
    /// The path of the file
    private path: FsPath;
    /// If the file is dirty (has unsaved changes)
    private dirty: boolean;
    /// The content of the file (in memory)
    private content: string | undefined;
    /// The last modified time of the file on fs when last checked
    private lastModified: number | undefined;

    constructor(fs: FileSys, path: FsPath) {
        this.fs = fs;
        this.path = path;
        this.dirty = false;
        this.content = undefined;
        this.lastModified = undefined;
    }

    public getDisplayPath(): string {
        return this.path.path;
    }

    public isDirty(): boolean {
        return this.dirty;
    }

    /// Get or load the content of the file
    public async getContent(): Promise<FsResult<string>> {
        if (this.content === undefined) {
            const result = await this.load();
            if (result !== FsResultCodes.Ok) {
                return { code: result };
            }
        }
        return {
            code: FsResultCodes.Ok,
            value: this.content ?? "",
        };
    }

    /// Set the content in memory. Does not save to FS.
    public setContent(content: string): void {
        if (content === this.content) {
            return;
        }
        this.content = content;
        this.dirty = true;
    }

    /// Load the file's content if it's not dirty
    ///
    /// Returns Ok if the file is dirty
    public async loadIfNotDirty(): Promise<FsResultCode> {
        if (this.dirty) {
            return FsResultCodes.Ok;
        }
        return await this.load();
    }

    /// Load the file's content from FS.
    ///
    /// Overwrites any unsaved changes only if the file has been
    /// modified since it was last loaded.
    ///
    /// If it fails, the file's content will not be changed
    public async load(): Promise<FsResultCode> {
        const result = await this.fs.readIfModified(
            this.path,
            this.lastModified,
        );
        if (result.code === FsResultCodes.NotModified) {
            return FsResultCodes.Ok;
        }
        if (result.code !== FsResultCodes.Ok) {
            return result.code;
        }
        const [content, lastModified] = result.value;
        this.content = content;
        this.lastModified = lastModified;
        this.dirty = false;
        return FsResultCodes.Ok;
    }

    /// Save the file's content to FS if it is dirty.
    ///
    /// If not dirty, returns Ok
    public async writeIfDirty(): Promise<FsResultCode> {
        if (!this.dirty) {
            return FsResultCodes.Ok;
        }
        return await this.write();
    }

    /// Write the content without checking if it's dirty. Overwrites the file currently on FS
    ///
    /// This is private - outside code should only use writeIfDirty
    private async write(): Promise<FsResultCode> {
        if (this.content === undefined) {
            // file was never read, so no need to save
            return FsResultCodes.Ok;
        }
        const result = await this.fs.writeFile(this.path, this.content);
        if (result !== FsResultCodes.Ok) {
            return result;
        }
        this.dirty = false;
        return FsResultCodes.Ok;
    }
}
