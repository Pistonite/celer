import { ReducerDecl, ReducerDeclWithPayload, withPayload } from "low/store";
import { EditorViewState } from "./state";

export const updateFileSys: ReducerDeclWithPayload<
    EditorViewState,
    {
        rootPath: string | undefined;
        supportsSave: boolean;
    }
> = withPayload((state: EditorViewState, { rootPath, supportsSave }) => {
    state.rootPath = rootPath;
    state.supportsSave = supportsSave;
    state.serial++;
});

export const incFileSysSerial: ReducerDecl<EditorViewState> = (
    state: EditorViewState,
) => {
    state.serial++;
};

export const updateOpenedFile: ReducerDeclWithPayload<
    EditorViewState,
    {
        openedFile: string | undefined;
        currentFileSupported: boolean;
    }
> = withPayload(
    (state: EditorViewState, { openedFile, currentFileSupported }) => {
        state.openedFile = openedFile;
        state.currentFileSupported = currentFileSupported;
    },
);

export const startFileSysLoad: ReducerDecl<EditorViewState> = (state) => {
    state.loadInProgress = true;
    state.lastLoadError = false;
};

export const endFileSysLoad: ReducerDeclWithPayload<EditorViewState, boolean> =
    withPayload((state: EditorViewState, success: boolean) => {
        state.loadInProgress = false;
        state.lastLoadError = !success;
    });

export const startFileSysSave: ReducerDecl<EditorViewState> = (state) => {
    state.saveInProgress = true;
    state.lastSaveError = false;
};

export const endFileSysSave: ReducerDeclWithPayload<EditorViewState, boolean> =
    withPayload((state: EditorViewState, success: boolean) => {
        state.saveInProgress = false;
        state.lastSaveError = !success;
    });

export const setAutoLoadActive: ReducerDeclWithPayload<
    EditorViewState,
    boolean
> = withPayload((state: EditorViewState, active: boolean) => {
    state.autoLoadActive = active;
});

export const setUnsavedFiles: ReducerDeclWithPayload<
    EditorViewState,
    string[]
> = withPayload((state: EditorViewState, unsavedFiles: string[]) => {
    state.unsavedFiles = unsavedFiles;
});

export const addUnsavedFile: ReducerDeclWithPayload<
    EditorViewState,
    string
> = withPayload((state: EditorViewState, unsavedFile: string) => {
    state.unsavedFiles.push(unsavedFile);
});

export const setShowFileTree: ReducerDeclWithPayload<EditorViewState, boolean> =
    withPayload((state: EditorViewState, showFileTree: boolean) => {
        state.showFileTree = showFileTree;
    });

export const setCompileInProgress: ReducerDeclWithPayload<EditorViewState, boolean> =
    withPayload((state: EditorViewState, compileInProgress: boolean) => {
        state.compileInProgress = compileInProgress;
    });
