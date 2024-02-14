//! core/editor
//! Editor related state

import { DOMId } from "low/utils";

export * from "./state";
export * as editorViewReducers from "./viewReducers";
export * as editorSettingsReducers from "./settingsReducers";

export const EditorContainerDOM = new DOMId("editor-container");
