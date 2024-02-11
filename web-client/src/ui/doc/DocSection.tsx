import { PropsWithChildren } from "react";
import { Text } from "@fluentui/react-components";

import { smartMergeClasses } from "low/utils";

import {
    useDocStyles,
    DocSectionHeadClass,
    DocSectionContainerClass,
    DocSectionBodyClass,
} from "./styles";

type DocSectionProps = {
    /// The section name
    name: string;
    /// The section index
    index: number;
};

/// Component for one section in the document
export const DocSection: React.FC<PropsWithChildren<DocSectionProps>> = ({
    name,
    index,
    children,
}) => {
    const styles = useDocStyles();
    return (
        <div
            className={DocSectionContainerClass.className}
            data-section={index}
        >
            <div className={smartMergeClasses(styles, DocSectionHeadClass)}>
                <Text size={700}>{name || "\u00a0"}</Text>
            </div>
            <div className={DocSectionBodyClass.className}>{children}</div>
        </div>
    );
};
