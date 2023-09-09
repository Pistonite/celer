//! File System Tree View implemented with Monaco Tree

import { TreeNode } from "./TreeNode";
import { TreeLegacy } from "./reexport";

export type MonacoTreeProps = {
    files: string[];
    onClickFile: (path: string) => void;
};

export const MonacoTree: React.FC = () => {
    const treeRef = useRef<TreeLegacy>(null);
};
