import { AppStore, settingsSelector } from "core/store";
import { FileSys } from "low/fs";

import { KernelAccess } from "./utils";
import { EditorKernel } from "./EditorKernel";

declare global {
    interface Window {
        __theEditorKernel: EditorKernel;
    }
}

export const initEditor = async (
    kernel: KernelAccess,
    fileSys: FileSys,
    store: AppStore,
): Promise<EditorKernel> => {
    deleteEditor();
    const { editorMode } = settingsSelector(store.getState());
    let editor;
    if (editorMode === "web") {
        const { initWebEditor } = await import("./WebEditorKernel");
        editor = initWebEditor(kernel, fileSys, store);
    } else {
        const { initExternalEditor } = await import("./ExternalEditorKernel");
        editor = initExternalEditor(kernel, fileSys);
    }

    window.__theEditorKernel = editor;
    return editor;
};

export const deleteEditor = (): void => {
    if (window.__theEditorKernel) {
        window.__theEditorKernel.delete();
    }
};
