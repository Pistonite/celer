import { makeStyles, shorthands } from "@fluentui/react-components";

export const useAppStyles = makeStyles({
    widgetContainer: {
        boxSizing: "border-box",
        display: "flex",
    },
    widgetContainerEditing: {
        ...shorthands.border("1px", "solid", "#000"),
        ...shorthands.borderRadius("2px"),
    },
    widgetToolbarTop: {
        flexDirection: "column",
    },
    widgetToolbarBottom: {
        flexDirection: "column-reverse",
    },
    widget: {
        ...shorthands.flex(1, 1, "auto"),
        minWidth: 0,
        minHeight: 0,
    },
});
