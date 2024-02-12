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
export const DocSectionHeadClass = new DOMClass("docsection-head");

// Marker-only, no styles
export const DocLineCurrentClass = new DOMClass("docline-current");
export const DocLineSplitClass = new DOMClass("docline-split");
export const DocLineWithIconClass = new DOMClass("docline-with-icon");
export const DocSectionContainerClass = new DOMClass("docsection-container");
export const DocSectionBodyClass = new DOMClass("docsection-body");

// Dynamic banner
export const DocContainerWidthVariable = new CSSVariable(
    "--doc-container-width",
);
