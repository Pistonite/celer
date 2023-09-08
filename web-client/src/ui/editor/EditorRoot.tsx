
import "./Editor.css";
import { EditorDropZone } from "./EditorDropZone";
import { EditorTree } from "./tree";

// const EditorContainerId = "editor-container";
const memo: {[key: string]: string[]} = {};

const testListDir = (paths: string[]): Promise<string[]> => {
    const path= paths.join("/");
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
export const EditorRoot: React.FC = () => {
    // <EditorTree 
    //     rootName="root"
    //     listDir={testListDir}
    //     onClickFile={(path) => {
    //         console.log(path);
    //     }}
    // />
    //     <div>Click a file to open it</div>
    return (
        <div id="editor-root">
            <EditorDropZone onFileSysCreate={(fileSys) => {
                console.log(fileSys);
            }}/>
        </div>
    );
};
