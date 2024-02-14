import { Text } from "@fluentui/react-components";

import { smartMergeClasses } from "low/utils";

import { DocLineProps } from "./props";
import { useDocStyles } from "./styles";
import {
    DocLineCounterClass,
    RichTextColorClass,
    getTagClassName,
} from "./dom";

export type DocLineCounterProps = Pick<DocLineProps, "counterText">;

/// The counter block in the line head that can show up to 5 characters
export const DocLineCounter: React.FC<DocLineCounterProps> = ({
    counterText,
}) => {
    const styles = useDocStyles();
    if (!counterText) {
        return null;
    }
    return (
        <div
            className={smartMergeClasses(
                styles,
                DocLineCounterClass,
                RichTextColorClass,
                counterText.tag && getTagClassName(counterText.tag),
            )}
        >
            <Text size={500} font="monospace">
                {counterText.text}
            </Text>
        </div>
    );
};
