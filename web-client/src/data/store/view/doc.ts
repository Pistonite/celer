//! View part of the view store
//!
//! This stores the current view state of the document viewer

export type DocViewStore = {
    /// Current section the user is on
    currentSection: number;
    /// Current line the user is on in the section
    currentLine: number;
};

export const initialDocViewStore: DocViewStore = {
    currentSection: 0,
    currentLine: 0,
};
