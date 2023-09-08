//! Editor logic that wraps monaco editor
import * as _monaco from 'monaco-editor'
// eslint-disable-next-line import/no-internal-modules
import editorWorker from 'monaco-editor/esm/vs/editor/editor.worker?worker'
// eslint-disable-next-line import/no-internal-modules
import jsonWorker from 'monaco-editor/esm/vs/language/json/json.worker?worker'
// eslint-disable-next-line import/no-internal-modules
import tsWorker from 'monaco-editor/esm/vs/language/typescript/ts.worker?worker'

import { AppStore, viewActions } from 'core/store';
import { FileSys, FsResultCode, fsRootPath } from 'low/fs';

import { EditorLog } from './utils';

declare global {
    interface Window {
        __theEditorState: EditorState;
    }
}

export const initEditor = (store: AppStore): EditorState => {
    if (window.__theEditorState) {
        window.__theEditorState.delete();
    }
    EditorLog.info("creating editor");
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

    const editor = new EditorState(store);
    window.__theEditorState = editor;
    return editor;
}

export class EditorState {
    private store: AppStore;
    private fs: FileSys | undefined;

    constructor(store: AppStore) {
        this.store = store;
    }

    /// Reset the editor with a new file system. Unsaved changes will be lost
    public reset(fs?: FileSys) {
        EditorLog.info("resetting editor");
        this.fs = fs;
        this.store.dispatch(viewActions.setOpenedFile(undefined));
        if (fs) {
            this.store.dispatch(viewActions.setRootPath(fs.getRootName()));
        } else {
            this.store.dispatch(viewActions.setRootPath(undefined));
        }
    }

    public delete() {
        EditorLog.info("deleting editor");
        this.reset();
    }

    /// Request directory listing
    ///
    /// See EditorTree for input/output format
    /// On failure this returns empty array. This function will not throw
    public async listDir(path: string[]): Promise<string[]> {
        if (!this.fs) {
            return [];
        }
        let fsPath = fsRootPath;
        for (let i = 0; i < path.length; i++) {
            fsPath = fsPath.resolve(path[i]);
        }
        const result = await this.fs.listDir(fsPath);
        if (result.code !== FsResultCode.Ok) {
            EditorLog.error(`listDir failed with code ${result.code}`);
            return [];
        }
        return result.value;
    }
}
