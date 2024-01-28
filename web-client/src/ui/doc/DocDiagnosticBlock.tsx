//! The diagnostic text block

import clsx from "clsx";
import { Text } from "@fluentui/react-components";

import { DocDiagnostic } from "low/celerc";
import { DOMClass } from "low/utils";

import { Poor } from "./Poor";

export type DocDiagnosticBlockProps = {
    /// The diagnostic to display
    diagnostic: DocDiagnostic;
    /// Show caret pointing to the line above
    showCaret: boolean;
};

const PREFIX = "docline-diagnostic";

const DocDiagnosticHeadClass = new DOMClass(`${PREFIX}-head`);
DocDiagnosticHeadClass.style({
    padding: "4px 4px 0 4px",
});
const DocDiagnosticBodyClass = new DOMClass(`${PREFIX}-body`);
DocDiagnosticBodyClass.style({
    padding: "4px 0 4px 4px",
    "word-break": "break-word",
});

export const DocDiagnosticBlock: React.FC<DocDiagnosticBlockProps> = ({
    diagnostic,
    showCaret,
}) => {
    const { msg, source, type } = diagnostic;
    return (
        <div className={PREFIX}>
            <div
                className={clsx(
                    DocDiagnosticHeadClass.className,
                    `${PREFIX}-${type}`,
                )}
            >
                <Text size={300} font="monospace">
                    {showCaret ? "^^^ " : ""}
                    {type}: {source}:
                </Text>
            </div>
            <div
                className={clsx(
                    DocDiagnosticBodyClass.className,
                    `${PREFIX}-${type}`,
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
