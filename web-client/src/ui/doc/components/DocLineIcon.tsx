import { smartMergeClasses } from "low/utils";
import { DocLineProps } from "./props";
import { useDocStyles } from "./styles";
import { DocLineIconContainerClass } from "./dom";

export type DocLineIconProps = Pick<DocLineProps, "iconUrl">;

/// Icon in the document line. Only shows if iconUrl is provided
export const DocLineIcon: React.FC<DocLineIconProps> = ({ iconUrl }) => {
    const styles = useDocStyles();
    if (!iconUrl) {
        return null;
    }
    return (
        <div
            className={smartMergeClasses(styles, DocLineIconContainerClass)}
            aria-hidden="true"
        >
            <img src={iconUrl} />
        </div>
    );
};
