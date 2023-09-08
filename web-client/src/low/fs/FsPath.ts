
import { FsResult, FsResultCode } from "./FsResult";

/// File system path
///
/// This is an abstraction on path to a file/directory in a FileSys,
/// so that we can have consistency between path representation.
/// It always represents an absolute path, relative to the root directory.
///
/// FsPath is immutable. Operations return new FsPath objects.
export interface FsPath {
    /// Returns if this path is the root directory.
    readonly isRoot: boolean;

    /// Get the parent directory of this path.
    /// 
    /// For files, return the directory it is in.
    /// For directories, return the parent directory.
    ///
    /// If this path is the root directory, return IsRoot.
    ///
    /// This does not check if the path exists.
    readonly parent: FsResult<FsPath>;

    /// Get the name of this path.
    ///
    /// Returns the last component of the path.
    /// Does not include leading or trailing slashes.
    //
    ///
    /// Examples:
    ///     "/foo/bar" -> "bar"
    ///     "/foo/bar/" -> "bar"
    ///     "/" -> IsRoot
    readonly name: FsResult<string>;

    /// Get the full path as string representation.
    ///
    /// This does not come with a leading slash.
    /// Returns an empty string for the root directory.
    readonly path: string;

    /// Resolve a descendant path.
    resolve(path: string): FsPath;

    /// Resolve a sibling path.
    ///
    /// Returns IsRoot if this is the root directory.
    resolveSibling(path: string): FsResult<FsPath>;
}

class FsPathImpl implements FsPath {
    /// Underlying path
    /// 
    /// This is the full path, with no the leading or trailing slash.
    /// For root, this is an empty string.
    private underlying: string;

    constructor(path: string) {
        this.underlying = path;
    }

    get isRoot(): boolean {
        return this.underlying === "";
    }

    get parent(): FsResult<FsPath> {
        if (this.underlying === "") {
            return {
            code:  FsResultCode.IsRoot,
            };
        }

        const i = this.underlying.lastIndexOf("/");
        if (i < 0) {
            return {
                code: FsResultCode.Ok,
                value: fsRootPath,
            };
        }
            return {
                code: FsResultCode.Ok,
                value: new FsPathImpl(this.underlying.substring(0, i)),
            };
    }

    get name(): FsResult<string> {
        if (this.underlying === "") {
            return {
            code:  FsResultCode.IsRoot,
            };
        }

        const i = this.underlying.lastIndexOf("/");
        if (i < 0) {
            return {
                code: FsResultCode.Ok,
                value: this.underlying,
            };
        }

        return {
            code: FsResultCode.Ok,
            value: this.underlying.substring(i + 1),
        };
    }

    get path(): string {
        return this.underlying;
    }

    public resolve(path: string): FsPath {
        if (path === "") {
            return this;
        }
        if (this.underlying === "") {
            return new FsPathImpl(cleanPath(path));
        }
        return new FsPathImpl(this.underlying + "/" + cleanPath(path));
    }

    public resolveSibling(path: string): FsResult<FsPath> {
        const result = this.parent;
        if (result.code !== FsResultCode.Ok) {
            return result;
        }

        const parentPath = result.value;
        const newPath = parentPath.resolve(path);
        result.value = newPath;
        return result;
    }

}

const cleanPath = (path: string) => {
    return path.replace(/^\/+|\/+$/g, "");
}

export const fsRootPath: FsPath = new FsPathImpl("");

