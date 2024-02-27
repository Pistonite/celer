import { useCallback, useEffect, useMemo, useState } from "react";
import { useSelector } from "react-redux";

import { fsComponents, fsJoin, fsRoot } from "pure/fs";

import { useKernel } from "core/kernel";
import { settingsSelector, viewSelector } from "core/store";

import { TreeItem } from "./TreeItem";
import { useEditorStyles } from "./styles";

/// Tree view of the project opened in the editor.
///
/// This component is connected to the store directly
export const EditorTree: React.FC = () => {
    const kernel = useKernel();
    const { serial, rootPath, openedFile, unsavedFiles } =
        useSelector(viewSelector);
    const { showFileTree } = useSelector(settingsSelector);
    const styles = useEditorStyles();

    // We are using serial to signal when to update
    // A new listDir reference will cause the tree to reload the entries
    /* eslint-disable react-hooks/exhaustive-deps*/
    const listDir = useCallback(
        (path: string) => {
            return kernel.getEditor()?.listDir(path) || Promise.resolve([]);
        },
        [serial],
    );
    /* eslint-enable react-hooks/exhaustive-deps*/

    const dirtyPaths = useMemo(() => {
        const set = new Set<string>();
        for (let i = 0; i < unsavedFiles.length; i++) {
            const path = unsavedFiles[i];
            let temp = fsRoot();
            for (const part of fsComponents(path)) {
                temp = fsJoin(temp, part);
                set.add(temp);
            }
        }
        if (set.size) {
            set.add(fsRoot());
        }
        return set;
    }, [unsavedFiles]);

    const [expandedPaths, setExpandedPaths] = useState<string[]>([]);
    const setIsExpanded = useCallback((path: string, isExpanded: boolean) => {
        if (isExpanded) {
            setExpandedPaths((x) => [...x, path]);
        } else {
            setExpandedPaths((x) => x.filter((p) => p !== path));
        }
    }, []);

    if (!showFileTree && openedFile) {
        return null;
    }

    return (
        <div className={styles.editorTreeContainer}>
            <TreeDirNode
                name={rootPath || ""}
                path={fsRoot()}
                level={0}
                listDir={listDir}
                onClickFile={async (path) => {
                    const editor = kernel.getEditor();
                    if (!editor) {
                        return;
                    }
                    editor.notifyActivity();
                    await editor.openFile(path);
                }}
                setIsExpanded={setIsExpanded}
                expandedPaths={expandedPaths}
                dirtyPaths={dirtyPaths}
                openedFile={openedFile}
            />
        </div>
    );
};

type TreeDirNodeProps = {
    /// Name of the entry, should end with /
    name: string;
    /// Full path of the directory entry, should end with /
    path: string;
    /// Level of in the tree this node is in. 0 is the root.
    level: number;
    /// Function to list the contents of a directory
    listDir: (path: string) => Promise<string[]>;
    /// Callback for when a file is clicked
    onClickFile: (path: string) => void;
    /// Callback for toggling the expanded state of the node
    setIsExpanded: (path: string, isExpanded: boolean) => void;

    /// Directory paths that are expanded
    ///
    /// All should end with /
    expandedPaths: string[];

    /// File and directory paths that have unsaved changes
    ///
    /// Directories should end with /
    dirtyPaths: Set<string>;
    /// The file currently opened in the editor
    openedFile: string | undefined;
};

const TreeDirNode: React.FC<TreeDirNodeProps> = ({
    name,
    path,
    listDir,
    onClickFile,
    level,
    setIsExpanded,
    openedFile,
    expandedPaths,
    dirtyPaths,
}) => {
    const [entries, setEntries] = useState<string[] | undefined>(undefined);

    const isExpanded = expandedPaths.includes(path);

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
    }, [path, isExpanded, listDir]);

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
                isDirty={dirtyPaths.has(path)}
            />
            {isExpanded &&
                entries !== undefined &&
                entries.map((entry, i) => {
                    if (entry.endsWith("/")) {
                        // remove the trailing / from the entry
                        // since file/directory are displayed through icon
                        const name = entry.slice(0, -1);
                        return (
                            <TreeDirNode
                                key={i}
                                name={name}
                                path={fsJoin(path, name)}
                                level={level + 1}
                                listDir={listDir}
                                onClickFile={onClickFile}
                                setIsExpanded={setIsExpanded}
                                expandedPaths={expandedPaths}
                                dirtyPaths={dirtyPaths}
                                openedFile={openedFile}
                            />
                        );
                    } else {
                        const filePath = fsJoin(path, entry);
                        return (
                            <TreeItem
                                key={i}
                                file={entry}
                                isDirectory={false}
                                isSelected={filePath === openedFile}
                                onClickFile={() => {
                                    onClickFile(filePath);
                                }}
                                level={level + 1}
                                isLoading={false}
                                isDirty={dirtyPaths.has(filePath)}
                            />
                        );
                    }
                })}
        </>
    );
};
// const TreeDirNode = memo(TreeDirNodeInternal);

/// Compare function for sorting entries in the file tree
function compareEntry(a: string, b: string): number {
    const isADir = a.endsWith("/");
    const isBDir = b.endsWith("/");
    if (isADir && !isBDir) {
        return -1;
    }
    if (!isADir && isBDir) {
        return 1;
    }
    return a.localeCompare(b);
}
