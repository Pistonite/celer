//! low/fs
//!
//! File System access

// We log a message here to ensure that fs is only loaded when editor is used
import { console } from "low/utils";
console.info("loading file system module");

export * from "./FileAccess";
export * from "./FileSys";
export * from "./FsResult";
export * from "./FsFile";
export * from "./FsPath";
export * from "./create";
