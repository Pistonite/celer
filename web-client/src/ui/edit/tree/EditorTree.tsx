
import clsx from 'clsx';
import { useEffect, useState } from 'react';
import { TreeItem } from './TreeItem';

export type EditorTreeProps = {
    listDir: (path: string) => Promise<string[]>;
    onClickFile: (path: string) => void;
}

const memo: {[key: string]: string[]} = {};

const testListDir = (path: string): Promise<string[]> => {
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

export const EditorTree: React.FC<EditorTreeProps> = ({ files, onClickFile }) => {
    const [expandedPaths, setExpandedPaths] = useState<string[]>([]);
    console.log(expandedPaths);
    return (
        <div id="editor-tree-root">
            <TreeDirNode 
                name="(root)"
                path="/"
                listDir={testListDir}
                onClickFile={onClickFile}
                getIsExpanded={(path) => expandedPaths.includes(path)}
                setIsExpanded={(path, isExpanded) => {

                    if (isExpanded) {
                        setExpandedPaths([...expandedPaths, path]);
                    } else {
                        setExpandedPaths(expandedPaths.filter((p) => path !== p));
                    }
                }}
                level={0}
            />
        </div>
    );
}

type TreeDirNodeProps = {
    name: string;
    path: string;
    listDir: (path: string) => Promise<string[]>;
    onClickFile: (path: string) => void;
    level: number;
    getIsExpanded: (path: string) => boolean;
    setIsExpanded: (path: string, isExpanded: boolean) => void;
}

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
                file={name + (isLoading ? " (loading...)" : "")}
                path={path} 
                isDirectory={true}
                isSelected={false} 
                isExpanded={isExpanded}
                onClickFile={() => {
                    setIsExpanded(path, !isExpanded);
                } } 
                level={level} 
            />
            {
                isExpanded && entries !== undefined && entries.map((entry, i) => {
                    if (entry.endsWith("/")) {
                        const subPath = `${path}${entry}`;
                        return (
                            <TreeDirNode
                                key={i}
                                name={entry.slice(0, -1)}
                                path={subPath}
                                listDir={listDir}
                                onClickFile={onClickFile}
                                getIsExpanded={getIsExpanded}
                                setIsExpanded={setIsExpanded}
                                level={level+1}
                            />
                        );
                    } else {
                        const subPath = `${path}${entry}`;
                        return (
                            <TreeItem 
                                key={i}
                                file={entry} 
                                path={subPath} 
                                isDirectory={false} 
                                isSelected={false} 
                                onClickFile={() => {
                                    onClickFile(subPath);
                                }}
                                level={level+1}
                            />
                        );
                    }
                })
            }
        </>
    );
}


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

}
