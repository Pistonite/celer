import { makeStyles, shorthands } from "@fluentui/react-components";
import { DocSectionHead } from "./utils";

export const useDocStyles = makeStyles({
    [DocSectionHead.className]: {
        boxSizing: "border-box",
        ...shorthands.padding("16px 0px 16px 64px"),
        "& span": {
            wordWrap: "break-word",
        },
    },
});
