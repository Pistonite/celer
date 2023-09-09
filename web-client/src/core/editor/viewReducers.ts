import { ReducerDecl, ReducerDeclWithPayload, withPayload } from "low/store";
import { EditorViewState } from "./state";

export const updateFileSys: ReducerDeclWithPayload<
EditorViewState, 
{
    rootPath: string|undefined,
    supportsSave: boolean,
}
> =
withPayload((state: EditorViewState, {rootPath, supportsSave}) => {
    state.rootPath = rootPath;
    state.supportsSave = supportsSave;
    state.serial++;
});

export const incFileSysSerial: ReducerDecl<EditorViewState> =
(state: EditorViewState) => {
    state.serial++;
};

export const updateOpenedFile: ReducerDeclWithPayload<
EditorViewState, 
{
    openedFile: string|undefined,
    currentFileSupported: boolean,
}
> =
withPayload((state: EditorViewState, {openedFile, currentFileSupported}) => {
    state.openedFile = openedFile;
    state.currentFileSupported = currentFileSupported;
});
