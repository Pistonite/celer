// eslint-disable-next-line import/no-internal-modules
import editorWorker from "monaco-editor/esm/vs/editor/editor.worker?worker";
// eslint-disable-next-line import/no-internal-modules
import jsonWorker from "monaco-editor/esm/vs/language/json/json.worker?worker";
// eslint-disable-next-line import/no-internal-modules
import tsWorker from "monaco-editor/esm/vs/language/typescript/ts.worker?worker";

import { AppStore } from "core/store";

import { EditorLog } from "./utils";
import { EditorKernel } from "./EditorKernel";
import { EditorKernelImpl } from "./EditorKernelImpl";

declare global {
    interface Window {
        __theEditorKernel: EditorKernel;
    }
}

export const initEditor = (store: AppStore): EditorKernel => {
    if (window.__theEditorKernel) {
        window.__theEditorKernel.delete();
    }
    EditorLog.info("creating editor");
    window.MonacoEnvironment = {
        getWorker(_, label) {
            if (label === "json") {
                return new jsonWorker();
            }
            if (label === "typescript" || label === "javascript") {
                return new tsWorker();
            }
            return new editorWorker();
        },
    };

    const editor = new EditorKernelImpl(store);
    window.__theEditorKernel = editor;
    return editor;
};
