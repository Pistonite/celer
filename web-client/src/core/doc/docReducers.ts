//! Reducers for the document state

import { ExpoContext } from "low/celerc";
import { withPayload } from "low/store";

import { DocumentState } from "./state";

/// Set the document from compiler output
///
/// Also automatically increment the serial number so that
/// the application re-renders the document and does necessary updates
export const setDocument = withPayload<DocumentState, ExpoContext | undefined>(
    (state, value) => {
        if (value) {
            state.serial += 1;
        } else {
            state.serial = 0;
        }
        state.document = value?.execDoc;
        state.pluginMetadata = value?.pluginMetadata;
        state.exportMetadata = value?.exportMetadata;
    },
);
