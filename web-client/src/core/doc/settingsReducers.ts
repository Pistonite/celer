//! Reducers for the document settings

import { withPayload } from "low/store";

import {
    DocSettingsState,
    KeyBinding,
    KeyBindingName,
    initialPerDocSettings,
} from "./state";

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

export const setRememberDocPosition = withPayload<DocSettingsState, boolean>(
    (state, value) => {
        state.rememberDocPosition = value;
    },
);

export const setForcePopupNotes = withPayload<DocSettingsState, boolean>(
    (state, value) => {
        state.forcePopupNotes = value;
    },
);

/// Set key bindings
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

// per-doc settings

type PerDocPayload<T> = { docId: string } & T;

/// Set doc initial location
export const setInitialDocLocation = withPayload<
    DocSettingsState,
    PerDocPayload<{
        section: number;
        line: number;
    }>
>((state, { docId, section, line }) => {
    if (!state.perDoc[docId]) {
        state.perDoc[docId] = { ...initialPerDocSettings };
    }
    state.perDoc[docId].initialCurrentSection = section;
    state.perDoc[docId].initialCurrentLine = line;
});

/// Set doc excluded diagnostic sources
export const setExcludedDiagnosticSources = withPayload<
    DocSettingsState,
    PerDocPayload<{
        value: string[];
    }>
>((state, { docId, value }) => {
    if (!state.perDoc[docId]) {
        state.perDoc[docId] = { ...initialPerDocSettings };
    }
    state.perDoc[docId].excludeDiagnosticSources = value;
});

/// Set tags to not split on
export const setExcludedSplitTags = withPayload<
    DocSettingsState,
    PerDocPayload<{
        value: string[];
    }>
>((state, { docId, value }) => {
    if (!state.perDoc[docId]) {
        state.perDoc[docId] = { ...initialPerDocSettings };
    }
    state.perDoc[docId].excludeSplitTags = value;
});
