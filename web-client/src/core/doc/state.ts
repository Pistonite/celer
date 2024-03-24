//! Document view, setting, and route document state
import { z } from "zod";

import { ExecDoc, ExportMetadata, PluginMetadata } from "low/celerc";

// Tsify doesn't generate schema for zod, so we need to define it here
export const PluginMetadataSchema = z.object({
    displayId: z.string(),
    isFromUser: z.boolean(),
    isEnabled: z.boolean(),
});

/// View state for the document
export type DocViewState = {
    /// Current section the user is on
    currentSection: number;
    /// Current line the user is on in the section
    currentLine: number;
    /// If the user is currently editing a key binding
    editingKeyBinding: KeyBindingName | undefined;
    /// If recompiles should be suppressed
    /// Used when the setting is updated from the result of compilation
    suppressRecompile: boolean;
};

export const initialDocViewState: DocViewState = {
    currentSection: 0,
    currentLine: 0,
    editingKeyBinding: undefined,
    suppressRecompile: false,
};

export const AppPluginTypeSchema = z.enum(["export-split"]);
export type AppPluginType = z.infer<typeof AppPluginTypeSchema>;

export const DocSettingsStateInternalSchema = z.object({
    /// Theme name for the doc viewer
    theme: z.string(),
    /// Set map view to fit doc when scrolled
    syncMapToDoc: z.boolean(),

    /// Hide the document when resizing UI
    hideDocWhenResizing: z.boolean(),

    /// Always anchor the notes to the line it is at
    /// instead of letting it shift around
    forceAnchorNotes: z.boolean(),

    /// Display names for types that should be considered splits
    ///
    /// Undefined means to use the document default
    splitTypes: z.string().array().optional(),

    enabledAppPlugins: z.record(AppPluginTypeSchema, z.boolean()),

    /// Plugin metadata for displaying plugin list and the
    /// enabled state for each document, identified by document title
    pluginMetadatas: z.record(z.string(), z.array(PluginMetadataSchema)),

    /// If user plugins are enabled
    enableUserPlugins: z.boolean(),

    /// Additional user plugin configuration YAML string
    userPluginConfig: z.string(),

    /// Saved export configurations
    exportConfigs: z.record(z.string(), z.string()),
});

/// Key binding type
///
/// Each key binding is an array of key names.
/// The keys need to be pressed in order to trigger the action.
export const KeyBindingSchema = z.array(z.string());
export type KeyBinding = z.infer<typeof KeyBindingSchema>;

export const KeyBindingSettingsSchema = z.object({
    prevLineKey: KeyBindingSchema,
    nextLineKey: KeyBindingSchema,
    prevSplitKey: KeyBindingSchema,
    nextSplitKey: KeyBindingSchema,
});

export type KeyBindingSettings = z.infer<typeof KeyBindingSettingsSchema>;
export type KeyBindingName = keyof KeyBindingSettings;

/// State type for doc settings
export const DocSettingsStateSchema = DocSettingsStateInternalSchema.merge(
    KeyBindingSettingsSchema,
);
export type DocSettingsState = z.infer<typeof DocSettingsStateSchema>;

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
    pluginMetadatas: {},
    enableUserPlugins: false,
    userPluginConfig: '# See the "Learn more" link above for more information',
    exportConfigs: {},
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
    /// The current document's export options
    exportMetadata: ExportMetadata[] | undefined;
};

/// The initial document state
export const initialDocumentState: DocumentState = {
    serial: 0,
    document: undefined,
    pluginMetadata: undefined,
    exportMetadata: undefined,
};
