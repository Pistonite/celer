//! Stage state slice

/// State type for stage view
export type StageViewState = {
    stageMode: StageMode;
    alertTitle: string;
    alertText: string;
    alertOkButton: string;
    alertCancelButton: string;
}

export type StageMode = "view" | "edit";

export const initialStageViewState: StageViewState = {
    stageMode: "view",
    alertTitle: "",
    alertText: "",
    alertOkButton: "",
    alertCancelButton: "",
};

