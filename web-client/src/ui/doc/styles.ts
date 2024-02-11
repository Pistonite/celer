import { makeStyles, shorthands } from "@fluentui/react-components";

import { CSSVariable, DOMClass, concatClassName, px } from "low/utils";

// Style constants
export const DOC_ICON_WIDTH = 50;
export const DOC_HEAD_WIDTH = 64;

// Classes - please keep sorted
// These class names are kept stable because they are used in the theme stylesheets
export const DocLineCounterPrefix = "docline-counter";
export const DocLineDiagnosticPrefix = "docline-diagnostic";

export const DocLineBodyClass = new DOMClass("docline-body");
export const DocLineContainerClass = new DOMClass("docline-container");
export const DocLineCounterClass = new DOMClass(DocLineCounterPrefix);
export const DocLineDiagnosticBodyClass = new DOMClass(
    concatClassName(DocLineDiagnosticPrefix, "body"),
);
export const DocLineDiagnosticClass = new DOMClass(DocLineDiagnosticPrefix);
export const DocLineDiagnosticHeadClass = new DOMClass(
    concatClassName(DocLineDiagnosticPrefix, "head"),
);
export const DocLineIconContainerClass = new DOMClass("docline-icon-container");
export const DocLineMainClass = new DOMClass("docline-main");
export const DocLineMainBannerClass = new DOMClass("docline-main-banner");
export const DocLineTextContainerClass = new DOMClass("docline-text-container");
export const DocLineTextPrimaryClass = new DOMClass("docline-text-primary");
export const DocLineTextSecondaryClass = new DOMClass("docline-text-secondary");
export const DocSectionHeadClass = new DOMClass("docsection-head");

// Marker-only, no styles
export const DocLineSplitClass = new DOMClass("docline-split");
export const DocLineWithIconClass = new DOMClass("docline-with-icon");
export const DocSectionContainerClass = new DOMClass("docsection-container");
export const DocSectionBodyClass = new DOMClass("docsection-body");

// Dynamic banner
export const DocContainerWidthVariable = new CSSVariable(
    "--doc-container-width",
);

export const useDocStyles = makeStyles({
    // by id
    docScroll: {
        height: "100%",
        // make document scrollable
        overflowY: "auto",
        // Hide the horizontal scrollbar which could show during layout operations
        overflowX: "hidden",
        scrollBehavior: "smooth",
    },

    // by class
    [DocLineBodyClass.className]: {
        boxSizing: "border-box",
        ...shorthands.padding("4px"),
        flexGrow: 1,
        display: "flex",
    },
    [DocLineContainerClass.className]: {
        boxSizing: "border-box",
    },
    [DocLineCounterClass.className]: {
        position: "absolute",
        boxSizing: "border-box",
        top: 0,
        left: 0,
        width: px(DOC_HEAD_WIDTH),
        height: "32px",
        ...shorthands.padding("4px"),
        textOverflow: "ellipsis",
        ...shorthands.overflow("hidden"),
    },
    [DocLineDiagnosticHeadClass.className]: {
        ...shorthands.padding("4px", "4px", 0, "4px"),
    },
    [DocLineDiagnosticBodyClass.className]: {
        ...shorthands.padding("4px", 0, "4px", "4px"),
        wordBreak: "break-word",
    },
    [DocLineIconContainerClass.className]: {
        minWidth: px(DOC_ICON_WIDTH),
        height: px(DOC_ICON_WIDTH),
        "& img": {
            width: px(DOC_ICON_WIDTH),
        },
    },
    [DocLineMainClass.className]: {
        display: "flex",
    },
    [DocLineMainBannerClass.className]: {
        width: `${DocContainerWidthVariable.fallback("initial")} !important`,
    },
    [DocLineTextContainerClass.className]: {
        width: "100%",
    },
    [DocLineTextPrimaryClass.className]: {
        "& span": {
            wordWrap: "break-word",
        },
    },
    [DocLineTextSecondaryClass.className]: {
        "& span": {
            wordWrap: "break-word",
        },
    },
    [DocSectionHeadClass.className]: {
        boxSizing: "border-box",
        ...shorthands.padding("16px", 0, "16px", px(DOC_HEAD_WIDTH)),
        "& span": {
            wordWrap: "break-word",
        },
        width: `${DocContainerWidthVariable.fallback("initial")} !important`,
    },
});
