import { PropsWithChildren } from "react";
import { Text, mergeClasses } from "@fluentui/react-components";

import { SectionBannerWidthClass } from "./updateBannerWidths";
import { DocSectionHead } from "./utils";
import { useDocStyles } from "./styles";

/// Component for one section in the document
type DocSectionProps = {
    /// The section name
    name: string;
    /// The section index
    index: number;
};

export const DocSection: React.FC<PropsWithChildren<DocSectionProps>> = ({
    name,
    index,
    children,
}) => {
    const styles = useDocStyles();
    return (
        <div className="docsection-container" data-section={index}>
            <div
                className={mergeClasses(
                    DocSectionHead.styledClassName(styles),
                    SectionBannerWidthClass,
                )}
            >
                <Text size={700}>{name || "\u00a0"}</Text>
            </div>
            <div className="docsection-body">{children}</div>
        </div>
    );
};
