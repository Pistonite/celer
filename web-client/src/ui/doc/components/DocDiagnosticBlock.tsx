//! The diagnostic text block

import { Text } from "@fluentui/react-components";

import type { DocDiagnostic } from "low/celerc";
import { concatClassName, smartMergeClasses } from "low/utils";

import { Poor } from "./Poor";
import { useDocStyles } from "./styles";
import {
    DocLineDiagnosticBodyClass,
    DocLineDiagnosticClass,
    DocLineDiagnosticHeadClass,
    DocLineDiagnosticPrefix,
} from "./dom";

export type DocDiagnosticBlockProps = {
    /// The diagnostic to display
    diagnostic: DocDiagnostic;
    /// Show caret pointing to the line above
    showCaret: boolean;
};

export const DocDiagnosticBlock: React.FC<DocDiagnosticBlockProps> = ({
    diagnostic,
    showCaret,
}) => {
    const { msg, source, type } = diagnostic;
    const styles = useDocStyles();
    const extraClass = concatClassName(DocLineDiagnosticPrefix, type);
    return (
        <div className={DocLineDiagnosticClass.className}>
            <div
                className={smartMergeClasses(
                    styles,
                    DocLineDiagnosticHeadClass,
                    extraClass,
                )}
            >
                <Text size={300} font="monospace">
                    {showCaret && <span aria-hidden="true">{"^^^ "}</span>}
                    {type}: {source}:
                </Text>
            </div>
            <div
                className={smartMergeClasses(
                    styles,
                    DocLineDiagnosticBodyClass,
                    extraClass,
                )}
            >
                <Poor
                    content={msg}
                    textProps={{ size: 300, font: "monospace" }}
                />
            </div>
        </div>
    );
};
