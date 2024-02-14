import { ErrorBoundary } from "ui/shared";

import { useEditorStyles } from "./styles";
import { EditorRoot } from "./EditorRoot";

export const Editor: React.FC = () => {
    const styles = useEditorStyles();
    return (
        <ErrorBoundary>
            <div className={styles.editorRoot}>
                <EditorRoot />
            </div>
        </ErrorBoundary>
    );
};
