//! Stage state slice

/// State type for stage view
export type StageViewState = {
    alertTitle: string;
    alertText: string;
    alertOkButton: string;
    alertCancelButton: string;
}

export const initialStageViewState: StageViewState = {
    alertTitle: "",
    alertText: "",
    alertOkButton: "",
    alertCancelButton: "",
};

