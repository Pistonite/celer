import { PropsWithChildren } from "react";

import { viewActions } from "core/store";
import { smartMergeClasses } from "low/utils";
import { useActions } from "low/store";

import { DocLineProps } from "./props";
import { useDocStyles } from "./styles";
import { DocLineHeadClass } from "./dom";

export type DocLineHeadProps = PropsWithChildren<Pick<DocLineProps, "sectionIndex" | "lineIndex" | "lineColor">>;

/// Head portion of a line
///
/// Contains the counter block and the line color indicator
export const DocLineHead: React.FC<DocLineHeadProps> = ({ sectionIndex, lineIndex, lineColor, children }) => {
    const { setDocLocation } = useActions(viewActions);
    const styles = useDocStyles();
    return (
        <div
            className={smartMergeClasses(styles, DocLineHeadClass)}
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
            {children}
        </div>
    );
};
