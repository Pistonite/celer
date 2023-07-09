//! View part of the view store
//!
//! This stores the current view state of the document viewer

export type DocViewStore = {
    /// Current section the user is on
    currentSection: number;
};

export const initialDocViewStore: DocViewStore = {
    currentSection: 0,
};
