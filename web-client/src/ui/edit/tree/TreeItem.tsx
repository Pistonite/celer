import {
    ChevronRight16Regular,
} from '@fluentui/react-icons';
import clsx from "clsx";
import { TreeIcon } from './TreeIcon';

export type TreeItemProps = {
    // Displayed file name
    file: string;
    // Path of the file
    path: string;
    // Whether the file is a directory
    isDirectory: boolean;
    // Whether the file is selected
    isSelected: boolean;
    // Callback when the file is clicked
    onClickFile: () => void;
    // Level of the file in the tree. 0 is the root.
    level: number;
    // For directory, if it is expanded
    isExpanded?: boolean;
}

export const TreeItem: React.FC<TreeItemProps> = ({
    file, path, isDirectory, isSelected, onClickFile, level, isExpanded
}) => {
    const LEVEL_INDENT = 8 /* px */;

    return (
        <div 
            className={clsx("editor-tree-item", isSelected && "editor-tree-item-selected")} 
            style={{paddingLeft: level*LEVEL_INDENT}}
            onClick={() => {
                onClickFile();
            }}
        >
            <span className={clsx("editor-tree-item-icon", isExpanded && "editor-tree-item-expanded")}>
                {isDirectory ? <ChevronRight16Regular /> : null}
            </span>
            <TreeIcon isDirectory={isDirectory} file={file} />
            <span className="editor-tree-item-name">
                {file}
            </span>
        </div>
    );
}


