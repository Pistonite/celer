//! core/editor
//! Web Editor module
import { consoleEditor as console } from "low/utils";

console.info("loading editor module");

export type * from "./EditorKernelAccess";
export type * from "./EditorKernel";
export * from "./initEditor";
export * from "./openHandler";
export * from "./dom";
