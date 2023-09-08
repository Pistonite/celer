export type EditorViewState = {
    /// Serial number
    ///
    /// Use to signal that the file system view should be rerendered
    serial: number;

    /// Root path of the opened project.
    ///
    /// Undefined means no editor is opened.
    rootPath: string | undefined;

    /// Currently opened file.
    ///
    /// Undefined means no file is opened.
    openedFile: string | undefined;
}

export const initialEditorViewState: EditorViewState = {
    serial: 0,
    rootPath: undefined,
    openedFile: undefined,
};

