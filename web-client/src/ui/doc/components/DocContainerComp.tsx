import { PropsWithChildren } from "react";

import { DocContainer, DocScroll } from "./dom";
import { useDocStyles } from "./styles";

export type DocContainerProps = PropsWithChildren<{
    /// Event handler for scrolling
    onScroll: () => void;
}>;

/// Scrolling container for the document
export const DocContainerComp: React.FC<DocContainerProps> = ({ onScroll, children }) => {
    const styles = useDocStyles();
    return (
        <div
            id={DocScroll.id}
            className={styles.docScroll}
            onScroll={onScroll}
            onKeyDown={(e) => {
                // prevent default scrolling behavior
                // because we have our own
                e.preventDefault();
            }}
        >
            <div
                id={DocContainer.id}
                className={styles.docContainer}
            >
                {children}
            </div>
        </div>
    );
}
