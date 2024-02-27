export { fsSave } from "./FsSave.ts";
export {
    fsOpenRead,
    fsOpenReadWrite,
    fsOpenReadFrom,
    fsOpenReadWriteFrom,
} from "./FsOpen.ts";
export { fsGetSupportStatus } from "./FsSupportStatus.ts";
export {
    fsRoot,
    fsIsRoot,
    fsGetBase,
    fsGetName,
    fsNormalize,
    fsJoin,
    fsComponents,
} from "./FsPath.ts";
export { FsErr, fsErr, fsFail } from "./FsError.ts";

export type { FsOpenRetryHandler } from "./FsOpen.ts";
export type { FsSupportStatus } from "./FsSupportStatus.ts";
export type {
    FsFileSystem,
    FsFileSystemUninit,
    FsCapabilities,
} from "./FsFileSystem.ts";
export type { FsFile } from "./FsFile.ts";
export type { FsError, FsResult, FsVoid } from "./FsError.ts";
