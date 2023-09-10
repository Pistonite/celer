//! Editor logic that wraps monaco editor

import { FileSys, FsResultCode } from "low/fs";

/// Interface used to access editor API
///
/// The editor kernel is lazy-loaded only if web editor is used.
/// The interface provides a way for TypeScript to know about the editor
/// without importing the editor module.
export interface EditorKernel {
    /// Reset the editor with a new file system. Unsaved changes will be lost
    reset(fs?: FileSys): Promise<void>;

    /// Request directory listing
    ///
    /// See EditorTree for input/output format
    /// On failure this returns empty array. This function will not throw
    listDir(path: string[], isUserAction: boolean): Promise<string[]>;

    /// Open a file in the editor
    openFile(path: string[], isUserAction: boolean): Promise<FsResultCode>;

    /// Check if there are unsaved changes
    hasUnsavedChanges(): boolean;

    /// Load changes from the file system for the opened files
    loadChangesFromFs(isUserAction: boolean): Promise<FsResultCode>;
}
