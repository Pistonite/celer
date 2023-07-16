//! The document store slice
//!
//! This slice stores the current ExecutedDocument ready to be viewed
//! by the document viewer and the map.

import { ExecDoc } from "data/model";
import { ReducerDeclWithPayload, configureSlice, withPayload } from "./util";

/// The document store type
export type DocumentStore = {
    /// Serial number of the document
    ///
    /// This is updated automatically with `setDocument`.
    /// The document will only be considered "changed" if the serial number
    /// changes, which causes rerenders, etc.
    serial: number;
    /// The current document
    document: ExecDoc;
};

/// The initial state
const initialState: DocumentStore = {
    serial: 0,
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
            tags: {},
        },
        route: [],
        map: [],
    },
};

const setDocument: ReducerDeclWithPayload<DocumentStore, ExecDoc> = withPayload(
    (state, value) => {
        state.serial += 1;
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
