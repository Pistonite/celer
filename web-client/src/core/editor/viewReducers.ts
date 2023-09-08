import { ReducerDeclWithPayload, withPayload } from "low/store";
import { EditorViewState } from "./state";

export const setRootPath: ReducerDeclWithPayload<EditorViewState, string|undefined> =
withPayload((state: EditorViewState, rootPath: string | undefined) => {
    state.rootPath = rootPath;
});

export const setOpenedFile: ReducerDeclWithPayload<EditorViewState, string|undefined> =
withPayload((state: EditorViewState, openedFile: string | undefined) => {
    state.openedFile = openedFile;
});
