//! Monaco File Tree Node
//!
//! Adopted from https://github.com/BlueMagnificent/monaco-tree/blob/master/src/monaco-tree/tree-node.js

export class TreeNode {
    // File name
    public name: string;
    // If this node is a directory
    public isDirectory: boolean;
    // Parent node
    public parent: TreeNode | undefined;
    // Children nodes
    public children: TreeNode[];

    constructor(
        name: string,
        isDirectory: boolean,
        parent: TreeNode | undefined,
    ) {
        this.name = name;
        this.isDirectory = isDirectory;
        this.children = [];
        this.parent = parent;
    }

    /// Get full path of this node
    public getPath(): string {
        if (!this.parent) {
            return "/";
        }

        return `${this.parent.getPath()}${this.name}${
            this.isDirectory ? "/" : ""
        }`;
    }

    public isDescendantOf(treeNode: TreeNode): boolean {
        let parent = this.parent;
        while (parent) {
            if (parent === treeNode) {
                return true;
            }
            parent = parent.parent;
        }
        return false;
    }
}
