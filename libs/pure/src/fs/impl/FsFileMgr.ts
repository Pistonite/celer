import { FsFile } from "../FsFile";
import { FsFileSystemInternal } from "./FsFileSystemInternal";
import { fsFile } from "./file";

/// Internal class to track opened files
export class FsFileMgr {
    private opened: { [path: string]: FsFile };

    public constructor() {
        this.opened = {};
    }

    public get(fs: FsFileSystemInternal, path: string): FsFile {
        let file = this.opened[path];
        if (!file) {
            file = fsFile(fs, path);
            this.opened[path] = file;
        }
        return file;
    }

    public close(path: string): void {
        delete this.opened[path];
    }

    public getOpenedPaths(): string[] {
        return Object.keys(this.opened);
    }
}
