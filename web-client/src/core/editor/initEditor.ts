import { FsFileSystem } from "pure/fs";

import { AppStore, settingsSelector } from "core/store";

import { EditorKernelAccess } from "./EditorKernelAccess";
import { EditorKernel } from "./EditorKernel";

declare global {
    interface Window {
        __theEditorKernel: EditorKernel;
    }
}

export const initEditor = async (
    kernel: EditorKernelAccess,
    fs: FsFileSystem,
    store: AppStore,
): Promise<EditorKernel> => {
    deleteEditor();
    const { editorMode } = settingsSelector(store.getState());
    let editor;
    if (editorMode === "web") {
        const { initWebEditor } = await import("./WebEditorKernel");
        editor = initWebEditor(kernel, fs, store);
    } else {
        const { initExternalEditor } = await import("./ExternalEditorKernel");
        editor = initExternalEditor(kernel, fs);
    }

    window.__theEditorKernel = editor;
    return editor;
};

export const deleteEditor = (): void => {
    if (window.__theEditorKernel) {
        window.__theEditorKernel.delete();
    }
};
