import clsx from "clsx";
import { PropsWithChildren } from "react";
import { Text } from "@fluentui/react-components";

import { SectionBannerWidthClass } from "./updateBannerWidths";
import { DocSectionHead } from "./utils";

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
    return (
        <div className="docsection-container" data-section={index}>
            <div className={clsx(DocSectionHead.className, SectionBannerWidthClass)}>
                <Text size={700}>{name || "\u00a0"}</Text>
            </div>
            <div className="docsection-body">{children}</div>
        </div>
    );
};
