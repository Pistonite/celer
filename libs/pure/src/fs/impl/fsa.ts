//! FsFileSystem implementation for FileSystemAccess API 

import { ResultHandle } from "../../result";
import { FsFileSystem, FsFileSystemUninit } from "../FsFileSystem";
import { FsErr, FsError, FsResult, fsErr } from "../error";

type PermissionStatus = "granted" | "denied" | "prompt";

/// FileSys implementation that uses FileSystem Access API
/// This is only supported in Chrome/Edge
export class FsImplFsa implements FsFileSystemUninit {
    /// If app requested write access
    private write: boolean;
    private rootPath: string;
    private rootHandle: FileSystemDirectoryHandle;
    private permissionStatus: PermissionStatus;

    constructor(
        rootPath: string,
        rootHandle: FileSystemDirectoryHandle,
        write: boolean,
    ) {
        this.rootPath = rootPath;
        this.rootHandle = rootHandle;
        this.write = write;
        this.permissionStatus = "prompt";
    }

    public async init(r: ResultHandle): Promise<FsResult<FsFileSystem>> {
        // @ts-expect-error ts lib does not have requestPermission
        this.permissionStatus = await this.rootHandle.requestPermission({
            mode: this.write ? "readwrite" : "read",
        });
        if (this.permissionStatus !== "granted") {
            return r.putErr(fsErr(FsErr.PermissionDenied, "User denied permission"));
        }
        return r.putOk(this);
    }
}


