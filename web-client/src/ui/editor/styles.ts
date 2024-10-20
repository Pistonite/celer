import { makeStyles, shorthands } from "@fluentui/react-components";
import { prefersColorScheme } from "low/utils";

export const useEditorStyles = makeStyles({
    editorRoot: {
        height: "100%",
        display: "flex",
        flexDirection: "row",
    },
    editorPanel: {
        display: "flex",
        flexGrow: 1,
        flexDirection: "column",
        boxSizing: "border-box",
        [prefersColorScheme("light")]: {
            ...shorthands.borderLeft("1px", "solid", "#ccc"),
        },
        [prefersColorScheme("dark")]: {
            ...shorthands.borderLeft("1px", "solid", "#333"),
            backgroundColor: "#1e1e1e",
        },
    },
    editorFileName: {
        ...shorthands.padding("4px"),
        ...shorthands.borderBottom("1px", "solid", "#ccc"),
    },
    editorOuterContainer: {
        flex: 1,
    },
    editorContainer: {
        height: "100%",
    },
    editorDropZone: {
        width: "100%",
        height: "100%",
        display: "flex",
        justifyContent: "center",
        alignItems: "center",
        ...shorthands.border("2px", "dashed", "#888"),
        boxSizing: "border-box",
        transitionDuration: "0.2s",
    },
    editorDropZoneDragging: {
        color: "#479ef5",
        ...shorthands.borderColor("#479ef5"),
    },
    editorTreeContainer: {
        minWidth: "200px",
        maxWidth: "200px",
        overflowX: "auto",
    },
    editorTreeItem: {
        display: "flex",
        flexDirection: "row",
        alignItems: "center",
        cursor: "pointer",
        pointerEvents: "auto",
        "&:hover": {
            backgroundColor: "#88888888",
        },
    },
    editorTreeItemName: {
        display: "inline-block",
        marginLeft: "4px",
        textOverflow: "ellipsis",
        whiteSpace: "nowrap",
    },
    editorTreeItemSelected: {
        backgroundColor: "#88888844",
    },
    editorTreeItemIcon: {
        display: "inline-block",
        minWidth: "16px",
        height: "16px",
    },
    editorTreeItemIconExpanded: {
        rotate: "90deg",
    },
    fileTypeFolder: {
        color: "#61afef",
    },
    fileTypeJs: {
        color: "#f1e05a",
    },
    fileTypeJson: {
        color: "#f1e05a",
    },
    fileTypeTs: {
        color: "#61afef",
    },
    fileTypePy: {
        color: "#61afef",
    },
    fileTypeYaml: {
        color: "#61afef",
    },
    fileTypeMd: {
        color: "#61afef",
    },
    fileTypeImage: {
        color: "#af81af",
    },
    fileTypeUnknown: {},
});
