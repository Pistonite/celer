
import { AppStore, settingsSelector } from "core/store";

import { EditorLog, KernelAccess } from "./utils";
import { EditorKernel } from "./EditorKernel";

declare global {
    interface Window {
        __theEditorKernel: EditorKernel;
    }
}

export const initEditor = async (kernel: KernelAccess, store: AppStore): Promise<EditorKernel> => {
    deleteEditor();
    const { editorMode } = settingsSelector(store.getState());
    let editor;
    if (editorMode === "web") {
        const { initWebEditor } = await import("./WebEditorKernel");
        editor = initWebEditor(store);
    } else {
            // TODO #122: bind FileSys directly to compiler
    }

    window.__theEditorKernel = editor;
    return editor;
};

export const deleteEditor = (): void => {
    if (window.__theEditorKernel) {
        window.__theEditorKernel.delete();
    }
}
