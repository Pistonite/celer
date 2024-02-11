import { DOMId } from "low/utils";

// DOM Nodes by id
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
