//! Reducers for the document settings

import { withPayload } from "low/store";
import type { ExportMetadata, PluginMetadata } from "low/celerc";

import type {
    AppPluginType,
    DocSettingsState,
    KeyBinding,
    KeyBindingName,
} from "./state";
import { getExporterId } from "./export";

export const setDocTheme = withPayload<DocSettingsState, string>(
    (state, theme) => {
        state.theme = theme;
    },
);

export const setSyncMapToDoc = withPayload<DocSettingsState, boolean>(
    (state, syncMapToDoc) => {
        state.syncMapToDoc = syncMapToDoc;
    },
);

export const setHideDocWhenResizing = withPayload<DocSettingsState, boolean>(
    (state, value) => {
        state.hideDocWhenResizing = value;
    },
);

export const setForceAnchorNotes = withPayload<DocSettingsState, boolean>(
    (state, value) => {
        state.forceAnchorNotes = value;
    },
);

export const setDocKeyBinding = withPayload<
    DocSettingsState,
    {
        /// name of the key binding to set
        name: KeyBindingName;
        /// new value of the key binding
        value: KeyBinding;
    }
>((state, { name, value }) => {
    state[name] = value;
});

export const setSplitTypes = withPayload<
    DocSettingsState,
    string[] | undefined
>((state, value) => {
    state.splitTypes = value;
});

export const setAppPluginEnabled = withPayload<
    DocSettingsState,
    { type: AppPluginType; enabled: boolean }
>((state, { type, enabled }) => {
    state.enabledAppPlugins[type] = enabled;
});

export const setPluginMetadata = withPayload<
    DocSettingsState,
    { title: string; metadata: PluginMetadata[] }
>((state, { title, metadata }) => {
    state.pluginMetadatas[title] = metadata;
});

export const setUserPluginEnabled = withPayload<DocSettingsState, boolean>(
    (state, value) => {
        state.enableUserPlugins = value;
    },
);

export const setUserPluginConfig = withPayload<DocSettingsState, string>(
    (state, value) => {
        state.userPluginConfig = value;
    },
);

export const setExportConfig = withPayload<
    DocSettingsState,
    {
        metadata: ExportMetadata;
        config: string;
    }
>((state, { metadata, config }) => {
    state.exportConfigs[getExporterId(metadata)] = config;
});

export const setExportConfigToDefault = withPayload<
    DocSettingsState,
    {
        metadata: ExportMetadata;
    }
>((state, { metadata }) => {
    delete state.exportConfigs[getExporterId(metadata)];
});
