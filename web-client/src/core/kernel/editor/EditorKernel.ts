//! Editor logic that wraps monaco editor

import { EntryPointsSorted } from "low/celerc";
import { FileSys, FsResult } from "low/fs";
import { Result } from "low/utils";

/// Interface used to access editor API
///
/// The editor kernel is lazy-loaded only if web editor is used.
/// The interface provides a way for TypeScript to know about the editor
/// without importing the editor module.
export interface EditorKernel {
    /// Initialize
    init(): Promise<void>;

    /// Delete the editor instance
    delete(): void;

    /// Reset the editor with a new file system. Unsaved changes will be lost
    reset(fs?: FileSys): Promise<void>;

    /// Request directory listing
    ///
    /// See EditorTree for input/output format
    /// On failure this returns empty array. This function will not throw
    listDir(path: string[], isUserAction: boolean): Promise<string[]>;

    /// Open a file in the editor
    openFile(path: string[], isUserAction: boolean): Promise<FsResult<void>>;

    /// Check if there are unsaved changes
    hasUnsavedChanges(): Promise<boolean>;

    /// Check if there are unsaved changes synchronously
    ///
    /// This could block UI. Use only when absolutely need to check this in
    /// a synchronous context, like window.onbeforeunload
    hasUnsavedChangesSync(): boolean;

    /// Load changes from the file system for the opened files
    loadChangesFromFs(isUserAction: boolean): Promise<FsResult<void>>;

    /// Save changes to the file system for the opened files
    saveChangesToFs(isUserAction: boolean): Promise<FsResult<void>>;

    /// Trigger a compiler run. If one is running, there will be another run after it
    compile(): void;

    /// Get compiler entry points
    getEntryPoints(): Promise<Result<EntryPointsSorted, unknown>>;
}
