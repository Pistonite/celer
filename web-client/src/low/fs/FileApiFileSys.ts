import { allocErr, allocOk } from "low/utils";
import { FsResult, FsResultCodes } from "./FsResult";
import { FsPath } from "./FsPath";
import { FileSys } from "./FileSys";

/// FileSystem implementation that uses a list of Files
/// This is supported in all browsers, but it is stale.
/// It's used for Firefox when the File Entries API is not available
/// i.e. opened from <input type="file">
export class FileApiFileSys implements FileSys {
    private rootPath: string;
    private files: Record<string, File>;

    constructor(rootPath: string, files: Record<string, File>) {
        this.rootPath = rootPath;
        this.files = files;
    }

    public async init(): Promise<FsResult<void>> {
        return allocOk();
    }

    public getRootName(): string {
        return this.rootPath;
    }

    public async listDir(path: FsPath): Promise<FsResult<string[]>> {
        const set = new Set<string>();
        const prefix = path.path;
        Object.keys(this.files).forEach((path) => {
            if (!path.startsWith(prefix)) {
                return;
            }
            const relPath = path.slice(prefix.length);
            if (!relPath.includes("/")) {
                // file
                set.add(relPath);
            } else {
                // directory
                const dir = relPath.slice(0, relPath.indexOf("/") + 1);
                set.add(dir);
            }
        });
        return allocOk(Array.from(set));
    }

    public async readFile(path: FsPath): Promise<FsResult<File>> {
        const file = this.files[path.path];
        if (!file) {
            return allocErr(FsResultCodes.NotFound);
        }
        return allocOk(file);
    }

    public isWritable(): boolean {
        return false;
    }

    public isStale(): boolean {
        return true;
    }

    public async writeFile(): Promise<FsResult<void>> {
        // File API does not support writing
        return allocErr(FsResultCodes.NotSupported);
    }
}
