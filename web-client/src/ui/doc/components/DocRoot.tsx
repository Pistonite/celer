import { ExecDoc } from "low/celerc";
import { smartMergeClasses } from "low/utils";

import { DocContainerComp } from "./DocContainerComp";
import { DocEndComp } from "./DocEndComp";
import { DocMainPanel } from "./DocMainPanel";
import { DocNotePanel } from "./DocNotePanel";
import {
    DocContentContainer,
    DocDiagnosticContainer,
    DocPrefaceBlockClass,
    DocPrefaceContainer,
} from "./dom";
import { Rich } from "./Rich";
import { DocDiagnosticBlock } from "./DocDiagnosticBlock";
import { useDocStyles } from "./styles";

export type DocRootProps = {
    /// Serial number of the document
    ///
    /// Will only re-render if the serial number changes
    serial: number;
    /// The document to render
    document: ExecDoc;
    /// The split types of the document
    splitTypes: string[];
    /// Callback when the document is scrolled
    onScroll: () => void;
    /// Callback when the document is rendered
    onRender: () => void;
};
/// Main doc viewer component
///
/// The document is not connected to the store to prevent
/// accidental re-renders. Do not use useSelector in this component.
export const DocRoot: React.FC<DocRootProps> = ({
    document,
    splitTypes,
    onScroll,
    onRender,
}) => {
    onRender();

    const styles = useDocStyles();

    return (
        <DocContainerComp onScroll={onScroll}>
            <div id={DocDiagnosticContainer.id}>
                {document.diagnostics.map((diagnostic, i) => (
                    <DocDiagnosticBlock
                        key={i}
                        diagnostic={diagnostic}
                        showCaret={false}
                    />
                ))}
            </div>
            <div id={DocPrefaceContainer.id}>
                {document.preface.map((text, i) => (
                    <div
                        key={i}
                        className={smartMergeClasses(
                            styles,
                            DocPrefaceBlockClass,
                        )}
                    >
                        <Rich content={text} size={400} />
                    </div>
                ))}
            </div>
            <div
                id={DocContentContainer.id}
                className={styles.docContentContainer}
            >
                <DocMainPanel
                    document={document}
                    splitTypes={new Set(splitTypes)}
                />
                <DocNotePanel document={document} />
            </div>
            <DocEndComp />
        </DocContainerComp>
    );
};
