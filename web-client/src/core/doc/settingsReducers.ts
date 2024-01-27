//! Reducers for the document settings

import { withPayload } from "low/store";

import { AppPluginType, DocSettingsState, KeyBinding, KeyBindingName } from "./state";

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
