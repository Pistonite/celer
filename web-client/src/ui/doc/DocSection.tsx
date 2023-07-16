import { PropsWithChildren } from "react";
import { Text } from "@fluentui/react-components";

/// Component for one section in the document
type DocSectionProps = {
    /// The section name
    name: string;
};

export const DocSection: React.FC<PropsWithChildren<DocSectionProps>> = ({
    name,
    children,
}) => {
    return (
        <div className="docsection-container">
            <div className="docsection-head">
                <Text size={700}>{name || "\u00a0"}</Text>
            </div>
            <div className="docsection-body">{children}</div>
        </div>
    );
};
