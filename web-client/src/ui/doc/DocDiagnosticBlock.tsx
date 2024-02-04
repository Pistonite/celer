//! The diagnostic text block

import clsx from "clsx";
import { Text, makeStyles, shorthands } from "@fluentui/react-components";

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
const DocDiagnosticBodyClass = new DOMClass(`${PREFIX}-body`);
const useStyles = makeStyles({
    [DocDiagnosticHeadClass.className]: {
        ...shorthands.padding("4px 4px 0 4px"),
    },
    [DocDiagnosticBodyClass.className]: {
        ...shorthands.padding("4px 0 4px 4px"),
        wordBreak: "break-word",
    },
});

export const DocDiagnosticBlock: React.FC<DocDiagnosticBlockProps> = ({
    diagnostic,
    showCaret,
}) => {
    const { msg, source, type } = diagnostic;
    const styles = useStyles();
    return (
        <div className={PREFIX}>
            <div
                className={clsx(
                    DocDiagnosticHeadClass.styledClassName(styles),
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
                    DocDiagnosticBodyClass.styledClassName(styles),
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
