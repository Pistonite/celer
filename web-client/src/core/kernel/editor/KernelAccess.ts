/// Interface for editor to access kernel functions
export interface KernelAccess {
    /// Reload the document, either through compiler or from server
    reloadDocument(): Promise<void>;
}
