//! Components for the document viewer

import clsx from "clsx";
import React from "react";
import { Text } from "@fluentui/react-components";

// import { RichText, removeTags } from "core/doc";
import { viewActions } from "core/store";
import { DocDiagnostic, DocRichText } from "low/celerc";
import { useActions } from "low/store";

import { Rich } from "./Rich";
import { DocLineContainerClass, findLineByIndex, getTagClassName } from "./utils";
import { Poor } from "./Poor";
import { updateNotePositions } from "./updateNotePositions";

/// One line in the document
type DocLineProps = {
    /// Section index of the line, used for tracking line position
    sectionIndex: number;
    /// Line index within the section, used for tracking line position
    lineIndex: number;
    /// Color of the line
    lineColor: string;
    /// The text to display
    text: DocRichText[];
    /// Url of the icon to display
    iconUrl?: string;
    /// Secondary text
    secondaryText: DocRichText[];
    /// Counter properties
    counterText?: DocRichText;
    /// Counter type if any
    counterType?: string;
    /// Diagnostic messages
    diagnostics: DocDiagnostic[];
};

export const DocLine: React.FC<DocLineProps> = ({
    sectionIndex,
    lineIndex,
    lineColor,
    text,
    iconUrl,
    secondaryText,
    counterText,
    counterType,
    diagnostics,
}) => {
    const { setDocLocation } = useActions(viewActions);
    return (
        <div
            className={DocLineContainerClass}
            data-section={sectionIndex}
            data-line={lineIndex}
        >
            <div className="docline-main">
                <div
                    className={clsx("docline-head")}
                    style={{
                        borderColor: lineColor,
                    }}
                    onClick={() => {
                        setDocLocation({
                            section: sectionIndex,
                            line: lineIndex,
                        });
                        const line = findLineByIndex(sectionIndex, lineIndex);
                        if (line) {
                            updateNotePositions(line);
                        }
                    }}
                >
                    {counterText && (
                        <div
                            className={clsx("docline-counter", counterText.tag && getTagClassName(counterText.tag))}
                        >
                            <Text size={500} font="monospace">
                                {counterText.text}
                            </Text>
                        </div>
                    )}
                </div>
                <div
                    className={clsx(
                        "docline-body",
                        counterType && `docline-body-${counterType}`,
                    )}
                >
                    {iconUrl && (
                        <div className="docline-icon-container">
                            <img src={iconUrl} alt="icon" />
                        </div>
                    )}
                    <div className="docline-text-container">
                        <div
                            className={clsx(
                                "docline-primary-text",
                                iconUrl && "docline-icon-text",
                            )}
                        >
                                <Rich size={500} content={text} />   
                        </div>
                        {secondaryText.length > 0 && (
                            <div
                                className={clsx(
                                    "docline-secondary-text",
                                    iconUrl && "docline-icon-text",
                                )}
                            >
                                <Rich size={400} content={secondaryText} />
                            </div>
                        )}
                    </div>
                </div>
            </div>
            {diagnostics.map(({ msg, type, source }, i) => (
                <div className="docline-diagnostic" key={i}>
                    <div
                        className={clsx(
                            "docline-diagnostic-head",
                            `docline-diagnostic-${type}`,
                        )}
                    >
                        <Text size={300} font="monospace">
                            ^^^ {type}: {source}:
                        </Text>
                    </div>
                    <div
                        className={clsx(
                            "docline-diagnostic-body",
                            `docline-diagnostic-${type}`,
                        )}
                    >
                        <Poor
                            content={msg}
                            textProps={{ size: 300, font: "monospace" }}
                        />
                    </div>
                </div>
            ))}
        </div>
    );
};
