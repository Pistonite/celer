
import { useSelector, useStore } from "react-redux";
import "./Editor.css";
import { EditorDropZone } from "./EditorDropZone";
import { EditorTree } from "./tree";
import { EditorState, initEditor } from "./EditorState";
import { useCallback, useEffect, useRef } from "react";
import { AppStore, viewSelector } from "core/store";
import { EditorLog } from "./utils";

// const EditorContainerId = "editor-container";
const memo: {[key: string]: string[]} = {};

const testListDir = (paths: string[]): Promise<string[]> => {
    const path= paths.join("/");
    if (memo[path] !== undefined) {
        return Promise.resolve(memo[path]);
    }
    const depth = path.split("/").length;
    if (depth > 5) {
        return Promise.resolve(["abc.txt"]);
    } 
    const exts = [".js", ".py", ".json", ".yaml", ".md"];
    const entries: string[] = [];
    const randomName = () => {
        const length = Math.floor(Math.random() * 10) + 3;
        let name = "";
        for(let i = 0; i < length; i++) {
            name += String.fromCharCode(Math.floor(Math.random() * 26) + "a".charCodeAt(0));
        }
        return name;
    };
    while(Math.random() < 0.8) {
        if(Math.random() < 0.5) {
            const ext = exts[Math.floor(Math.random() * exts.length)];
            entries.push(randomName() + ext);
        } else {
            entries.push(randomName() + "/");
        }
    }
    memo[path] = entries;
    return new Promise((resolve) => {
        setTimeout(() => {
            resolve(entries);
        }, 1000);
    });

}
export const EditorRoot: React.FC = () => {
    const store = useStore();
    const { serial, rootPath } = useSelector(viewSelector);
    const editorState = useRef<EditorState | null>(null);
    useEffect(() => {
        editorState.current = initEditor(store as AppStore);
    }, [store]);

    // Disabling this rule as we are using serial to signal when to update
    // A new listDir reference will cause the tree to update
    /* eslint-disable react-hooks/exhaustive-deps*/
    const listDir = useCallback(async (paths: string[]): Promise<string[]> => {
        if (editorState.current === null) {
            return [];
        }
        return editorState.current.listDir(paths);
    }, [serial]);
    return (
        <div id="editor-root">
            {
                rootPath !== undefined ? (
                    <>
                        <div id="editor-tree-container">
                            <EditorTree 
                                rootName={rootPath}
                                listDir={listDir}
                                onClickFile={(path) => {
                                    console.log(path);
                                }}
                            />
                        </div>
                        <div>Click a file to open it</div>
                            </>
                ) : (
            <EditorDropZone onFileSysCreate={(fileSys) => {
                const setFs = (attempt: number) => {
                    if (attempt > 10) {
                        EditorLog.error("Editor not initialized after max attempts");
                        return;
                    }
                    if (!editorState.current) {
                        EditorLog.warn("Editor not initialized. Will try again.")
                        setTimeout(() => {
                            setFs(attempt + 1);
                        }, 1000);
                        return;
                    }
                    editorState.current.reset(fileSys);
                };
                setFs(0);
            }}/>
                )
            }
        </div>
    );
};
