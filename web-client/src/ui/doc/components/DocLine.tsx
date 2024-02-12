//! Components for the document viewer

import React from "react";

import { concatClassName, smartMergeClasses } from "low/utils";

import { DocDiagnosticBlock } from "./DocDiagnosticBlock";
import { DocLineProps } from "./props";
import { DocLineMain } from "./DocLineMain";
import { useDocStyles } from "./styles";
import { DocLineContainerClass, DocLineCounterPrefix, DocLineSplitClass, DocLineWithIconClass } from "./dom";

/// One line in the document
export const DocLine: React.FC<DocLineProps> = (props) => {
    const  {
        sectionIndex,
        lineIndex,
        iconUrl,
        counterType,
        diagnostics,
        isSplit,
        splitType,
    } = props;
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
            <DocLineMain {...props} />
            {diagnostics.map((diagnostic, i) => (
                <DocDiagnosticBlock key={i} diagnostic={diagnostic} showCaret />
            ))}
        </div>
    );
};

