//! low/fs
//!
//! File System access

// We log a message here to ensure that fs is only loaded when editor is used
// eslint-disable-next-line no-console
console.log("loading filesystem module");

export * from "./FileSys";
export * from "./FsResult";
export * from "./FsFile";
export * from "./FsPath";
export * from "./create";
