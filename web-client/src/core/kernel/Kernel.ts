import type { FsFileSystem } from "pure/fs";
import type { CompilerKernel } from "core/compiler";
import type { EditorKernel } from "core/editor";
import type { ExpoDoc, ExportRequest } from "low/celerc";
import type { AlertMgr } from "low/utils";

/// Kernel is the global interface for the application
/// It also owns global state and resources such as the redux store
export interface Kernel {
    /// Initialize the kernel
    init(): void;
    /// Delete the kernel
    /// May be called in dev environment when hot reloading
    delete(): void;

    /// Get the alert manager
    readonly alertMgr: AlertMgr;

    /// Get access to APIs only in EDIT mode
    /// Will throw if called in VIEW mode
    asEdit(): KernelEdit;

    /// Reload the document
    reloadDocument(): Promise<void>;

    /// Execute an export request
    exportDocument(request: ExportRequest): Promise<ExpoDoc>;
}

export interface KernelEdit {
    /// Get the editor, will be undefined if the editor is
    /// not initialized (project not opened)
    getEditor(): EditorKernel | undefined;

    /// Get or initialized the compiler.
    ensureCompiler(): Promise<CompilerKernel>;

    /// Open a project file system
    openProjectFileSystem(fs: FsFileSystem): Promise<void>;

    /// Close the opened project file system
    closeProjectFileSystem(): Promise<void>;
}
