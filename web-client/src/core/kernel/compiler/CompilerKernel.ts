import { EntryPointsSorted, ExpoDoc, ExportRequest } from "low/celerc";
import { FileAccess } from "low/fs";
import { Result } from "low/utils";

/// Interface used to access the compiler
///
/// The compiler kernel is lazy-loaded only if compiler is needed.
/// The interface provides a way for TypeScript to know about the compiler
/// without importing the compiler module.
export interface CompilerKernel {
    /// Initialize the compiler and bind it to a FileAccess implementation
    init(fileAccess: FileAccess): Promise<void>;

    /// Unbind the compiler.
    ///
    /// Note that this does not terminate the worker. The worker will be
    /// terminated when a new one is created.
    uninit(): void;

    /// Trigger a compiler run asynchrounously
    ///
    /// The entry point will be fetched from the state and validated.
    /// If a compilation is running, there will be another run after it
    compile(): Promise<void>;

    /// Get compiler entry points
    getEntryPoints(): Promise<Result<EntryPointsSorted, unknown>>;

    /// Export the document with the given request
    ///
    /// Any error will be stored in the return value. This function will not throw
    export(request: ExportRequest): Promise<ExpoDoc>;
}
