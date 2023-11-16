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
    const intervals = findAvailableIntervals();
    const noteContainer = document.getElementById(DocNoteContainerId);
    if (!noteContainer || noteContainer.children.length === 0) {
        // no notes to layout
        return;
    }

    const baseIndex = findBaseNoteIndex(noteContainer.children, baseLine);
    const containerOffsetY = getScrollContainerOffsetY(DocContentContainerId);

    // Layout the base note
    const baseNoteBlock = noteContainer.children[baseIndex] as HTMLElement;
    const [sectionIndex, lineIndex] = getLineLocationFromElement(baseNoteBlock);
    const lineElement = findLineByIndex(sectionIndex, lineIndex);
    if (!lineElement) {
        return;
    }
    const basePreferredTop = lineElement.getBoundingClientRect().y - containerOffsetY;
    const baseHeight = baseNoteBlock.getBoundingClientRect().height;
    const [baseTop, baseSplitIndex] = takeSpaceInIntervals(intervals, basePreferredTop, baseHeight);

    setNotePosition(baseNoteBlock, baseTop);

    const yielder = createYielder(64);
    const promises = [];
    if (baseIndex > 0) {
        const intervalsBefore = intervals.slice(0, baseSplitIndex);
        const update = async () => {
            // Layout blocks before base note
            for (let i = baseIndex - 1; i >= 0; i--) {
                const noteBlock = noteContainer.children[i] as HTMLElement;
                if (!noteBlock) {
                    return;
                }

                // preferably, anchor the note to the line it is at if possible:
                const preferredTop = getPreferredTop(noteBlock, containerOffsetY);
                if (!preferredTop) {
                    return;
                }

                const [top, splitIndex] = takeSpaceInIntervals(intervalsBefore, preferredTop, noteBlock.clientHeight);
                setNotePosition(noteBlock, top);
                const didYield = await yielder();
                if (didYield) {
                    if (shouldCancel()) {
                        return;
                    }
                }
                // Remove the spaces after the note, since the next note must be above this note
                intervalsBefore.splice(splitIndex, intervalsBefore.length - splitIndex);
            }
        };
        promises.push(update());
    }

    if (baseIndex < noteContainer.children.length - 1) {
        const update = async () => {
            const intervalsAfter = intervals.slice(baseSplitIndex);
            for (let i = baseIndex + 1; i < noteContainer.children.length; i++) {
                const noteBlock = noteContainer.children[i] as HTMLElement;
                if (!noteBlock) {
                    return;
                }
                // preferably, anchor the note to the line it is at if possible:
                const preferredTop = getPreferredTop(noteBlock, containerOffsetY);
                if (!preferredTop) {
                    return;
                }

                const [top, splitIndex] = takeSpaceInIntervals(intervalsAfter, preferredTop, noteBlock.clientHeight);
                setNotePosition(noteBlock, top);
                const didYield = await yielder();
                if (didYield) {
                    if (shouldCancel()) {
                        return;
                    }
                }
                // Remove the spaces before the note, since the next note must be below this note
                intervalsAfter.splice(0, splitIndex);
            }
        };
        promises.push(update());
    }

    // Run updates above and below concurrently
    await Promise.all(promises);

    DocLog.info("finished updating note positions");
};

