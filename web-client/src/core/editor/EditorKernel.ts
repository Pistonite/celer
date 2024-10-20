//! Editor logic that wraps monaco editor

import type { CompilerFileAccess } from "core/compiler";

/// Interface used to access editor API
///
/// The editor kernel is lazy-loaded only if web editor is used.
/// The interface provides a way for TypeScript to know about the editor
/// without importing the editor module.
export interface EditorKernel {
    /// Delete the editor instance
    delete(): void;

    /// Nofity the editor that the user is active
    notifyActivity(): void;

    /// Request directory listing
    ///
    /// See EditorTree for input/output format
    /// On failure this returns empty array. This function will not throw
    listDir(path: string): Promise<string[]>;

    /// Open a file in the editor
    openFile(path: string): Promise<void>;

    /// Check if there are unsaved changes
    hasUnsavedChanges(): Promise<boolean>;

    /// Check if there are unsaved changes synchronously
    ///
    /// This could block UI. Use only when absolutely need to check this in
    /// a synchronous context, like window.onbeforeunload
    hasUnsavedChangesSync(): boolean;

    /// Load changes from the file system for the opened non-dirty files
    loadFromFs(): Promise<void>;

    /// Save changes to the file system for the opened files
    saveToFs(): Promise<void>;

    /// Get a CompilerFileAccess implementation
    getFileAccess(): CompilerFileAccess;
}
