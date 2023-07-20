//! Reducers for document settings

import { ReducerDeclWithPayload, withPayload } from "data/store/util";

import { DocKeyBinding, DocSettings, KeyBindingName, initialPerDocSettings } from "./doc";

/// Set the document viewer theme
export const setDocTheme: ReducerDeclWithPayload<DocSettings, string> =
    withPayload((state, theme) => {
        state.theme = theme;
    });

/// Set whether to sync map view to doc
export const setSyncMapToDoc: ReducerDeclWithPayload<DocSettings, boolean> =
    withPayload((state: DocSettings, syncMapToDoc: boolean) => {
        state.syncMapToDoc = syncMapToDoc;
    });

/// Set whether position should be remembered on close
export const setRememberDocPosition: ReducerDeclWithPayload<
    DocSettings,
    boolean
> = withPayload((state: DocSettings, value: boolean) => {
    state.rememberDocPosition = value;
});

/// Set whether to force notes to be popups
export const setForcePopupNotes: ReducerDeclWithPayload<DocSettings, boolean> =
    withPayload((state: DocSettings, value: boolean) => {
        state.forcePopupNotes = value;
    });

/// Set key bindings
export const setDocKeyBinding: ReducerDeclWithPayload<
    DocSettings,
    {
        /// name of the key binding to set
        name: KeyBindingName,
        /// new value of the key binding
        value: DocKeyBinding,
    }
> = withPayload((state: DocSettings, {name, value}) => {
    state[name] = value;
});

// per-doc settings

type PerDocPayload<T> = { docId: string } & T;

/// Set doc initial location
export const setInitialDocLocation: ReducerDeclWithPayload<
    DocSettings,
    PerDocPayload<{
        section: number;
        line: number;
    }>
> = withPayload((state: DocSettings, { docId, section, line }) => {
    if (!state.perDoc[docId]) {
        state.perDoc[docId] = { ...initialPerDocSettings };
    }
    state.perDoc[docId].initialCurrentSection = section;
    state.perDoc[docId].initialCurrentLine = line;
});

/// Set doc excluded diagnostic sources
export const setExcludedDiagnosticSources: ReducerDeclWithPayload<
    DocSettings,
    PerDocPayload<{
        value: string[];
    }>
> = withPayload((state: DocSettings, { docId, value }) => {
    if (!state.perDoc[docId]) {
        state.perDoc[docId] = { ...initialPerDocSettings };
    }
    state.perDoc[docId].excludeDiagnosticSources = value;
});

/// Set tags to not split on
export const setExcludedSplitTags: ReducerDeclWithPayload<
    DocSettings,
    PerDocPayload<{
        value: string[];
    }>
> = withPayload((state: DocSettings, { docId, value }) => {
    if (!state.perDoc[docId]) {
        state.perDoc[docId] = { ...initialPerDocSettings };
    }
    state.perDoc[docId].excludeSplitTags = value;
});
