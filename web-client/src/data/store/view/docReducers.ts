//! Reducers for doc view state

import { ReducerDeclWithPayload, withPayload } from "data/store/util";
import { DocViewStore } from "./doc";

/// Set the current doc section
export const setDocSection: ReducerDeclWithPayload<DocViewStore, number> =
    withPayload((state: DocViewStore, value: number) => {
        state.currentSection = value;
    });
