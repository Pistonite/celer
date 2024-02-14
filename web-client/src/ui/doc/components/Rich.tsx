//! Rich text component

import { Text, TextProps } from "@fluentui/react-components";

import { DocRichText, DocRichTextBlock } from "low/celerc";
import { smartMergeClasses } from "low/utils";

import {
    DocLineTextRichClass,
    RichTextColorClass,
    getTagClassName,
} from "./dom";
import { useDocStyles } from "./styles";

/// Rich text display component
type RichProps = {
    /// The text to display
    content: DocRichText;
    /// Size of the text
    size: TextProps["size"];
};

export const Rich: React.FC<RichProps> = ({ content, size }) => {
    // if all blocks are white spaces, return a non-breaking space to keep the line height
    if (!content.find((t) => t.text.trim())) {
        return <span>&nbsp;</span>;
    }
    return (
        <>
            {content.map((richText, index) => (
                <RichBlock key={index} {...richText} size={size} />
            ))}
        </>
    );
};

/// Internal rich text display component
type RichBlockProps = DocRichTextBlock & Partial<TextProps>;

const RichBlock: React.FC<RichBlockProps> = ({ text, tag, link, ...rest }) => {
    const styles = useDocStyles();
    if (!tag) {
        return (
            <Text as="span" {...rest}>
                {text}
            </Text>
        );
    }

    return (
        <Text
            as="span"
            className={smartMergeClasses(
                styles,
                RichTextColorClass,
                DocLineTextRichClass.className,
                tag && getTagClassName(tag),
            )}
            {...rest}
        >
            {link ? (
                <a href={link} target="_blank">
                    {text}
                </a>
            ) : (
                text
            )}
        </Text>
    );
};
