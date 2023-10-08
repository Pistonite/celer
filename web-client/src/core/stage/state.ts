//! Stage state slice

/// State type for stage view
export type StageViewState = {
    stageMode: StageMode;
    alertTitle: string;
    alertText: string;
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
    alertOkButton: "",
    alertCancelButton: "",
    settingsTab: "doc",
};
