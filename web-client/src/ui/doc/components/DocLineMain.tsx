import { smartMergeClasses } from "low/utils";

import { DocLineProps } from "./props";
import { DocLineHead } from "./DocLineHead";
import { DocLineCounter } from "./DocLineCounter";
import { DocLineIcon } from "./DocLineIcon";
import { DocLineTextSecondary } from "./DocLineTextSecondary";
import { Rich } from "./Rich";
import { useDocStyles } from "./styles";
import { DocLineBodyClass, DocLineMainBannerClass, DocLineMainClass, DocLineTextContainerClass, DocLineTextPrimaryClass } from "./dom";

/// Root container of a document line
export const DocLineMain: React.FC<DocLineProps> = ({
    sectionIndex,
    lineIndex,
    lineColor,
    text,
    iconUrl,
    secondaryText,
    counterText,
    isBanner,
}) => {
    const styles = useDocStyles();
    return (
        <div
            className={smartMergeClasses(
                styles,
                DocLineMainClass,
                isBanner && DocLineMainBannerClass,
            )}
        >
            <DocLineHead
                sectionIndex={sectionIndex}
                lineIndex={lineIndex} 
                lineColor={lineColor}>
                <DocLineCounter counterText={counterText} />
            </DocLineHead>
            <div className={smartMergeClasses(styles, DocLineBodyClass)}>
                <DocLineIcon iconUrl={iconUrl} />
                <div
                    className={smartMergeClasses(
                        styles,
                        DocLineTextContainerClass,
                    )}
                >
                    <div className={smartMergeClasses(styles, DocLineTextPrimaryClass)}>
                        <Rich size={500} content={text} />
                    </div>
                    <DocLineTextSecondary secondaryText={secondaryText} />
                </div>
            </div>
        </div>
    );
}
