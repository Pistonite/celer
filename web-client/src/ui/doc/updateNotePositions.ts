//! Logic for updating positions of note block in the document view
//!
//! Note block are initially all hidden. When updated, the note block of the current line
//! is anchored to the line, while the above and below note blocks are attempted to be
//! anchored to their corresponding lines, and pushed up/down if needed.

import {
    DocLog,
    getLineLocationFromElement,
    getScrollContainerOffsetY,
    findLineByIndex,
} from "./utils";

/// The id of the note panel
export const DocNoteContainerId = "doc-side";

/// Async note update position event serial
///
/// Used to cancel previous events
let updateNotesSerial = 0;

/// Async notes update speed (ms per note block)
const NoteUpdateDelay = 10;

/// Layout notes based on the given position
///
/// If position not given, layout from the first note
export const updateNotePositions = (baseLine: HTMLElement): void => {
    const noteContainer = document.getElementById(DocNoteContainerId);
    if (!noteContainer) {
        return;
    }

    const baseIndex = findBaseNoteIndex(noteContainer.children, baseLine);
    if (baseIndex >= noteContainer.children.length) {
        // no notes to layout
        return;
    }
    // Cancel the previous async updates
    updateNotesSerial += 1;
    const containerOffsetY = getScrollContainerOffsetY();

    // Layout the base note
    const baseNoteBlock = noteContainer.children[baseIndex] as HTMLElement;
    const [sectionIndex, lineIndex] = getLineLocationFromElement(baseNoteBlock);
    const lineElement = findLineByIndex(sectionIndex, lineIndex);
    if (!lineElement) {
        return;
    }
    const baseTop = lineElement.getBoundingClientRect().y - containerOffsetY;
    setNotePostion(baseNoteBlock, baseTop);

    // Layout block before it asynchronously with delay
    const layoutNoteBeforeAsync = (
        serial: number,
        i: number,
        top: number,
    ): void => {
        if (serial !== updateNotesSerial) {
            // cancelled
            return;
        }
        const noteBlock = noteContainer.children[i] as HTMLElement;
        if (!noteBlock) {
            DocLog.warn(`update note position called with invalid index ${i}`);
            // index out of bound
            return;
        }
        // The minimum top
        top -= noteBlock.clientHeight;
        // Get the line position so we can anchor the note to it
        const [sectionIndex, lineIndex] = getLineLocationFromElement(noteBlock);
        const lineElement = findLineByIndex(sectionIndex, lineIndex);
        if (lineElement) {
            const lineTop =
                lineElement.getBoundingClientRect().y - containerOffsetY;
            top = Math.min(top, lineTop);
        }
        const changed = setNotePostion(noteBlock, top);
        if (changed && i > 0) {
            window.setTimeout(() => {
                layoutNoteBeforeAsync(serial, i - 1, top);
            }, NoteUpdateDelay);
        } else {
            DocLog.info(`finished updating note positions at ${i}`);
        }
    };
    if (baseIndex > 0) {
        layoutNoteBeforeAsync(updateNotesSerial, baseIndex - 1, baseTop);
    }

    // Layout block after it
    const layoutNoteAfterAsync = (
        serial: number,
        i: number,
        top: number,
    ): void => {
        if (serial !== updateNotesSerial) {
            // cancelled
            return;
        }
        const noteBlock = noteContainer.children[i] as HTMLElement;
        if (!noteBlock) {
            DocLog.warn(`update note position called with invalid index ${i}`);
            return;
        }
        const [sectionIndex, lineIndex] = getLineLocationFromElement(noteBlock);
        const lineElement = findLineByIndex(sectionIndex, lineIndex);
        if (lineElement) {
            const lineTop =
                lineElement.getBoundingClientRect().y - containerOffsetY;
            top = Math.max(top, lineTop);
        }
        const changed = setNotePostion(noteBlock, top);
        if (changed && i < noteContainer.children.length - 1) {
            window.setTimeout(() => {
                layoutNoteAfterAsync(
                    serial,
                    i + 1,
                    top + noteBlock.clientHeight,
                );
            }, NoteUpdateDelay);
        } else {
            DocLog.info(`finished updating note positions at ${i}`);
        }
    };
    if (baseIndex < noteContainer.children.length - 1) {
        layoutNoteAfterAsync(
            updateNotesSerial,
            baseIndex + 1,
            baseTop + baseNoteBlock.clientHeight,
        );
    }
};

/// Helper for setting the position of a note block
///
/// Return if the position is changed
const setNotePostion = (noteBlock: HTMLElement, top: number): boolean => {
    const newTop = `${top}px`;
    if (noteBlock.style.top === newTop) {
        return false;
    }
    noteBlock.style.display = "block";
    noteBlock.style.position = "absolute";
    noteBlock.style.top = newTop;

    return true;
};

/// Find the first note block index that is equal or after the given line
const findBaseNoteIndex = (
    noteBlocks: HTMLElement["children"],
    baseLine: HTMLElement,
): number => {
    // binary search for the note corresponding to the line
    // if the line doesn't have the note, return the next note
    let lo = 0;
    let hi = noteBlocks.length - 1;
    const [baseSectionIndex, baseLineIndex] =
        getLineLocationFromElement(baseLine);
    while (lo <= hi) {
        const mid = Math.floor((lo + hi) / 2);
        const noteBlock = noteBlocks[mid] as HTMLElement;
        const [noteSectionIndex, noteLineIndex] =
            getLineLocationFromElement(noteBlock);
        if (noteSectionIndex < baseSectionIndex) {
            lo = mid + 1;
        } else if (noteSectionIndex > baseSectionIndex) {
            hi = mid - 1;
        } else if (noteLineIndex < baseLineIndex) {
            lo = mid + 1;
        } else if (noteLineIndex > baseLineIndex) {
            hi = mid - 1;
        } else {
            lo = mid;
            break;
        }
    }
    return lo;
};
