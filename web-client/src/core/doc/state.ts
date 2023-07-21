//! Document view, setting, and route document state

import { ExecDoc } from "low/compiler";

/// View state for the document
export type DocViewState = {
    /// Current section the user is on
    currentSection: number;
    /// Current line the user is on in the section
    currentLine: number;
    /// If the user is currently editing a key binding
    editingKeyBinding: KeyBindingName | undefined;
};

export const initialDocViewState: DocViewState = {
    currentSection: 0,
    currentLine: 0,
    editingKeyBinding: undefined,
};

/// State type for doc settings
export type DocSettingsState = {
    /// Theme name for the doc viewer
    theme: string;
    /// Set map view to fit doc when scrolled
    syncMapToDoc: boolean;
    /// Remember doc position on close
    rememberDocPosition: boolean;
    /// Always display notes as popup
    forcePopupNotes: boolean;
    /// Per-doc settings
    ///
    /// The key is the name of the document
    perDoc: Record<string, PerDocSettings>;
} & KeyBindingSettings;

export type KeyBindingSettings = {
    prevLineKey: KeyBinding;
    nextLineKey: KeyBinding;
    prevSplitKey: KeyBinding;
    nextSplitKey: KeyBinding;
}

export type KeyBindingName = keyof KeyBindingSettings;

/// Key binding type
///
/// Each key binding is an array of key names.
/// The keys need to be pressed in order to trigger the action.
export type KeyBinding = string[];

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
export const initialDocSettingsState: DocSettingsState = {
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

export const initialPerDocSettings: PerDocSettings = {
    initialCurrentSection: 0,
    initialCurrentLine: 0,
    excludeDiagnosticSources: [],
    excludeSplitTags: [],
};

/// The document state type
export type DocumentState = {
    /// Serial number of the document
    ///
    /// This is updated automatically with `setDocument`.
    /// The document will only be considered "changed" if the serial number
    /// changes, which causes rerenders, etc.
    serial: number;
    /// The current document
    document: ExecDoc;
};

/// The initial document state for the 
export const initialDocumentState: DocumentState = {
    serial: 0,
    document: {
        loaded: false,
        project: {
            name: "",
            title: "",
            version: "",
            authors: [],
            url: "",
            map: {
                layers: [],
                coordMap: {
                    "2d": ["x", "y"],
                    "3d": ["x", "y", "z"],
                },
                initialCoord: [0, 0, 0],
                initialZoom: 0,
            },
            icons: {},
            tags: {},
        },
        route: [],
        map: [],
    },
};

