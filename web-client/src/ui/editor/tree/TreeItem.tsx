import { ChevronRight16Regular } from "@fluentui/react-icons";
import clsx from "clsx";
import { TreeIcon } from "./TreeIcon";

export type TreeItemProps = {
    // Displayed file name
    file: string;
    // Callback when the file is clicked
    onClickFile: () => void;
    // Level of the file in the tree. 0 is the root.
    level: number;

    isDirectory: boolean;
    isSelected: boolean;
    isExpanded?: boolean;
    isLoading: boolean;
    isDirty?: boolean;
};

export const TreeItem: React.FC<TreeItemProps> = ({
    file,
    isDirectory,
    isSelected,
    onClickFile,
    level,
    isExpanded,
    isLoading,
    isDirty,
}) => {
    const LEVEL_INDENT = 8; /* px */

    return (
        <div
            className={clsx(
                "editor-tree-item",
                isSelected && "editor-tree-item-selected",
            )}
            style={{ paddingLeft: level * LEVEL_INDENT }}
            onClick={() => {
                onClickFile();
            }}
        >
            <span
                className={clsx(
                    "editor-tree-item-icon",
                    isExpanded && "editor-tree-item-expanded",
                )}
            >
                {isDirectory ? <ChevronRight16Regular /> : null}
            </span>
            <TreeIcon file={file} isDirectory={isDirectory} />
            <span className="editor-tree-item-name">
                {file}
                {isLoading && " (loading...)"}
                {isDirty && "*"}
            </span>
        </div>
    );
};
