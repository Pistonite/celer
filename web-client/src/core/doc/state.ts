//! Document view, setting, and route document state

import { ExecDoc, PluginMetadata } from "low/celerc";

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

    /// Hide the document when resizing UI
    hideDocWhenResizing: boolean;

    /// Always anchor the notes to the line it is at
    /// instead of letting it shift around
    forceAnchorNotes: boolean;

    /// Display names for types that should be considered splits
    ///
    /// Undefined means to use the document default
    splitTypes: string[] | undefined;

    enabledAppPlugins: Partial<Record<AppPluginType, boolean>>;

    /// Plugins to disable (remove) for each document, identified by document title
    disabledPlugins: Record<string, string[]>;

    /// If user plugins are enabled
    enableUserPlugins: boolean;

    /// Additional user plugin configuration YAML string
    userPluginConfig: string;
} & KeyBindingSettings;

export type AppPluginType = "export-split";

export type KeyBindingSettings = {
    prevLineKey: KeyBinding;
    nextLineKey: KeyBinding;
    prevSplitKey: KeyBinding;
    nextSplitKey: KeyBinding;
};

export type KeyBindingName = keyof KeyBindingSettings;

/// Key binding type
///
/// Each key binding is an array of key names.
/// The keys need to be pressed in order to trigger the action.
export type KeyBinding = string[];

/// Default doc settings
export const initialDocSettingsState: DocSettingsState = {
    theme: "default",
    syncMapToDoc: true,
    hideDocWhenResizing: false,
    forceAnchorNotes: false,
    splitTypes: undefined,
    prevLineKey: ["Alt", "ArrowUp"],
    nextLineKey: ["Alt", "ArrowDown"],
    prevSplitKey: ["PageUp"],
    nextSplitKey: ["PageDown"],
    enabledAppPlugins: {
        "export-split": true,
    },
    disabledPlugins: {},
    enableUserPlugins: false,
    userPluginConfig: '# See the "Learn more" link above for more information',
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
    document: ExecDoc | undefined;
    /// The current document's plugin metadata
    pluginMetadata: PluginMetadata[] | undefined;
};

/// The initial document state
export const initialDocumentState: DocumentState = {
    serial: 0,
    document: undefined,
    pluginMetadata: undefined,
};
