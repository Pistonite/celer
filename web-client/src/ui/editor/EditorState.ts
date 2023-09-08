//! Editor logic that wraps monaco editor
import * as _monaco from 'monaco-editor'
// eslint-disable-next-line import/no-internal-modules
import editorWorker from 'monaco-editor/esm/vs/editor/editor.worker?worker'
// eslint-disable-next-line import/no-internal-modules
import jsonWorker from 'monaco-editor/esm/vs/language/json/json.worker?worker'
// eslint-disable-next-line import/no-internal-modules
import tsWorker from 'monaco-editor/esm/vs/language/typescript/ts.worker?worker'

import { AppStore } from 'core/store';

declare global {
    interface Window {
        __theEditorState: EditorState;
    }
}

export const initEditor = (_store: AppStore): EditorState => {
    window.MonacoEnvironment = {
        getWorker(_, label) {
            if (label === 'json') {
                return new jsonWorker()
            }
            if (label === 'typescript' || label === 'javascript') {
                return new tsWorker()
            }
            return new editorWorker()
        }
    };

    return new EditorState();
}

export class EditorState {}
