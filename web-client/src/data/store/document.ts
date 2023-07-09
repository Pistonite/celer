//! The document store slice
//!
//! This slice stores the current ExecutedDocument ready to be viewed
//! by the document viewer and the map.

import { ExecDoc } from "data/model";
import { ReducerDeclWithPayload, configureSlice, withPayload } from "./util";

/// The document store type
type DocumentStore = {
    /// The current document
    document: ExecDoc;
};

/// The initial state
const initialState: DocumentStore = {
    document: {
        loaded: false,
        project: {
            name: "",
            title: "",
            version: "",
            authors: [],
            url: "",
            map: {
                layers: [],
                coordMap: {
                    "2d": ["x", "y"],
                    "3d": ["x", "y", "z"],
                },
                initialCoord: [0, 0, 0],
                initialZoom: 0,
            },
            icons: {},
        },
        map: [],
    },
};

const setDocument: ReducerDeclWithPayload<DocumentStore, ExecDoc> = withPayload(
    (state, value) => {
        state.document = value;
    },
);

/// The document store slice
export const { documentReducer, documentActions, documentSelector } =
    configureSlice({
        name: "document",
        initialState,
        reducers: {
            setDocument,
        },
    });
