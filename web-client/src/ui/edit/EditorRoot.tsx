
import "./Editor.css";
import { EditorTree } from "./tree";

// const EditorContainerId = "editor-container";
export const EditorRoot: React.FC = () => {
    return (
        <div id="editor-root">
            <EditorTree files={[
                "/test1.js",
                "/test2.js",
            ]}
                onClickFile={(path) => {
                    console.log(path);
                }}
            />
            <div>Click a file to open it</div>
        </div>
    )
};
