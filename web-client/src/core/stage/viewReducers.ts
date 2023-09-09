//! Reducers for stage view state

import { ReducerDecl, ReducerDeclWithPayload, withPayload } from "low/store";

import { StageMode, StageViewState } from "./state";

export const setStageMode: ReducerDeclWithPayload<StageViewState, StageMode> = withPayload((state: StageViewState, mode: "view" | "edit") => {
    state.stageMode = mode;
});

export const setAlert: ReducerDeclWithPayload<
StageViewState,
{
    title: string;
    text: string;
    okButton: string;
    cancelButton: string;
}> = withPayload((state: StageViewState, {title, text, okButton, cancelButton}) => {
    state.alertTitle = title;
    state.alertText = text;
    state.alertOkButton = okButton;
    state.alertCancelButton = cancelButton;
});

export const clearAlert: ReducerDecl<StageViewState> = (state: StageViewState) => {
    state.alertText = "";
    state.alertOkButton = "";
    state.alertCancelButton = "";
}

