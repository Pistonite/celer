//! Components for the document viewer

import clsx from "clsx";
import React from "react";

import { Text } from "@fluentui/react-components";
import { DocDiagnostic, RichText, removeTags } from "data/model";
import { Rich } from "./Rich";
import { DocLineContainerClass } from "./util";
import { useActions, viewActions } from "data/store";

/// One line in the document
type DocLineProps = {
    /// Section index of the line, used for tracking line position
    sectionIndex: number;
    /// Line index within the section, used for tracking line position
    lineIndex: number;
    /// Color of the line
    lineColor: string;
    /// The text to display
    text: RichText[];
    /// Url of the icon to display
    iconUrl?: string;
    /// Secondary text
    secondaryText: RichText[];
    /// Counter properties
    counterText?: RichText;
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
                    }}
                >
                    {counterText && (
                        <div
                            className="docline-counter"
                            style={{
                                backgroundColor: counterText.tag?.background,
                                color: counterText.tag?.color,
                            }}
                        >
                            <Text size={500} font="monospace">
                                {counterText.text}
                            </Text>
                        </div>
                    )}
                </div>
                <div className="docline-body">
                    {iconUrl && (
                        <div className="docline-icon-container">
                            <img src={iconUrl} alt="icon" />
                        </div>
                    )}
                    <div className="docline-text-container">
                        <div className="docline-primary-text">
                            {removeTags(text).trim().length === 0 ? (
                                <span>&nbsp;</span>
                            ) : (
                                <Rich size={500} content={text} />
                            )}
                        </div>
                        {secondaryText.length > 0 && (
                            <div className="docline-secondary-text">
                                <Rich size={400} content={secondaryText} />
                            </div>
                        )}
                    </div>
                </div>
            </div>
            {diagnostics.map(({ message, type, source }, i) => (
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
                        <Text size={300} font="monospace">
                            {message}
                        </Text>
                    </div>
                </div>
            ))}
        </div>
    );
};
