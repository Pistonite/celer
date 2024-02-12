import { Text } from "@fluentui/react-components";

import { ExecDoc } from "low/celerc";

import { smartMergeClasses } from "low/utils";
import { useDocStyles } from "./styles";
import { DocSectionBodyClass, DocSectionContainerClass, DocSectionHeadClass } from "./dom";
import { DocLine } from "./DocLine";

export type DocSectionProps = {
    /// The section index
    index: number;
    /// The document to render
    document: ExecDoc;
    /// The split types from the settings
    splitTypes: Set<string>;
};

/// Component for one section in the document
export const DocSection: React.FC<DocSectionProps> = ({
    index,
    document,
    splitTypes,
}) => {
    const styles = useDocStyles();
    const section = document.route[index];
    const project = document.project;
    return (
        <div
            className={DocSectionContainerClass.className}
            data-section={index}
        >
            <div className={smartMergeClasses(styles, DocSectionHeadClass)}>
                <Text size={700}>{section.name || "\u00a0"}</Text>
            </div>
            <div className={DocSectionBodyClass.className}>
                {section.lines.map((line, j) => {
                    const { counterText, icon } = line;
                    const counterTag = counterText?.tag || undefined;
                    const splitType = counterTag && project.tags[counterTag] ?.splitType;
                    const isSplit = splitType && splitTypes.has(splitType);
                    const iconUrl = icon ? project.icons[ icon ] : undefined;
                    return (
                        <DocLine
                            sectionIndex={index}
                            lineIndex={j}
                            key={j}
                            diagnostics={line.diagnostics}
                            lineColor={line.lineColor}
                            text={line.text}
                            iconUrl={iconUrl}
                            secondaryText={line.secondaryText}
                            counterText={line.counterText}
                            counterType={counterTag}
                            isBanner={line.isBanner}
                            isSplit={!!isSplit}
                            splitType={splitType || undefined}
                        />
                    );
                })}
            </div>
        </div>
    );
};
