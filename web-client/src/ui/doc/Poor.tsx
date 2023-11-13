//! Poor text component

import { Text, TextProps } from "@fluentui/react-components";

import { DocPoorText, DocPoorTextBlock } from "low/celerc";

/// Poor text display component
type PoorProps = {
    /// The text to display
    content: DocPoorText;
    /// Size of the text
    textProps: TextProps;
};

export const Poor: React.FC<PoorProps> = ({ content, textProps }) => {
    if (!content.find((t) => t.data)) {
        return <span>&nbsp;</span>;
    }
    return (
        <>
            {content.map((poorText, index) => (
                <PoorBlock key={index} {...poorText} textProps={textProps} />
            ))}
        </>
    );
};

/// Internal rich text display component
type PoorBlockProps = DocPoorTextBlock & {
    textProps: TextProps;
};

const PoorBlock: React.FC<PoorBlockProps> = ({ type, data, textProps }) => {
    return (
        <Text as="span" {...textProps}>
            {type === "link" ? (
                <a href={data} target="_blank">
                    {data}
                </a>
            ) : (
                data
            )}
        </Text>
    );
};
