import { FsFileSystem } from "pure/fs";

import { AppStore, settingsSelector } from "core/store";

import { EditorKernelAccess } from "./EditorKernelAccess";
import { EditorKernel } from "./EditorKernel";

declare global {
    interface Window {
        __theEditor: EditorKernel;
    }
}

export async function initEditor(
    kernel: EditorKernelAccess,
    fs: FsFileSystem,
    store: AppStore,
): Promise<EditorKernel> {
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

    window.__theEditor = editor;
    return editor;
}

export function deleteEditor(): void {
    if (window.__theEditor) {
        window.__theEditor.delete();
    }
}
