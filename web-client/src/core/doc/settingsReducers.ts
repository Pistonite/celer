//! Reducers for the document settings

import { DocSettingsState, KeyBinding, KeyBindingName, initialPerDocSettings } from "./state";

import { ReducerDeclWithPayload, withPayload } from "low/store";

/// Set the document viewer theme
export const setDocTheme: ReducerDeclWithPayload<DocSettingsState, string> =
    withPayload((state, theme) => {
        state.theme = theme;
    });

/// Set whether to sync map view to doc
export const setSyncMapToDoc: ReducerDeclWithPayload<DocSettingsState, boolean> =
    withPayload((state: DocSettingsState, syncMapToDoc: boolean) => {
        state.syncMapToDoc = syncMapToDoc;
    });

/// Set whether position should be remembered on close
export const setRememberDocPosition: ReducerDeclWithPayload<
    DocSettingsState,
    boolean
> = withPayload((state: DocSettingsState, value: boolean) => {
    state.rememberDocPosition = value;
});

/// Set whether to force notes to be popups
export const setForcePopupNotes: ReducerDeclWithPayload<DocSettingsState, boolean> =
    withPayload((state: DocSettingsState, value: boolean) => {
        state.forcePopupNotes = value;
    });

/// Set key bindings
export const setDocKeyBinding: ReducerDeclWithPayload<
    DocSettingsState,
    {
        /// name of the key binding to set
        name: KeyBindingName,
        /// new value of the key binding
        value: KeyBinding,
    }
> = withPayload((state: DocSettingsState, {name, value}) => {
    state[name] = value;
});

// per-doc settings

type PerDocPayload<T> = { docId: string } & T;

/// Set doc initial location
export const setInitialDocLocation: ReducerDeclWithPayload<
    DocSettingsState,
    PerDocPayload<{
        section: number;
        line: number;
    }>
> = withPayload((state: DocSettingsState, { docId, section, line }) => {
    if (!state.perDoc[docId]) {
        state.perDoc[docId] = { ...initialPerDocSettings };
    }
    state.perDoc[docId].initialCurrentSection = section;
    state.perDoc[docId].initialCurrentLine = line;
});

/// Set doc excluded diagnostic sources
export const setExcludedDiagnosticSources: ReducerDeclWithPayload<
    DocSettingsState,
    PerDocPayload<{
        value: string[];
    }>
> = withPayload((state: DocSettingsState, { docId, value }) => {
    if (!state.perDoc[docId]) {
        state.perDoc[docId] = { ...initialPerDocSettings };
    }
    state.perDoc[docId].excludeDiagnosticSources = value;
});

/// Set tags to not split on
export const setExcludedSplitTags: ReducerDeclWithPayload<
    DocSettingsState,
    PerDocPayload<{
        value: string[];
    }>
> = withPayload((state: DocSettingsState, { docId, value }) => {
    if (!state.perDoc[docId]) {
        state.perDoc[docId] = { ...initialPerDocSettings };
    }
    state.perDoc[docId].excludeSplitTags = value;
});
