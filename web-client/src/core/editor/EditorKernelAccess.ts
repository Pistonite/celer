/// Interface for editor to access kernel functions
export interface EditorKernelAccess {
    /// Reload the document, either through compiler or from server
    reloadDocument(): Promise<void>;
}
