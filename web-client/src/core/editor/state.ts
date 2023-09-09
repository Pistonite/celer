export type EditorViewState = {
    /// Serial number
    ///
    /// Use to signal that the file system view should be rerendered
    serial: number;

    /// Root path of the opened project.
    ///
    /// Undefined means no editor is opened.
    rootPath: string | undefined;
    supportsSave: boolean;
    openedFile: string | undefined;
    currentFileSupported: boolean;
    hasUnsavedChanges: boolean;
    autoLoadActive: boolean;

    loadInProgress: boolean;
    lastLoadError: boolean;
    saveInProgress: boolean;
};

export const initialEditorViewState: EditorViewState = {
    serial: 0,
    supportsSave: true,
    rootPath: undefined,
    openedFile: undefined,
    currentFileSupported: true,
    hasUnsavedChanges: false,
    autoLoadActive: true,
    loadInProgress: false,
    lastLoadError: false,
    saveInProgress: false,
};

export type EditorSettingsState = {
    autoSaveEnabled: boolean;
    autoLoadEnabled: boolean;
    deactivateLoadAfterMinutes: number;
};

export const initialEditorSettingsState: EditorSettingsState = {
    autoSaveEnabled: true,
    autoLoadEnabled: true,
    deactivateLoadAfterMinutes: 30,
};
