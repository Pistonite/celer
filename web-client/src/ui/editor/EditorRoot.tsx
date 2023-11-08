import "./Editor.css";
import { useCallback } from "react";
import { useSelector } from "react-redux";
import { Body1 } from "@fluentui/react-components";

import { ErrorBoundary } from "ui/shared";
import { settingsSelector, viewSelector } from "core/store";
import { EditorKernel, Kernel, useKernel } from "core/kernel";

import { EditorTree } from "./tree";
import { EditorDropZone } from "./EditorDropZone";

const EditorRootId = "editor-root";

export const EditorRoot: React.FC = () => {
    const kernel = useKernel();
    const { serial, rootPath, openedFile, unsavedFiles, currentFileSupported } =
        useSelector(viewSelector);
    const { showFileTree, editorMode } = useSelector(settingsSelector);

    // Disabling this rule as we are using serial to signal when to update
    // A new listDir reference will cause the tree to update
    /* eslint-disable react-hooks/exhaustive-deps*/
    const listDir = useCallback(
        async (paths: string[]): Promise<string[]> => {
            const editor = kernel.getEditor();
            if (!editor) {
                return [];
            }
            return editor.listDir(paths);
        },
        [serial],
    );
    /* eslint-enable react-hooks/exhaustive-deps*/

    if (rootPath !== undefined && editorMode === "external") {
        return (
        <ErrorBoundary>
                <div id={EditorRootId}>
        <div className="blank-div-message">
                        Web editor is not available because you are using the external editor workflow.
                        Switch to the default layout or a layout without the editor to hide this widget.
                    </div>;
                </div>
        </ErrorBoundary>
        );
    }
    return (
        <ErrorBoundary>
            <div id={EditorRootId}>
                {rootPath !== undefined ? (
                    <>
                        {showFileTree && (
                            <div id="editor-tree-container">
                                <EditorTree
                                    rootName={rootPath}
                                    listDir={listDir}
                                    openedFile={openedFile}
                                    unsavedFiles={unsavedFiles}
                                    onClickFile={(path) => {
                                        tryWithEditorRef(
                                            kernel,
                                            10,
                                            (editor) => {
                                                editor.notifyActivity();
                                                editor.openFile(
                                                    path,
                                                );
                                            },
                                        );
                                    }}
                                />
                            </div>
                        )}
                        <div id="editor-panel">
                            {openedFile !== undefined ? (
                                <>
                                    <div id="editor-file-name">
                                        <Body1>
                                            {openedFile}
                                            {unsavedFiles.includes(
                                                openedFile,
                                            ) && "*"}
                                        </Body1>
                                    </div>
                                    <div id="editor-outer-container">
                                        {currentFileSupported ? (
                                            <div id="editor-container"></div>
                                        ) : (
                                            <Body1>
                                                Cannot open this file. Either
                                                this is not a text file or there
                                                was a problem opening it.
                                            </Body1>
                                        )}
                                    </div>
                                </>
                            ) : (
                                <Body1>
                                    {showFileTree
                                        ? "Click a file to open it"
                                        : "File tree is hidden. Go to Settings > Editor and show the file tree to open files."}
                                </Body1>
                            )}
                        </div>
                    </>
                ) : (
                    <EditorDropZone />
                )}
            </div>
        </ErrorBoundary>
    );
};

const tryWithEditorRef = (
    kernel: Kernel,
    attempts: number,
    callback: (editor: EditorKernel) => void,
) => {
    const doTry = (attempt: number) => {
        if (attempt > attempts) {
            return;
        }
        const editor = kernel.getEditor();
        if (!editor) {
            setTimeout(() => {
                doTry(attempt + 1);
            }, 1000);
            return;
        }
        callback(editor);
    };
    doTry(0);
};
