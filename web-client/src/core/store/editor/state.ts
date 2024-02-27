export type EditorViewState = {
    /// Serial number
    ///
    /// Use to signal that the file system view should be rerendered
    serial: number;

    /// Root path of the opened project.
    ///
    /// Undefined means no editor is opened.
    rootPath: string | undefined;

    // Compiler stuff
    compilerReady: boolean;
    compileInProgress: boolean;

    // Editor stuff
    openedFile: string | undefined;
    currentFileSupported: boolean;
    unsavedFiles: string[];

    loadInProgress: boolean;
    lastLoadError: boolean;
    saveInProgress: boolean;
    lastSaveError: boolean;
};

export const initialEditorViewState: EditorViewState = {
    serial: 0,
    rootPath: undefined,
    compilerReady: false,
    compileInProgress: false,
    openedFile: undefined,
    currentFileSupported: true,
    unsavedFiles: [],
    loadInProgress: false,
    lastLoadError: false,
    saveInProgress: false,
    lastSaveError: false,
};

export type EditorMode = "external" | "web";

export type EditorSettingsState = {
    autoSaveEnabled: boolean;
    showFileTree: boolean;
    compilerEntryPath: string;
    compilerUseCachedPrepPhase: boolean;
    editorMode: EditorMode;
};

export const initialEditorSettingsState: EditorSettingsState = {
    autoSaveEnabled: true,
    showFileTree: true,
    compilerEntryPath: "",
    compilerUseCachedPrepPhase: true,
    editorMode: "web",
};
