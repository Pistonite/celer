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
    // Callback to check if the update should be cancelled
    shouldCancel: () => boolean,
): Promise<void> => {
    DocLog.info("updating note positions...");
    const avoidElements = findAvoidElements();
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
    let baseTop = lineElement.getBoundingClientRect().y - containerOffsetY;
    const baseHeight = baseNoteBlock.getBoundingClientRect().height;
    const baseAvoidIndex = findIndexAfter(baseTop+containerOffsetY, avoidElements);
    if (overlaps(baseTop + containerOffsetY, baseTop + baseHeight + containerOffsetY, avoidElements[baseAvoidIndex])) {
        // bottom of the note overlaps with the next element
        // push it up until the bottom does not overlap
        let i = baseAvoidIndex;
        let tempBottom = avoidElements[i].y;
        i--;
        while (overlaps(tempBottom - baseHeight, tempBottom, avoidElements[i])) {
            tempBottom = avoidElements[i].y;
            i--;
        }
        baseTop = tempBottom - baseHeight - containerOffsetY;
    } else if (overlaps(baseTop+containerOffsetY, baseTop+baseHeight+containerOffsetY, avoidElements[baseAvoidIndex-1])) {
        // top of the note is overlapping
        // push it down until the top does not overlap
        let i = baseAvoidIndex-1;
        const rect = avoidElements[i];
        let tempTop = rect.y + rect.height;
        i++;
        while (overlaps(tempTop, tempTop + baseHeight, avoidElements[i])) {
            const rect = avoidElements[i];
            tempTop = rect.y + rect.height;
            i++;
        }
        baseTop = tempTop - containerOffsetY;
    }
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

                // calculate the max. top this note can be at
                // to avoid overlapping with the previous note
                // avoid elements above the note at the max top
                top = findTop(
                    top - noteBlock.clientHeight, 
                    noteBlock.clientHeight, 
                    containerOffsetY, 
                    avoidElements, 
                    -1);

                // preferably, anchor the note to the line it is at if possible:
                const [sectionIndex, lineIndex] = getLineLocationFromElement(noteBlock);
                const lineElement = findLineByIndex(sectionIndex, lineIndex);
                if (lineElement) {
                    const lineTopWithOffset = lineElement.getBoundingClientRect().y;
                    const preferredTop = findTop(
                        lineTopWithOffset-containerOffsetY, 
                        noteBlock.clientHeight, 
                        containerOffsetY, 
                        avoidElements, 
                        -1);
                    top = Math.min(top, preferredTop);
                }
                setNotePosition(noteBlock, top);
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

                // calculate the min. top this note can be at
                // to avoid overlapping with the previous note
                // avoid elements below the note at the min top
                top = findTop(
                    top, 
                    noteBlock.clientHeight, 
                    containerOffsetY, 
                    avoidElements, 
                    1);

                // find the preferred top for the note
                const [sectionIndex, lineIndex] = getLineLocationFromElement(noteBlock);
                const lineElement = findLineByIndex(sectionIndex, lineIndex);
                if (lineElement) {
                    const lineTopWithoutOffset =
                        lineElement.getBoundingClientRect().y;
                    const preferredTop = findTop(
                        lineTopWithoutOffset-containerOffsetY,
                        noteBlock.clientHeight,
                        containerOffsetY,
                        avoidElements,
                        1);
                    top = Math.max(top, preferredTop);
                }

                setNotePosition(noteBlock, top);
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

const findAvoidElements = (): DOMRect[] => {
    // note: insertion sort is probably faster
    const output: DOMRect[] = [];
    function add(e: Element) {
        output.push(e.getBoundingClientRect());
    }
    document.querySelectorAll(".docsection-head").forEach(add);
    document.querySelectorAll(".docline-banner").forEach(add);
    document.querySelectorAll(".docline-diagnostic").forEach(add);

    output.sort((a, b) => a.y - b.y);
    return output;
}

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

/// Find the index of the element in the array that is just after the given y
const findIndexAfter = (y: number, elements: DOMRect[]): number => {
    let lo = 0;
    let hi = elements.length - 1;
    while (lo <= hi) {
        const mid = Math.floor((lo + hi) / 2);
        const element = elements[mid];
        if (element.y < y) {
            lo = mid + 1;
        } else {
            hi = mid - 1;
        }
    }
    return lo;
}

/// Find the top position for the note with the current top and height while avoiding elements in the direction (1 = down, -1 = up)
const findTop = (noteTop: number, noteHeight: number, containerOffsetY: number, avoidElements: DOMRect[], direction: number): number => {
    let top = noteTop + containerOffsetY;
    const fromI = findIndexAfter(top, avoidElements);
    let i = fromI;
    if (direction > 0) {
        i--;
    }
    // check the first 2
    if (overlaps(top, top + noteHeight, avoidElements[i])) {
        top = avoidElements[i].y;
        if (direction > 0) {
            top += avoidElements[i].height;
        } else {
            top -= noteHeight;
        }
        i += direction;
    }
    if (overlaps(top, top + noteHeight, avoidElements[i])) {
        top = avoidElements[i].y;
        if (direction > 0) {
            top += avoidElements[i].height;
        } else {
            top -= noteHeight;
        }
        i += direction;
    }
    while (overlaps(top, top + noteHeight, avoidElements[i])) {
        top = avoidElements[i].y;
        if (direction > 0) {
            top += avoidElements[i].height;
        } else {
            top -= noteHeight;
        }
        i += direction;
    }
    return top;
}

/// Returns if the top to bottom y range overlaps with the other element
const overlaps = (top: number, bottom: number, rect: DOMRect | undefined): boolean => {
    if (!rect) {
        return false;
    }
    return top <= rect.y + rect.height && bottom >= rect.y;
}
