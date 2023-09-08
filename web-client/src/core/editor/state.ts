export type EditorViewState = {
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
    rootPath: undefined,
    openedFile: undefined,
};

