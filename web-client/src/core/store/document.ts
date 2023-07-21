//! Document store slice
//!
//! This stores the compiled route document, including map features

import { documentReducers, initialDocumentState } from "core/doc";
import { configureSlice } from "low/store";

/// The document store slice
export const { documentReducer, documentActions, documentSelector } =
    configureSlice({
        name: "document",
        initialState: initialDocumentState,
        reducers: {
            ...documentReducers,
        },
    });
