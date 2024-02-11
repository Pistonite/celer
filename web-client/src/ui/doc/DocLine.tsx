//! Components for the document viewer

import React from "react";
import { Text } from "@fluentui/react-components";

import { viewActions } from "core/store";
import { DocDiagnostic, DocRichText, DocRichTextBlock } from "low/celerc";
import { useActions } from "low/store";
import { concatClassName, smartMergeClasses } from "low/utils";

import { Rich } from "./Rich";
import { getTagClassName } from "./utils";
import { DocDiagnosticBlock } from "./DocDiagnosticBlock";
import {
    DocLineBodyClass,
    DocLineContainerClass,
    DocLineCounterClass,
    DocLineCounterPrefix,
    DocLineIconContainerClass,
    DocLineMainBannerClass,
    DocLineMainClass,
    DocLineSplitClass,
    DocLineTextContainerClass,
    DocLineTextPrimaryClass,
    DocLineTextSecondaryClass,
    DocLineWithIconClass,
    useDocStyles,
} from "./styles";

/// One line in the document
type DocLineProps = {
    /// Section index of the line, used for tracking line position
    sectionIndex: number;
    /// Line index within the section, used for tracking line position
    lineIndex: number;
    /// Color of the line
    lineColor: string;
    /// The text to display
    text: DocRichText;
    /// Url of the icon to display
    iconUrl?: string;
    /// Secondary text
    secondaryText: DocRichText;
    /// Counter properties
    counterText?: DocRichTextBlock;
    /// Counter type if any
    counterType?: string;
    /// Diagnostic messages
    diagnostics: DocDiagnostic[];
    /// If the line is a banner
    isBanner: boolean;
    /// If the line is a split. Will be false if disabled in the settings
    isSplit: boolean;
    /// The split type (display string). Should be present even if isSplit is false
    splitType: string | undefined;
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
    isBanner,
    isSplit,
    splitType,
}) => {
    const { setDocLocation } = useActions(viewActions);
    const styles = useDocStyles();
    return (
        <div
            className={smartMergeClasses(
                styles,
                DocLineContainerClass,
                iconUrl && DocLineWithIconClass.className,
                isSplit && DocLineSplitClass.className,
                counterType &&
                    concatClassName(DocLineCounterPrefix, counterType),
            )}
            data-section={sectionIndex}
            data-line={lineIndex}
            data-split-type={splitType}
        >
            <div
                className={smartMergeClasses(
                    styles,
                    DocLineMainClass,
                    isBanner && DocLineMainBannerClass,
                )}
            >
                <div
                    className={smartMergeClasses(styles, "docline-head")}
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
                    <DocLineCounter text={counterText} />
                </div>
                <div className={smartMergeClasses(styles, DocLineBodyClass)}>
                    <DocLineIcon src={iconUrl} />
                    <div
                        className={smartMergeClasses(
                            styles,
                            DocLineTextContainerClass,
                        )}
                    >
                        <DocLineTextPrimary text={text} />
                        <DocLineTextSecondary text={secondaryText} />
                    </div>
                </div>
            </div>
            {diagnostics.map((diagnostic, i) => (
                <DocDiagnosticBlock key={i} diagnostic={diagnostic} showCaret />
            ))}
        </div>
    );
};

const DocLineCounter: React.FC<{ text?: DocRichTextBlock }> = ({
    text: counterText,
}) => {
    const styles = useDocStyles();
    if (!counterText) {
        return null;
    }
    return (
        <div
            className={smartMergeClasses(
                styles,
                DocLineCounterClass,
                // TODO
                counterText.tag && getTagClassName(counterText.tag),
            )}
        >
            <Text size={500} font="monospace">
                {counterText.text}
            </Text>
        </div>
    );
};

const DocLineIcon: React.FC<{ src?: string }> = ({ src }) => {
    const styles = useDocStyles();
    if (!src) {
        return null;
    }
    return (
        <div
            className={smartMergeClasses(styles, DocLineIconContainerClass)}
            aria-hidden="true"
        >
            <img src={src} />
        </div>
    );
};

type DocLineTextProps = {
    text: DocRichText;
};
const DocLineTextPrimary: React.FC<DocLineTextProps> = ({ text }) => {
    const styles = useDocStyles();
    return (
        <div className={smartMergeClasses(styles, DocLineTextPrimaryClass)}>
            <Rich size={500} content={text} />
        </div>
    );
};

const DocLineTextSecondary: React.FC<DocLineTextProps> = ({ text }) => {
    const styles = useDocStyles();
    if (!text.length) {
        return null;
    }
    return (
        <div className={smartMergeClasses(styles, DocLineTextSecondaryClass)}>
            <Rich size={400} content={text} />
        </div>
    );
};
