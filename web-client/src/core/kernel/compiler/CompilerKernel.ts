/// Interface used to access the compiler
///
/// The compiler kernel is lazy-loaded only if compiler is needed.
/// The interface provides a way for TypeScript to know about the compiler

import { EntryPointsSorted } from "low/celerc";
import { FileAccess } from "low/fs";
import { Result } from "low/utils";

/// without importing the compiler module.
export interface CompilerKernel {
    /// Initialize the compiler and bind it to a FileAccess implementation
    init(fileAccess: FileAccess): Promise<void>;

    /// Cleanup the compiler.
    ///
    /// Note that this may not stop the worker,
    /// which is automatically stopped if there is a new worker
    delete(): void;

    /// Trigger a compiler run asynchrounously
    ///
    /// The entry point will be fetched from the state and validated.
    /// If a compilation is running, there will be another run after it
    compile(): Promise<void>;

    /// Get compiler entry points
    getEntryPoints(): Promise<Result<EntryPointsSorted, unknown>>;
}
