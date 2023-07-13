//! Components for the document viewer

import "./DocLine.css";
import clsx from "clsx";

import { Run20Regular } from "@fluentui/react-icons";

/// The head of DocLine
///
/// This is the indicator for error/warning and the line color.
/// This is also the clickable part to center the map on the line.
type DocLindHeadProps = {
    /// If the line is selected
    selected: boolean;
    /// Mode
    mode: "error" | "warning" | "normal";
    /// Color of the line
    lineColor: string;
};

const DocLineHead: React.FC<DocLindHeadProps> = ({
    selected,
    mode,
    lineColor,
}) => {
    return (
        <div className={clsx(
            "docline-head",
            mode === "error" && "docline-head-error",
            mode === "normal" && "docline-head-normal",
            mode === "warning" && "docline-head-warning",
        )} style={{
                borderColor: lineColor,
            }}>
            { selected && <Run20Regular /> }
        </div>
    );
};
