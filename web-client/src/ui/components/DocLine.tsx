//! Components for the document viewer

import "./DocLine.css";
import clsx from "clsx";

import { Text } from "@fluentui/react-components";
import { PersonRunning20Regular } from "@fluentui/react-icons";
import { DocCounter, RichText } from "data/model";
import { Rich } from "./Rich";

/// The head of DocLine
///
/// This is the indicator for error/warning and the line color.
/// This is also the clickable part to center the map on the line.
type DocLineProps = {
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
    secondaryText?: RichText[];
    /// Counter properties
    counter?: {
        /// Style of the counter
        style: DocCounter;
        /// Text of the counter
        text: string;
    };

};

export const DocLine: React.FC<DocLineProps> = ({
    selected,
    mode,
    lineColor,
    text,
    iconUrl,
    secondaryText,
    counter
}) => {
    return (
        <div className="docline-container">
            <div className={clsx(
                "docline-head",
                mode === "error" && "docline-head-error",
                mode === "normal" && "docline-head-normal",
                mode === "warning" && "docline-head-warning",
            )} style={{
                borderColor: lineColor,
            }}>
                {
                    counter &&
                    <div className="docline-counter" style={{ 
                            backgroundColor: counter.style.background, 
                            color: counter.style.color 
                        }} >
                        <Text size={500} font="monospace">
                            {counter.text}
                        </Text>
                    </div>
                }
                {selected && <div className="docline-cursor">
                    <PersonRunning20Regular />
                </div>
                }
            </div>
            <div className="docline-body">
                {
                    iconUrl &&
                    <div className="docline-icon-container">
                        <img src={iconUrl} alt="icon" />
                    </div>
                }
                <div className="docline-text-container">

                    <div className="docline-primary-text">
                        <Rich size={500} content={text} />
                    </div>
                    {
                        secondaryText && <div className="docline-secondary-text">
                            <Rich size={400} content={secondaryText} />
                        </div>
                    }
                </div>
            </div>
        </div>
    );
};

