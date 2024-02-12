import { smartMergeClasses } from "low/utils";

import { DocLineProps } from "./props";
import { useDocStyles } from "./styles";
import { Rich } from "./Rich";
import { DocLineTextSecondaryClass } from "./dom";

export type DocLineTextSecondaryProps = Pick<DocLineProps, "secondaryText">;

export const DocLineTextSecondary: React.FC<DocLineTextSecondaryProps> = ({
    secondaryText,
}) => {
    const styles = useDocStyles();
    if (!secondaryText.length) {
        return null;
    }
    return (
        <div className={smartMergeClasses(styles, DocLineTextSecondaryClass)}>
            <Rich size={400} content={secondaryText} />
        </div>
    );
};
