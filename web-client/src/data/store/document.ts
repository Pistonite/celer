//! The document store slice
//!
//! This slice stores the current ExecutedDocument ready to be viewed
//! by the document viewer and the map.

import { ExecutedDocument } from "data/model";
import { ReducerDeclWithPayload, configureSlice, withPayload } from "./util";

/// The document store type
type DocumentStore = {
    /// The current document
    document: ExecutedDocument;
}

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
                    _2d: ["x", "y"],
                    _3d: ["x", "y", "z"]
                },
                zoomBounds: [0, 0],
                attribution: {
                    link: "",
                    text: "",
                    copyRight: false
                },
            },
            icons: {},
        },
        map: {
            lines: undefined,
            icons: [],
        }
    }
}

const setDocument: ReducerDeclWithPayload<
    DocumentStore, ExecutedDocument
> = withPayload((state, value) => {
    state.document = value;
});

/// The document store slice
export const {
    documentReducer,
    documentActions,
    documentSelector
} = configureSlice({
    name: "document",
    initialState,
    reducers: {
        setDocument
    }
});