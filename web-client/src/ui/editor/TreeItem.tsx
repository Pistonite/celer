import { mergeClasses } from "@fluentui/react-components";
import { ChevronRight16Regular } from "@fluentui/react-icons";

import { TreeIcon } from "./TreeIcon";
import { useEditorStyles } from "./styles";

export type TreeItemProps = {
    /// Displayed file name
    file: string;
    /// Callback when the file is clicked
    onClickFile: () => void;
    /// Level of the file in the tree. 0 is the root.
    level: number;
    /// If this entry is a directory
    isDirectory: boolean;
    /// Should the entry be displayed as selected
    isSelected: boolean;
    /// Should the entry be displayed as expanded
    isExpanded?: boolean;
    isLoading: boolean;
    isDirty: boolean;
};

/// A single directory or file entry in the file tree
///
/// Does not display content of directory
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
    const styles = useEditorStyles();

    return (
        <div
            className={mergeClasses(
                styles.editorTreeItem,
                isSelected && styles.editorTreeItemSelected,
            )}
            style={{ paddingLeft: level * LEVEL_INDENT }}
            onClick={() => {
                onClickFile();
            }}
        >
            <span
                className={mergeClasses(
                    styles.editorTreeItemIcon,
                    isExpanded && styles.editorTreeItemIconExpanded,
                )}
            >
                {isDirectory ? <ChevronRight16Regular /> : null}
            </span>
            <TreeIcon file={file} isDirectory={isDirectory} />
            <span className={styles.editorTreeItemName}>
                {file}
                {isLoading && " (loading...)"}
                {isDirty && "*"}
            </span>
        </div>
    );
};
