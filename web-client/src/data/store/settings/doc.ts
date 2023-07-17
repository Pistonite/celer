//! doc setting state

/// State type for doc settings
export type DocSettings = {
    // General settings

    /// Theme name for the doc viewer
    theme: string;
    /// Set map view to fit doc when scrolled
    syncMapToDoc: boolean;
    /// Remember doc position on close
    rememberDocPosition: boolean;
    /// Always display notes as popup
    forcePopupNotes: boolean;
    /// Key bindings
    prevLineKey: DocKeyBinding;
    nextLineKey: DocKeyBinding;
    prevSplitKey: DocKeyBinding;
    nextSplitKey: DocKeyBinding;
    /// Per-doc settings
    ///
    /// The key is the name of the document
    perDoc: Record<string, PerDocSettings>;
};

/// Key binding type
///
/// Each key binding is an array of key names.
/// The keys need to be pressed in order to trigger the action.
export type DocKeyBinding = string[];

/// Per-doc settings
export type PerDocSettings = {
    /// The initial current line position
    ///
    /// Document will be scrolled to this line on load
    initialCurrentSection: number;
    initialCurrentLine: number;
    /// Hide diagnostics from sources
    excludeDiagnosticSources: string[];
    /// Tags to not split on
    excludeSplitTags: string[];
};

/// Default doc settings
export const initialDocSettings: DocSettings = {
    theme: "default",
    syncMapToDoc: true,
    rememberDocPosition: true,
    forcePopupNotes: false,
    prevLineKey: ["ArrowUp"],
    nextLineKey: ["ArrowDown"],
    prevSplitKey: ["PageUp"],
    nextSplitKey: ["PageDown"],
    perDoc: {},
};

export const initialPerDocSettings = {
    initialCurrentSection: 0,
    initialCurrentLine: 0,
    excludeDiagnosticSources: [],
    excludeSplitTags: [],
};
