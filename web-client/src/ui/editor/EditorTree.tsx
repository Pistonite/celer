import { useCallback, useEffect, useState } from "react";
import { useSelector } from "react-redux";

import { useKernel } from "core/kernel";
import { settingsSelector, viewSelector } from "core/store";

import { TreeItem } from "./TreeItem";
import { useEditorStyles } from "./styles";

export const EditorTree: React.FC = () => {
    const kernel = useKernel();
    const { serial, rootPath, openedFile, unsavedFiles } =
        useSelector(viewSelector);
    const { showFileTree } = useSelector(settingsSelector);
    const styles = useEditorStyles();

    // We are using serial to signal when to update
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

    const [expandedPaths, setExpandedPaths] = useState<string[]>([""]);

    if (!showFileTree && openedFile) {
        return null;
    }
    return (
        <div className={styles.editorTreeContainer}>
            <TreeDirNode
                name={rootPath || ""}
                path={[]}
                openedFile={openedFile}
                unsavedFiles={unsavedFiles}
                listDir={listDir}
                onClickFile={async (path) => {
                    const editor = kernel.getEditor();
                    if (!editor) {
                        return;
                    }
                    editor.notifyActivity();
                    editor.openFile(path);
                }}
                getIsExpanded={(path) => expandedPaths.includes(path.join("/"))}
                setIsExpanded={(path, isExpanded) => {
                    const pathStr = path.join("/");

                    if (isExpanded) {
                        setExpandedPaths((x) => [...x, pathStr]);
                    } else {
                        setExpandedPaths((x) => x.filter((p) => pathStr !== p));
                    }
                }}
                level={0}
            />
        </div>
    );
};

type TreeDirNodeProps = {
    name: string;
    path: string[];
    listDir: (path: string[]) => Promise<string[]>;
    onClickFile: (path: string[]) => void;
    level: number;
    getIsExpanded: (path: string[]) => boolean;
    setIsExpanded: (path: string[], isExpanded: boolean) => void;
    openedFile: string | undefined;
    unsavedFiles: string[];
};

const TreeDirNode: React.FC<TreeDirNodeProps> = ({
    name,
    path,
    listDir,
    onClickFile,
    level,
    getIsExpanded,
    setIsExpanded,
    openedFile,
    unsavedFiles,
}) => {
    const [entries, setEntries] = useState<string[] | undefined>(undefined);

    const isExpanded = getIsExpanded(path);

    // using path.join since path is an array
    /* eslint-disable react-hooks/exhaustive-deps*/
    useEffect(() => {
        if (!isExpanded) {
            return;
        }
        const loadEntries = async () => {
            const entries = await listDir(path);
            entries.sort(compareEntry);
            setEntries(entries);
        };
        loadEntries();
    }, [path.join("/"), isExpanded, listDir]);
    /* eslint-enable react-hooks/exhaustive-deps*/

    const isLoading = isExpanded && entries === undefined;

    return (
        <>
            <TreeItem
                file={name}
                isDirectory={true}
                isSelected={false}
                isExpanded={isExpanded}
                onClickFile={() => {
                    setIsExpanded(path, !isExpanded);
                }}
                level={level}
                isLoading={isLoading}
            />
            {isExpanded &&
                entries !== undefined &&
                entries.map((entry, i) => {
                    if (entry.endsWith("/")) {
                        const name = entry.slice(0, -1);
                        return (
                            <TreeDirNode
                                key={i}
                                name={name}
                                path={[...path, name]}
                                listDir={listDir}
                                onClickFile={onClickFile}
                                getIsExpanded={getIsExpanded}
                                setIsExpanded={setIsExpanded}
                                level={level + 1}
                                openedFile={openedFile}
                                unsavedFiles={unsavedFiles}
                            />
                        );
                    } else {
                        const pathStr =
                            path.length === 0
                                ? entry
                                : `${path.join("/")}/${entry}`;
                        return (
                            <TreeItem
                                key={i}
                                file={entry}
                                isDirectory={false}
                                isSelected={pathStr === openedFile}
                                onClickFile={() => {
                                    onClickFile([...path, entry]);
                                }}
                                level={level + 1}
                                isLoading={false}
                                isDirty={unsavedFiles.includes(pathStr)}
                            />
                        );
                    }
                })}
        </>
    );
};

const compareEntry = (a: string, b: string): number => {
    const isADir = a.endsWith("/");
    const isBDir = b.endsWith("/");
    if (isADir && !isBDir) {
        return -1;
    }
    if (!isADir && isBDir) {
        return 1;
    }
    return a.localeCompare(b);
};