/// Layout notes always anchored to the line element they are for
export const updateNotePositionsAnchored = async (
    // Callback to check if the update should be cancelled
    shouldCancel: () => boolean,
): Promise<void> => {
    DocLog.info("updating note positions (anchored)...");
    const noteContainer = document.getElementById(DocNoteContainerId);
    if (!noteContainer || noteContainer.children.length === 0) {
        // no notes to layout
        return;
    }
    const containerOffsetY = getScrollContainerOffsetY(DocContentContainerId);
    const yielder = createYielder(64);
    for (let i = 0; i < noteContainer.children.length; i++) {
        const noteBlock = noteContainer.children[i] as HTMLElement;
        if (!noteBlock) {
            return;
        }
        const top = getPreferredTop(noteBlock, containerOffsetY);
        if (!top) {
            return;
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

/// Get the preferred top position of the note block
///
/// The preferred top is where the line is at in the main panel
const getPreferredTop = (noteBlock: HTMLElement, containerOffsetY: number): number | undefined => {
    // preferably, anchor the note to the line it is at if possible:
    const [sectionIndex, lineIndex] = getLineLocationFromElement(noteBlock);
    const lineElement = findLineByIndex(sectionIndex, lineIndex);
    if (!lineElement) {
        DocLog.warn(`cannot find line when updating note position: ${sectionIndex}-${lineIndex}`);
        return undefined;
    }
    return lineElement.getBoundingClientRect().y - containerOffsetY;
}

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

/// Interval type. Even indices are start, odd indices are end
type Intervals = number[];

/// Find available intervals for the notes
const findAvailableIntervals = (): Intervals => {
    const rects: DOMRect[] = [];
    function add(e: Element) {
        rects.push(e.getBoundingClientRect());
    }
    document.querySelectorAll(".docsection-head").forEach(add);
    document.querySelectorAll(".docline-banner").forEach(add);

    const end = document.getElementById(DocContentContainerId);
    if(!end) {
        return [];
    }
    const endRect = end.getBoundingClientRect();

    // note: insertion sort is probably faster
    rects.sort((a, b) => a.y - b.y);

    const intervals = [0];
    rects.forEach((rect) => {
        const last = intervals.length - 1;
        const end = rect.y - endRect.y;
        const start = rect.y + rect.height - endRect.y;
        if (Math.abs(intervals[last] - end) < 1) {
            intervals[last] = start;
        } else {
            intervals.push(end);
            intervals.push(start);
        }
    });
    const last = intervals.length - 1;
    if (Math.abs(intervals[last] - endRect.height) < 1) {
        intervals.pop();
    } else {
        intervals.push(endRect.height);
    }
    
    return intervals;
}

/// Find an available space in the intervals starting from preferredTop
///
/// Edits the intervals array to remove the space taken.
///
/// Returns the top and the index of the split. i is always even, intervals[0..i] are the intervals
/// before the taken space and intervals[i..] are the intervals after the taken space. End indices are exclusive
///
/// If there are not enough space in the intervals, space at preferredTop will be taken anyway
const takeSpaceInIntervals = (intervals: Intervals, preferredTop: number, height: number): [number, number] => {
    const i = findInIntervals(preferredTop, intervals);
    const startIndex = i * 2;
    const endIndex = startIndex + 1;

    let top;
    let spliceStartI;
    const startFits = preferredTop >= intervals[startIndex];
    const endFits = preferredTop + height <= intervals[endIndex];
    if (startFits && endFits) {
        // fits, use preferred
        top = preferredTop;
        spliceStartI = startIndex;
    } else {
        const nextAvailableStart = findNextAvailableSpaceBelow(
            intervals, startFits?startIndex+2: startIndex, height);
        const previousAvailableEnd = findNextAvailableSpaceAbove(
            intervals, startFits?startIndex + 1:startIndex-1, height);
        if (nextAvailableStart === undefined) {
            if (previousAvailableEnd === undefined) {
                // no available space, use preferred
                top = preferredTop;
                spliceStartI = startIndex;
            } else {
                // use above
                top = intervals[previousAvailableEnd] - height;
                spliceStartI = previousAvailableEnd - 1;
            }
        } else {
            if (previousAvailableEnd === undefined) {
                // use below
                top = intervals[nextAvailableStart];
                spliceStartI = nextAvailableStart;
            } else {
                // both available space above and below, find the one with the least difference
                const diffAbove = preferredTop - (intervals[previousAvailableEnd] - height);
                const diffBelow = intervals[nextAvailableStart] - preferredTop;
                if( diffAbove > diffBelow) {
                    top = intervals[nextAvailableStart];
                    spliceStartI = nextAvailableStart;
                } else {
                    top = intervals[previousAvailableEnd] - height;
                    spliceStartI = previousAvailableEnd - 1;
                }
            }
        }
    }

    const returnTop = top;

    // take the space at top
    let remainingHeight = height;
    if (top > intervals[spliceStartI]) {
        if (top + height < intervals[spliceStartI + 1]) {
            intervals.splice(spliceStartI + 1, 0, top, top + height);
            return [returnTop, spliceStartI + 2];
        }
        // set the remaining space before top
        remainingHeight -= intervals[spliceStartI + 1] - top;
        const temp = intervals[spliceStartI + 1];
        intervals[spliceStartI + 1] = top;
        top = temp;
        spliceStartI += 2;
    }

    let spliceEndJ = spliceStartI+ 1;
    while(spliceEndJ < intervals.length && intervals[spliceEndJ] <= top + remainingHeight) {
        spliceEndJ += 2;
    }

    if (spliceEndJ >= intervals.length) {
        intervals.splice(spliceStartI, intervals.length - spliceStartI);
    } else {
        if (top + remainingHeight > intervals[spliceEndJ - 1]) {
            intervals[spliceEndJ - 1] = top + remainingHeight;
        }
        intervals.splice(spliceStartI, spliceEndJ - 1- spliceStartI);
    }

    return [returnTop, spliceStartI];

}

/// Find the index of the interval that contains y.
/// 
/// If no intervals contain y, return the index of the interval after y
///
/// For return value i, 2i is the start of the interval and 2i+1 is the end
const findInIntervals = (y: number, intervals: Intervals): number => {
    let lo = 0;
    let hi = intervals.length / 2 - 1;
    while (lo <= hi) {
        const mid = Math.floor((lo + hi) / 2);
        const start = intervals[mid * 2];
        const end = intervals[mid * 2 + 1];
        if (end < y) {
            lo = mid + 1;
        } else if (start > y){
            hi = mid - 1;
        } else {
            return mid;
        }
    }
    return lo;
}

/// Find the previous interval that is at least height tall, starting from previousEndIndex
///
/// Returns the index of the end of that interval, or undefined if no such interval exists
const findNextAvailableSpaceAbove = (intervals: Intervals, previousEndIndex: number, height: number): number | undefined => {
    for (let i = previousEndIndex; i > 0; i -= 2) {
        const start = intervals[i-1];
        const end = intervals[i];
        if (end - start >= height) {
            return i;
        }
    }
    return undefined;
}

/// Find the next interval that is at least height tall, starting from nextStartIndex
///
/// Returns the index of the start of that interval, or undefined if no such interval exists
const findNextAvailableSpaceBelow = (intervals: Intervals, nextStartIndex: number, height: number): number | undefined=> {
    // note: i is always even and intervals.length is always even, so i < intervals.length is fine even if we are accessing i+1
    for (let i = nextStartIndex; i < intervals.length; i += 2) {
        const start = intervals[i];
        const end = intervals[i+1];
        if (end - start >= height) {
            return i;
        }
    }
    return undefined;
}
