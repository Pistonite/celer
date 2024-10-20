//! Stage state slice

import type { AlertExtraAction } from "low/utils";

/// State type for stage view
export type StageViewState = {
    stageMode: StageMode;
    alertTitle: string;
    alertText: string;
    alertLearnMoreLink: string;
    alertOkButton: string;
    alertCancelButton: string;
    alertExtraActions: AlertExtraAction[];
    settingsTab: SettingsTab;
    isResizingWindow: boolean;
};

export type StageMode = "view" | "edit";

export type SettingsTab = "map" | "doc" | "editor" | "meta" | "plugin";

export const initialStageViewState: StageViewState = {
    stageMode: window.location.pathname.startsWith("/edit") ? "edit" : "view",
    alertTitle: "",
    alertText: "",
    alertLearnMoreLink: "",
    alertOkButton: "",
    alertCancelButton: "",
    alertExtraActions: [],
    settingsTab: "doc",
    isResizingWindow: false,
};
