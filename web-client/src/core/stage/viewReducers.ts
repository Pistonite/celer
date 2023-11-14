//! Reducers for stage view state

import { ReducerDecl, withPayload } from "low/store";

import { SettingsTab, StageMode, StageViewState } from "./state";

export const setStageMode = withPayload<StageViewState, StageMode>(
    (state, mode) => {
        state.stageMode = mode;
    },
);

export const setSettingsTab = withPayload<StageViewState, SettingsTab>(
    (state, tab) => {
        state.settingsTab = tab;
    },
);

export type AlertPayload = {
    title: string;
    text: string;
    learnMore: string;
    okButton: string;
    cancelButton: string;
};

export const setAlert = withPayload<StageViewState, AlertPayload>(
    (state, { title, text, learnMore: link, okButton, cancelButton }) => {
        state.alertTitle = title;
        state.alertText = text;
        state.alertLearnMoreLink = link;
        state.alertOkButton = okButton;
        state.alertCancelButton = cancelButton;
    },
);

export const clearAlert: ReducerDecl<StageViewState> = (state) => {
    state.alertText = "";
    state.alertOkButton = "";
    state.alertCancelButton = "";
};

export const setIsResizingWindow = withPayload<StageViewState, boolean>(
    (state, isResizingWindow) => {
        state.isResizingWindow = isResizingWindow;
    },
);
