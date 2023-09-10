import "./EditorTree.css";
import { useEffect, useState } from "react";
import { TreeItem } from "./TreeItem";

export type EditorTreeProps = {
    /// Name of the root directory
    rootName: string;

    /// Callback for listing a directory.
    ///
    /// The path is path segments from the root directory.
    /// Each segment does not contain any "/".
    /// Empty array means the root directory.
    ///
    /// Returns the file and directory names in the directory.
    /// For directories, the returned name should end with "/".
    listDir: (path: string[]) => Promise<string[]>;

    /// Callback when a file is clicked.
    onClickFile: (path: string[]) => void;
};

export const EditorTree: React.FC<EditorTreeProps> = ({
    rootName,
    listDir,
    onClickFile,
}) => {
    const [expandedPaths, setExpandedPaths] = useState<string[]>([""]);
    return (
        <div id="editor-tree-root">
            <TreeDirNode
                name={rootName}
                path={[]}
                listDir={listDir}
                onClickFile={onClickFile}
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
};

const TreeDirNode: React.FC<TreeDirNodeProps> = ({
    name,
    path,
    listDir,
    onClickFile,
    level,
    getIsExpanded,
    setIsExpanded,
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
                            />
                        );
                    } else {
                        return (
                            <TreeItem
                                key={i}
                                file={entry}
                                isDirectory={false}
                                isSelected={false}
                                onClickFile={() => {
                                    onClickFile([...path, entry]);
                                }}
                                level={level + 1}
                                isLoading={false}
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
