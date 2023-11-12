//! Stage state slice

/// State type for stage view
export type StageViewState = {
    stageMode: StageMode;
    alertTitle: string;
    alertText: string;
    alertLearnMoreLink: string;
    alertOkButton: string;
    alertCancelButton: string;
    settingsTab: SettingsTab;
};

export type StageMode = "view" | "edit";

export type SettingsTab = "map" | "doc" | "editor" | "meta";

export const initialStageViewState: StageViewState = {
    stageMode: window.location.pathname.startsWith("/edit") ? "edit" : "view",
    alertTitle: "",
    alertText: "",
    alertLearnMoreLink: "",
    alertOkButton: "",
    alertCancelButton: "",
    settingsTab: "doc",
};
