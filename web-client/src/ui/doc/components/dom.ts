import { CSSVariable, DOMClass, DOMId, concatClassName } from "low/utils";

// DOM Nodes
//
// This is the structure:
// DocScroll
//   DocContainer
//     DocDiagnosticContainer
//     DocPrefaceContainer
//     DocContentContainer
//       DocMainContainer
//         DocSectionContainerClass
//           DocSectionHeadClass
//           DocSectionBodyClass
//             DocLineContainerClass
//             DocLineContainerClass
//             ...
//         DocSectionContainerClass
//         ...
//       DocNoteContainer
//         DocNoteContainerClass
//           DocNoteBlockClass
//           ...
//         DocNoteContainerClass
//         ...
//     DocEnd

export const DocScroll = new DOMId("doc-scroll");
export const DocContainer = new DOMId("doc-container");
export const DocDiagnosticContainer = new DOMId("docdiagnostic-container");
export const DocPrefaceContainer = new DOMId("docpreface-container");
export const DocContentContainer = new DOMId("doccontent-container");
export const DocMainContainer = new DOMId("doc-main");
export const DocNoteContainer = new DOMId("doc-side");
export const DocEnd = new DOMId("doc-end");

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
export const DocLineHeadClass = new DOMClass("docline-head");
export const DocLineIconContainerClass = new DOMClass("docline-icon-container");
export const DocLineMainClass = new DOMClass("docline-main");
export const DocLineMainBannerClass = new DOMClass("docline-main-banner");
export const DocLineTextContainerClass = new DOMClass("docline-text-container");
export const DocLineTextPrimaryClass = new DOMClass("docline-text-primary");
export const DocLineTextSecondaryClass = new DOMClass("docline-text-secondary");
export const DocNoteBlockClass = new DOMClass("docnote-block");
export const DocNoteContainerClass = new DOMClass("docnote-container");
export const DocPrefaceBlockClass = new DOMClass("docpreface-block");
export const DocSectionHeadClass = new DOMClass("docsection-head");

// Marker-only, no styles

/// Marks the current line container
export const DocLineCurrentClass = new DOMClass("docline-current");
/// Marks the line is a split
export const DocLineSplitClass = new DOMClass("docline-split");
/// Marks the line has icon
export const DocLineWithIconClass = new DOMClass("docline-with-icon");

export const DocSectionContainerClass = new DOMClass("docsection-container");

export const DocSectionBodyClass = new DOMClass("docsection-body");
/// Mark the text is rich text in the document (not in the counter)
export const DocLineTextRichClass = new DOMClass("docline-text-rich");
/// Mark the note container as expanded (hovered when note column is small)
export const DocNoteContainerExpandedClass = new DOMClass(
    "docnote-container-expanded",
);

// Dynamic banner
export const DocContainerWidthVariable = new CSSVariable(
    "--doc-container-width",
);

// Rich Text
/// Marks receiver of rich text color
export const RichTextColorClass = new DOMClass("rich-text-color");
export const RichTextVariables = {
    fg: {
        light: new CSSVariable("--tag-fg-l"),
        dark: new CSSVariable("--tag-fg-d"),
    },
    bg: {
        light: new CSSVariable("--tag-bg-l"),
        dark: new CSSVariable("--tag-bg-d"),
    },
} as const;

export const getTagClassName = (tag: string) => {
    return `tag-${getTagId(tag)}`;
};

const tagIdMap = new Map<string, number>();

function getTagId(tag: string) {
    const id = tagIdMap.get(tag);
    if (id) {
        return id;
    }
    const nextTagId = tagIdMap.size + 1;
    tagIdMap.set(tag, nextTagId);
    return nextTagId;
}
