//! Helper selectors for the doc setting state

import { DocSettings, PerDocSettings, initialPerDocSettings } from "./doc";

/// Get per-doc settings by doc id
export const getPerDocSettings = (
    state: DocSettings,
    docId: string,
): PerDocSettings => {
    if (!state.perDoc[docId]) {
        return initialPerDocSettings;
    }
    return state.perDoc[docId];
};
