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

export const setRoutePluginEnabled = withPayload<
    DocSettingsState,
    { docTitle: string; plugin: string; enabled: boolean }
>((state, { docTitle, plugin, enabled }) => {
    if (enabled) {
        if (state.disabledPlugins[docTitle]) {
            state.disabledPlugins[docTitle] = state.disabledPlugins[docTitle].filter(x => x !== plugin);
        }
        return;
    }
    if (state.disabledPlugins[docTitle]) {
        state.disabledPlugins[docTitle].push(plugin);
        return;
    }
    state.disabledPlugins[docTitle] = [plugin];

});

export const setUserPluginEnabled = withPayload<
    DocSettingsState,
    boolean
>((state, value) => {
    state.enableUserPlugins = value;
});

export const setUserPluginConfig = withPayload<
    DocSettingsState,
    string
>((state, value) => {
    state.userPluginConfig = value;
});
