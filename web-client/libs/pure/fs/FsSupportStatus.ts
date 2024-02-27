/// What is supported by the current environment
export type FsSupportStatus = {
    /// Returned by window.isSecureContext
    isSecureContext: boolean;

    /// The implementation for FsFileSystem used
    ///
    /// See README.md for more information
    implementation: "File" | "FileSystemAccess" | "FileEntry";
};

/// Get which implementation will be used for the current environment
export function fsGetSupportStatus(): FsSupportStatus {
    if (isFileSystemAccessSupported()) {
        return {
            isSecureContext: window.isSecureContext,
            implementation: "FileSystemAccess",
        };
    }
    if (isFileEntrySupported()) {
        return {
            isSecureContext: window.isSecureContext,
            implementation: "FileEntry",
        };
    }

    return {
        isSecureContext: !!window && window.isSecureContext,
        implementation: "File",
    };
}

function isFileSystemAccessSupported() {
    if (!window) {
        return false;
    }
    if (!window.isSecureContext) {
        // In Chrome, you can still access the APIs but they just crash the page entirely
        return false;
    }
    if (!window.FileSystemDirectoryHandle) {
        return false;
    }

    if (!window.FileSystemFileHandle) {
        return false;
    }

    // since TSlib doesn't have these, let's check here

    // @ts-expect-error FileSystemDirectoryHandle should have a values() method
    if (!window.FileSystemDirectoryHandle.prototype.values) {
        return false;
    }

    // @ts-expect-error window should have showDirectoryPicker
    if (!window.showDirectoryPicker) {
        return false;
    }

    return true;
}

function isFileEntrySupported(): boolean {
    if (!window) {
        return false;
    }

    // Chrome/Edge has this but it's named DirectoryEntry
    // AND, they don't work (I forgot how exactly they don't work)

    if (
        navigator &&
        navigator.userAgent &&
        navigator.userAgent.includes("Chrome")
    ) {
        return false;
    }

    if (!window.FileSystemDirectoryEntry) {
        return false;
    }

    if (!window.FileSystemFileEntry) {
        return false;
    }

    return true;
}
