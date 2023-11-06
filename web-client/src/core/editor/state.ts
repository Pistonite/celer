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
    unsavedFiles: string[];

    autoLoadActive: boolean;

    loadInProgress: boolean;
    lastLoadError: boolean;
    saveInProgress: boolean;
    lastSaveError: boolean;
    compileInProgress: boolean;
};

export const initialEditorViewState: EditorViewState = {
    serial: 0,
    supportsSave: true,
    rootPath: undefined,
    openedFile: undefined,
    currentFileSupported: true,
    unsavedFiles: [],
    autoLoadActive: true,
    loadInProgress: false,
    lastLoadError: false,
    saveInProgress: false,
    lastSaveError: false,
    compileInProgress: false,
};

export type EditorSettingsState = {
    autoSaveEnabled: boolean;
    autoLoadEnabled: boolean;
    showFileTree: boolean;
    deactivateAutoLoadAfterMinutes: number;
    compilerEntryPath: string;
    compilerUseCachePack0: boolean;
};

export const initialEditorSettingsState: EditorSettingsState = {
    autoSaveEnabled: true,
    autoLoadEnabled: false,
    showFileTree: true,
    deactivateAutoLoadAfterMinutes: 30,
    compilerEntryPath: "",
    compilerUseCachePack0: true,
};
