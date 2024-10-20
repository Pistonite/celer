import type { ReducerDecl } from "low/store";
import { withPayload } from "low/store";
import type { EditorViewState } from "./state";

export const updateFileSys = withPayload<EditorViewState, string | undefined>(
    (state, rootPath) => {
        state.rootPath = rootPath;
        state.serial++;
    },
);

export const incFileSysSerial: ReducerDecl<EditorViewState> = (state) => {
    state.serial++;
};

export const updateOpenedFile = withPayload<
    EditorViewState,
    {
        openedFile: string | undefined;
        currentFileSupported: boolean;
    }
>((state, { openedFile, currentFileSupported }) => {
    state.openedFile = openedFile;
    state.currentFileSupported = currentFileSupported;
});

export const startFileSysLoad: ReducerDecl<EditorViewState> = (state) => {
    state.loadInProgress = true;
    state.lastLoadError = false;
};

export const endFileSysLoad = withPayload<EditorViewState, boolean>(
    (state, success) => {
        state.loadInProgress = false;
        state.lastLoadError = !success;
    },
);

export const startFileSysSave: ReducerDecl<EditorViewState> = (state) => {
    state.saveInProgress = true;
    state.lastSaveError = false;
};

export const endFileSysSave = withPayload<EditorViewState, boolean>(
    (state, success) => {
        state.saveInProgress = false;
        state.lastSaveError = !success;
    },
);

export const setUnsavedFiles = withPayload<EditorViewState, string[]>(
    (state, unsavedFiles) => {
        state.unsavedFiles = unsavedFiles;
    },
);

export const addUnsavedFile = withPayload<EditorViewState, string>(
    (state, unsavedFile) => {
        state.unsavedFiles.push(unsavedFile);
    },
);

export const setCompileInProgress = withPayload<EditorViewState, boolean>(
    (state, compileInProgress) => {
        state.compileInProgress = compileInProgress;
    },
);

export const setCompilerReady = withPayload<EditorViewState, boolean>(
    (state, compilerReady) => {
        state.compilerReady = compilerReady;
    },
);
