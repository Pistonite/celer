import { makeStyles, shorthands } from "@fluentui/react-components";

export const useHeaderStyles = makeStyles({
    header: {
        ...shorthands.overflow("hidden"),
        display: "flex",
        minHeight: "80px",
    },
    top: {
        flexDirection: "column",
    },
    bottom: {
        flexDirection: "column-reverse",
    },
    toolbar: {
        ...shorthands.overflow("hidden"),
    },
    title: {
        display: "flex",
        flexDirection: "row",
    },
    titleText: {
        ...shorthands.margin(0),
        ...shorthands.overflow("hidden"),
        whiteSpace: "nowrap",
        textOverflow: "ellipsis",
    },
    logo: {
        width: "32px",
        height: "32px",
    },
});
