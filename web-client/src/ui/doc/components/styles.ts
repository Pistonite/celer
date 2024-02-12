import { makeStyles, shorthands } from "@fluentui/react-components";

import { prefersColorScheme, px } from "low/utils";
import { DocContainerWidthVariable, DocLineBodyClass, DocLineContainerClass, DocLineCounterClass, DocLineCurrentClass, DocLineDiagnosticBodyClass, DocLineDiagnosticHeadClass, DocLineHeadClass, DocLineIconContainerClass, DocLineMainBannerClass, DocLineMainClass, DocLineTextContainerClass, DocLineTextPrimaryClass, DocLineTextSecondaryClass, DocPrefaceBlockClass, DocSectionHeadClass, RichTextColorClass, RichTextVariables } from "./dom";

// DocLineCurrentClass.style({
//     [` .${DocLineHeadClass.className}`]: {
//         "border-right-width": "24px",
//     }
// });

// DocRichTextColorClass.style({});

// Style constants
export const DOC_ICON_WIDTH = 50;
export const DOC_HEAD_WIDTH = 64;
export const DOC_MAIN_MAX_WIDTH = 380;
export const DOC_NOTE_MIN_WIDTH = 48;

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
    docContainer: {
        display: "flex",
        flexDirection: "column",
        height: "100%",
        ["--note-min-width" as string]: px(DOC_NOTE_MIN_WIDTH),
    },
    docContentContainer: {
        display: "flex"
    },
    docMainContainer: {
        minWidth: `min(calc(100% - ${px(DOC_NOTE_MIN_WIDTH)}), ${px(DOC_MAIN_MAX_WIDTH)})`,
        maxWidth: px(DOC_MAIN_MAX_WIDTH),
    },
    docNoteContainer: {
        // need this because note blocks have position: absolute
        position: "relative",
        ...shorthands.flex(1),
        minWidth: px(DOC_NOTE_MIN_WIDTH),
        // need for container query on the notes
        containerType: "size",
    },
    docEnd: {
        ...shorthands.flex(1),
        textAlign: "center",
        ...shorthands.padding("32px"),
        color: "#888",
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
    [DocLineHeadClass.className]: {
        minWidth: px(DOC_HEAD_WIDTH),
        ...shorthands.borderRight("4px", "solid"),
        boxSizing: "border-box",
        position: "relative",
        textAlign: "right",
        transitionDuration: "0.1s",
        cursor: "pointer",
        [`:global(.${DocLineCurrentClass.className})`]: {
            borderRightWidth: "24px",
        }
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
    [DocPrefaceBlockClass.className]: {
        ...shorthands.padding("4px", 0, "4px", "8px"),
        boxSizing: "border-box",
    },
    [DocSectionHeadClass.className]: {
        boxSizing: "border-box",
        ...shorthands.padding("16px", 0, "16px", px(DOC_HEAD_WIDTH)),
        "& span": {
            wordWrap: "break-word",
        },
        width: `${DocContainerWidthVariable.fallback("initial")} !important`,
    },

    // These provide the baseline for changing rich text
    // color with color scheme. Each theme should override
    // this based on if each surface has light or dark background
    [RichTextColorClass.className]: {
        [prefersColorScheme("light")]: {
            color: RichTextVariables.fg.light.fallback("inherit"),
            backgroundColor: RichTextVariables.bg.light.fallback("inherit"),
        },
        [prefersColorScheme("dark")]: {
            color: RichTextVariables.fg.dark.fallback("inherit"),
            backgroundColor: RichTextVariables.bg.dark.fallback("inherit"),
        },
    }
});
