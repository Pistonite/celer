//! Reducers for the document state

import { ExecDoc } from "low/celerc";
import { ReducerDeclWithPayload, withPayload } from "low/store";

import { DocumentState } from "./state";

/// Set the document
///
/// Also automatically increment the serial number so that
/// the application re-renders the document and does necessary updates
export const setDocument: ReducerDeclWithPayload<DocumentState, ExecDoc> =
    withPayload((state, value) => {
        state.serial += 1;
        state.document = value;
    });
