import { useSelector } from "react-redux";

import { HintScreen } from "ui/shared";
import { settingsSelector, viewSelector } from "core/store";

import { EditorTree } from "./EditorTree";
import { EditorDropZone } from "./EditorDropZone";
import { EditorPanel } from "./EditorPanel";

export const EditorRoot: React.FC = () => {
    const { rootPath } = useSelector(viewSelector);
    const { editorMode } = useSelector(settingsSelector);

    if (rootPath === undefined) {
        return <EditorDropZone />;
    }

    if (editorMode === "external") {
        return (
            <HintScreen>
                <p>
                    Web editor is not available because you are using the
                    external editor workflow.
                </p>
                <p>
                    Switch to the default layout or a layout without the editor
                    to hide this widget.
                </p>
            </HintScreen>
        );
    }

    return (
        <>
            <EditorTree />
            <EditorPanel />
        </>
    );
};
