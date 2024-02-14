import { useSelector } from "react-redux";

import { viewSelector } from "core/store";
import { HintScreen } from "ui/shared";
import { Body1 } from "@fluentui/react-components";
import { useEditorStyles } from "./styles";
import { EditorContainer } from "./EditorContainer";

export const EditorPanel: React.FC = () => {
    const { openedFile, unsavedFiles } = useSelector(viewSelector);
    const styles = useEditorStyles();
    if (openedFile === undefined) {
        return (
            <div className={styles.editorPanel}>
                <HintScreen>Click a file to open it</HintScreen>
            </div>
        );
    }
    return (
        <div className={styles.editorPanel}>
            <div className={styles.editorFileName}>
                <Body1>
                    {openedFile}
                    {unsavedFiles.includes(openedFile) && "*"}
                </Body1>
            </div>
            <div className={styles.editorOuterContainer}>
                <EditorContainer />
            </div>
        </div>
    );
};
