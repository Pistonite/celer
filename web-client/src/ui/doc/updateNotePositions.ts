//! Logic for updating positions of note block in the document view
//!
//! Note block are initially all hidden. When updated, the note block of the current line
//! is anchored to the line, while the above and below note blocks are attempted to be
//! anchored to their corresponding lines, and pushed up/down if needed.

import { createYielder } from "low/utils";
import {
    DocLog,
    getLineLocationFromElement,
    getScrollContainerOffsetY,
    findLineByIndex,
    DocContentContainerId,
} from "./utils";

/// The id of the note panel
export const DocNoteContainerId = "doc-side";

/// Layout notes based on the given position
///
/// If position not given, layout from the first note
export const updateNotePositions = async (
    // The base line element to layout the notes from
    baseLine: HTMLElement,
    // Position intervals the note blocks should avoid overlapping with
    _avoidIntervals: number[][],
    // Callback to check if the update should be cancelled
    shouldCancel: () => boolean,
): Promise<void> => {
    DocLog.info("updating note positions...");
    const noteContainer = document.getElementById(DocNoteContainerId);
    if (!noteContainer || noteContainer.children.length === 0) {
        // no notes to layout
        return;
    }

    const baseIndex = findBaseNoteIndex(noteContainer.children, baseLine);

    // if (baseIndex >= 0 && baseIndex < noteContainer.children.length) {
    //     // no notes to layout
    //     return;
    // }

    // // Cancel the previous async updates
    // updateNotesSerial += 1;
    
    const containerOffsetY = getScrollContainerOffsetY(DocContentContainerId);

    // Layout the base note
    const baseNoteBlock = noteContainer.children[baseIndex] as HTMLElement;
    const [sectionIndex, lineIndex] = getLineLocationFromElement(baseNoteBlock);
    const lineElement = findLineByIndex(sectionIndex, lineIndex);
    if (!lineElement) {
        return;
    }
    const baseTop = lineElement.getBoundingClientRect().y - containerOffsetY;
    setNotePosition(baseNoteBlock, baseTop);

    const yielder = createYielder(64);
    const promises = [];
    if (baseIndex > 0) {
        const update = async () => {
            // Layout blocks before base note
            let top = baseTop;
            for (let i = baseIndex - 1; i >= 0; i--) {
                const noteBlock = noteContainer.children[i] as HTMLElement;
                if (!noteBlock) {
                    // index out of bound
                    DocLog.warn(`update note position called with invalid index ${i}`);
                    return;
                }
                // The max. top this note can be at
                top -= noteBlock.clientHeight;
                // Get the line position so we can anchor the note to it
                const [sectionIndex, lineIndex] = getLineLocationFromElement(noteBlock);
                const lineElement = findLineByIndex(sectionIndex, lineIndex);
                if (lineElement) {
                    const lineTop =
                        lineElement.getBoundingClientRect().y - containerOffsetY;
                    top = Math.min(top, lineTop);
                }
                const changed = setNotePosition(noteBlock, top);
                if (!changed) {
                    // If the note is not changed, then notes before it doesn't need to change either
                    // (no longer true because of overlapping intervals may change)
                    break;
                }
                const didYield = await yielder();
                if (didYield) {
                    if (shouldCancel()) {
                        return;
                    }
                }
            }
        };
        promises.push(update());
    }

    if (baseIndex < noteContainer.children.length - 1) {
        const update = async () => {
            let top = baseTop + baseNoteBlock.clientHeight;
            for (let i = baseIndex + 1; i < noteContainer.children.length; i++) {
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
                const changed = setNotePosition(noteBlock, top);
                if (!changed) {
                    break;
                } 
                const didYield = await yielder();
                if (didYield) {
                    if (shouldCancel()) {
                        return;
                    }
                }
            }
        };
        promises.push(update());
    }

    // Run updates above and below concurrently
    await Promise.all(promises);

    DocLog.info("finished updating note positions");
};

/// Helper for setting the position of a note block
///
/// Return if the position is changed
const setNotePosition = (noteBlock: HTMLElement, top: number): boolean => {
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
    // if there's no next note, return the last note
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
    return Math.min(lo, noteBlocks.length - 1);
};
