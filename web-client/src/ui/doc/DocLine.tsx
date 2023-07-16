//! Components for the document viewer

import clsx from "clsx";
import React from "react";

import { Text } from "@fluentui/react-components";
import { PersonRunning20Regular } from "@fluentui/react-icons";
import { RichText } from "data/model";
import { Rich } from "./Rich";
import { DocLineContainerClass } from "./util";

/// One line in the document
type DocLineProps = {
    /// Section index of the line, used for tracking line position
    sectionIndex: number;
    /// Line index within the section, used for tracking line position
    lineIndex: number;
    /// If the line is selected
    selected: boolean;
    /// Mode
    mode: "error" | "warning" | "normal";
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
};

export const DocLine: React.FC<DocLineProps> = ({
    sectionIndex,
    lineIndex,
    selected,
    mode,
    lineColor,
    text,
    iconUrl,
    secondaryText,
    counterText,
}) => {
    return (
        <div
            className={DocLineContainerClass}
            data-section={sectionIndex}
            data-line={lineIndex}
        >
            <div
                className={clsx(
                    "docline-head",
                    mode === "error" && "docline-head-error",
                    mode === "normal" && "docline-head-normal",
                    mode === "warning" && "docline-head-warning",
                )}
                style={{
                    borderColor: lineColor,
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
                {selected && (
                    <div className="docline-cursor">
                        <PersonRunning20Regular />
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
                        <Rich size={500} content={text} />
                    </div>
                    {secondaryText.length > 0 && (
                        <div className="docline-secondary-text">
                            <Rich size={400} content={secondaryText} />
                        </div>
                    )}
                </div>
            </div>
        </div>
    );
};
