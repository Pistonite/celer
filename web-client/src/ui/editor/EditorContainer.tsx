import { useSelector } from "react-redux";

import { HintScreen } from "ui/shared";
import { viewSelector } from "core/store";
import { EditorContainerDOM } from "core/editor";

import { useEditorStyles } from "./styles";

export const EditorContainer: React.FC = () => {
    const { currentFileSupported } = useSelector(viewSelector);
    const styles = useEditorStyles();

    if (!currentFileSupported) {
        return (
            <HintScreen>
                Cannot open this file. Either this is not a text file or there
                was a problem opening it.
            </HintScreen>
        );
    }

    return (
        <div id={EditorContainerDOM.id} className={styles.editorContainer} />
    );
};
