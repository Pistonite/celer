//! Rich text component

import { Text, TextProps } from "@fluentui/react-components";
import { RichText } from "data/model";

/// Rich text display component
type RichProps = {
    /// The text to display
    content: RichText[];
    /// Size of the text
    size: TextProps["size"];
};

export const Rich: React.FC<RichProps> = ({ content, size }) => {
    return (
        <>
            {
                content.map((richText, index) => (
                    <RichBlock key={index} {...richText} size={size} />
                ))
            }
        </>
    );
};

/// Internal rich text display component
type RichBlockProps = RichText & {
    size: TextProps["size"];
};

const RichBlock: React.FC<RichBlockProps> = ({ text, tag, size }) => {
    if (!tag) {
        return <Text as="span" size={size} >{text}</Text>;
    }

    return (
        <Text
            as="span"
            size={size}
            weight={tag.bold ? "bold" : "regular"}
            underline={tag.underline}
            strikethrough={tag.strikethrough}
            italic={tag.italic}
            style={{
                color: tag.color,
                backgroundColor: tag.background,
            }}
        >
            {
                tag.link? <a href={tag.link}>{text}</a> : text
            }
        </Text>

    );
};
