//! doc setting state

/// State type for doc settings
export type DocSettings = {
    /// Theme name for the doc viewer
    theme: string;
}

/// Default doc settings
export const initialDocSettings: DocSettings = {
    theme: "default",
}

