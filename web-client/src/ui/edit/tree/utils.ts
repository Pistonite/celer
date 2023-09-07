export type TreeEntry = {
    name: string;
    children?: TreeEntry[];
}
export type TreeDir = TreeEntry & {
    children: TreeEntry[];
}

export const findOrCreateDir = (dir: TreeDir, path: string[], i: number): TreeDir => {
    if (i >= path.length) {
        return dir;
    }
    const name = path[i];
    let child = dir.children.find(child => child.name === name);
    if (!child) {
        child = {
            name,
            children: []
        };
        dir.children.push(child);
    }
    if (!child.children) {
        child.children = [];
    }
    return findOrCreateDir(child as TreeDir, path, 1);
}

/// Create tree structure from flat file path list
export const createTree = (files: string[]): TreeDir => {
    const root = {
        type: "d",
        name: "/",
        children: []
    };
    const dirMap: Record<string, TreeDir> = {
        "/": root
    };
    files.forEach(file => {
        file = file.trim();
        if (file.startsWith("/")) {
            file = file.slice(1);
        }
        if (!file) {
            return;
        }
        const parts = file.split("/");
        const fileName = parts.pop() as string;
        const parentDir = parts.join("/");
        let dir = dirMap[parentDir];
        if (!dir) {
            dir = findOrCreateDir(root, parts, 0);
            dirMap[parentDir] = dir;
        }
        dir.children.push({
            name: fileName
        });
    });

    return root;
}
