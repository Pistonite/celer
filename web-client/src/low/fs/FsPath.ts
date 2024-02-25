// import { ResultHandle } from "pure/result";
//
// import { FsResult, FsResultCodes } from "./FsResult";
//
// /// File system path
// ///
// /// This is an abstraction on path to a file/directory in a FileSys,
// /// so that we can have consistency between path representation.
// /// It always represents an absolute path, relative to the root directory.
// ///
// /// FsPath is immutable. Operations return new FsPath objects.
// export interface FsPath {
//     /// Returns if this path is the root directory.
//     readonly isRoot: boolean;
//
//     /// Get the parent directory of this path.
//     ///
//     /// For files, return the directory it is in.
//     /// For directories, return the parent directory.
//     ///
//     /// If this path is the root directory, return IsRoot.
//     ///
//     /// This does not check if the path exists.
//     getParent(r: ResultHandle): FsResult<FsPath>;
//
//     /// Get the name of this path.
//     ///
//     /// Returns the last component of the path.
//     /// Does not include leading or trailing slashes.
//     //
//     ///
//     /// Examples:
//     ///     "/foo/bar" -> "bar"
//     ///     "/foo/bar/" -> "bar"
//     ///     "/" -> IsRoot
//     getName(r: ResultHandle): FsResult<string>;
//
//     /// Get the full path as string representation.
//     ///
//     /// This does not come with a leading slash.
//     /// Returns an empty string for the root directory.
//     readonly path: string;
//
//     /// Resolve a descendant path.
//     resolve(path: string): FsPath;
//
//     /// Resolve a sibling path.
//     ///
//     /// Returns IsRoot if this is the root directory.
//     resolveSibling(r: ResultHandle, path: string): FsResult<FsPath>;
// }
//
// class FsPathImpl implements FsPath {
//     /// Underlying path
//     ///
//     /// This is the full path, with no the leading or trailing slash.
//     /// For root, this is an empty string.
//     private underlying: string;
//
//     constructor(path: string) {
//         this.underlying = path;
//     }
//
//     get isRoot(): boolean {
//         return this.underlying === "";
//     }
//
//     getParent(r: ResultHandle): FsResult<FsPath> {
//         if (this.underlying === "") {
//             return r.putErr(FsResultCodes.IsRoot);
//         }
//
//         const i = this.underlying.lastIndexOf("/");
//         if (i < 0) {
//             return r.putOk(fsRootPath);
//         }
//         return r.putOk(new FsPathImpl(this.underlying.substring(0, i)));
//     }
//
//     getName(r: ResultHandle): FsResult<string> {
//         if (this.underlying === "") {
//             return r.putErr(FsResultCodes.IsRoot);
//         }
//
//         const i = this.underlying.lastIndexOf("/");
//         if (i < 0) {
//             return r.putOk(this.underlying);
//         }
//         return r.putOk(this.underlying.substring(i + 1));
//     }
//
//     get path(): string {
//         return this.underlying;
//     }
//
//     public resolve(path: string): FsPath {
//         if (path === "") {
//             return this;
//         }
//         if (this.underlying === "") {
//             return new FsPathImpl(cleanPath(path));
//         }
//         return new FsPathImpl(this.underlying + "/" + cleanPath(path));
//     }
//
//     // public resolveSibling(r: ResultHandle, path: string): FsResult<FsPath> {
//     //     r.put(this.getParent(r));
//     //     if (r.isErr()) {
//     //         return r;
//     //     }
//     //     return r.putOk(r.value.resolve(path));
//     // }
// }
//
// const cleanPath = (path: string) => {
//     return path.replace(/^\/+|\/+$/g, "");
// };
//
// export const fsRootPath: FsPath = new FsPathImpl("");
