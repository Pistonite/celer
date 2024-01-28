//! Controller of the doc that handles doc view update, store updates, etc.
//!
//! The controll works directly with the DOM.
//! We need this because the react component is only used to render the doc, and does not
//! rerender when the view updates.

import reduxWatch from "redux-watch";
import isEqual from "is-equal";

import {
    AppStore,
    documentSelector,
    settingsSelector,
    viewActions,
    viewSelector,
} from "core/store";
import { Debouncer, sleep } from "low/utils";
import { GameCoord } from "low/celerc";

import {
    DocContainer,
    DocLog,
    DocScroll,
    findLineByIndex,
    findNoteByIndex,
    findSectionByIndex,
    getLineLocationFromElement,
    getLineScrollView,
    getScrollContainerOffsetY,
    getScrollView,
    updateDocTagsStyle,
} from "./utils";
import { findVisibleLines } from "./findVisibleLines";
import {
    updateNotePositions,
    updateNotePositionsAnchored,
} from "./updateNotePositions";
import { updateBannerWidths } from "./updateBannerWidths";

/// Storing doc state as window global because HMR will cause the doc to be recreated
declare global {
    interface Window {
        __theDocController: DocController | null;
    }
}

/// Class for the current line indicator
export const DocCurrentLineClass = "doc-current-line";

DocLog.info("loading doc module");

/// Create the doc controller singleton
export const initDocController = (store: AppStore): DocController => {
    if (window.__theDocController) {
        window.__theDocController.delete();
    }

    DocLog.info("creating doc controller");

    const controller = new DocController(store);
    window.__theDocController = controller;

    return controller;
};

let nextNoteZIndex = 100;

/// Controller class
///
/// The document DOM can call the controller to update the view.
export class DocController {
    /// Reference to the app store
    private store: AppStore;

    /// The current update event id. Used for canceling previous async updates
    private currentUpdateEventId = 0;
    /// Debouncer for updating the view
    private scrollUpdateDebouncer: Debouncer;
    /// Clean up function
    private cleanup: () => void;

    constructor(store: AppStore) {
        this.store = store;
        this.scrollUpdateDebouncer = new Debouncer(200, () => {
            this.onScrollUpdate();
        });

        // Subscribe to store updates
        const watchStore = reduxWatch(store.getState);
        const unwatchStore = store.subscribe(
            watchStore(async (newState, oldState) => {
                const newDoc = documentSelector(newState);
                const newDocSerial = newDoc.serial;
                const oldDocSerial = documentSelector(oldState).serial;
                const newView = viewSelector(newState);
                const oldView = viewSelector(oldState);
                const newSettings = settingsSelector(newState);
                const oldSettings = settingsSelector(oldState);

                let needFullUpdate = false;
                if (newDocSerial !== oldDocSerial) {
                    // Document update
                    if (newDoc.document) {
                        updateDocTagsStyle(newDoc.document.project.tags);
                        needFullUpdate = true;
                    }
                }
                if (
                    !newView.isEditingLayout &&
                    newView.isEditingLayout !== oldView.isEditingLayout
                ) {
                    // layout has changed
                    needFullUpdate = true;
                } else if (
                    !newView.isResizingWindow &&
                    newView.isResizingWindow !== oldView.isResizingWindow
                ) {
                    // window is resized
                    needFullUpdate = true;
                } else if (!isEqual(newSettings, oldSettings)) {
                    // settings has changed
                    needFullUpdate = true;
                }
                if (needFullUpdate) {
                    // Make sure UI finishes updating
                    await sleep(0);
                    await this.onFullUpdate();
                    return;
                }
                if (
                    newView.currentSection === oldView.currentSection &&
                    newView.currentLine === oldView.currentLine
                ) {
                    // position didn't change
                    return;
                }
                this.removeCurrentLineIndicator(
                    oldView.currentSection,
                    oldView.currentLine,
                );
                await this.onLocationUpdate();
            }),
        );

        this.cleanup = () => {
            unwatchStore();
        };
    }

    public delete() {
        DocLog.info("deleting doc controller");
        this.cleanup();
    }

    /// Check if there is a newer update event and the current event should be cancelled.
    ///
    /// This should be checked after each async operation in an update
    private isEventObsolete(eventId: number): boolean {
        return eventId !== this.currentUpdateEventId;
    }

    /// Completely update the document view
    ///
    /// Triggered after layout or document change
    private async onFullUpdate() {
        DocLog.info("fully updating document view...");
        const eventId = ++this.currentUpdateEventId;
        updateBannerWidths();
        await sleep(0);
        if (this.isEventObsolete(eventId)) {
            return;
        }
        const scrollUpdated = await this.onLocationUpdateInternal(eventId);
        if (this.isEventObsolete(eventId)) {
            return;
        }
        if (!scrollUpdated) {
            // for full update, if the scroll wasn't updated, manually updated it
            await this.onScrollUpdateInternal(eventId);
        }
    }

    /// Called when the scroll changes, which handles the event through a debouncer
    public onScroll() {
        this.scrollUpdateDebouncer.dispatch();
    }

    private onScrollUpdate() {
        return this.onScrollUpdateInternal(++this.currentUpdateEventId);
    }

    private onLocationUpdate() {
        return this.onLocationUpdateInternal(++this.currentUpdateEventId);
    }

    /// Handle scroll change
    ///
    /// This updates the current line if it's no longer visible,
    /// and also updates the map view if needed.
    ///
    /// Returns if current line was updated
    private async onScrollUpdateInternal(_eventId: number): Promise<boolean> {
        DocLog.info("updating document view after scroll...");
        const view = viewSelector(this.store.getState());
        const scrollView = getScrollView();
        if (!scrollView) {
            return false;
        }

        // see if we need to update the current line
        let needUpdateCurrentLine = false;
        const currentLine = findLineByIndex(
            view.currentSection,
            view.currentLine,
        );
        if (!currentLine) {
            needUpdateCurrentLine = true;
        } else {
            const { scrollTop, scrollBottom } = scrollView;
            const containerOffsetY = getScrollContainerOffsetY(DocContainer);
            const {
                scrollTop: currentLineTop,
                scrollBottom: currentLineBottom,
            } = getLineScrollView(currentLine, containerOffsetY);
            needUpdateCurrentLine =
                currentLineTop < scrollTop || currentLineBottom > scrollBottom;
        }

        // don't know if we need to find the visible lines yet, so leave this undefined
        let visibleLines: HTMLElement[] | undefined = undefined;

        if (needUpdateCurrentLine) {
            // current line is not visible
            visibleLines = findVisibleLines();
            if (visibleLines.length === 0) {
                DocLog.warn("cannot find any visible lines");
                return false;
            }
            // make center line current
            const centerLine =
                visibleLines[Math.floor(visibleLines.length / 2)];
            const [section, line] = getLineLocationFromElement(centerLine);
            DocLog.info(
                `current line not visible, updating to ${section}-${line}...`,
            );
            this.store.dispatch(viewActions.setDocLocation({ section, line }));
        }

        const { syncMapToDoc } = settingsSelector(this.store.getState());
        const { document } = documentSelector(this.store.getState());
        if (syncMapToDoc && document) {
            if (!visibleLines) {
                visibleLines = findVisibleLines();
            }
            const coords = visibleLines
                .flatMap((line) => {
                    const [sectionIndex, lineIndex] =
                        getLineLocationFromElement(line);
                    return document.route[sectionIndex]?.lines[lineIndex]
                        ?.mapCoords;
                })
                .filter(Boolean) as GameCoord[];
            if (coords.length > 0) {
                this.store.dispatch(viewActions.setMapView(coords));
            }
        }

        return needUpdateCurrentLine;
    }

    private removeCurrentLineIndicator(section: number, line: number) {
        const lineElement = findLineByIndex(section, line);
        if (lineElement) {
            lineElement.classList.remove(DocCurrentLineClass);
        }
        const noteElement = findNoteByIndex(section, line);
        if (noteElement) {
            noteElement.classList.remove(DocCurrentLineClass);
        }
    }

    /// Update after current line change
    ///
    /// This also updates the note positions.
    /// Returns if the scroll was updated
    private async onLocationUpdateInternal(eventId: number): Promise<boolean> {
        const newView = viewSelector(this.store.getState());
        DocLog.info(
            `updating document view to ${newView.currentSection}-${newView.currentLine}...`,
        );

        // find the current line element and update current line indicator
        let newCurrentLine: HTMLElement | undefined = undefined;
        let retryCount = 0;
        const maxRetryCount = 10;
        while (!newCurrentLine) {
            newCurrentLine = findLineByIndex(
                newView.currentSection,
                newView.currentLine,
            );
            if (newCurrentLine) {
                newCurrentLine.classList.add(DocCurrentLineClass);
            } else {
                // Try to scroll to the section instead if the line is not found
                newCurrentLine = findSectionByIndex(newView.currentSection);
                if (!newCurrentLine) {
                    if (retryCount < maxRetryCount) {
                        DocLog.warn(
                            `cannot find current section: section=${newView.currentSection}. Will retry in 1s.`,
                        );
                    } else if (retryCount === maxRetryCount) {
                        DocLog.warn(
                            `cannot find current line after too many retries. Further warnings will be suppressed.`,
                        );
                    }
                    await sleep(1000);
                    if (this.isEventObsolete(eventId)) {
                        DocLog.info("canceling previous update");
                        return false;
                    }
                    retryCount++;
                }
            }
        }
        const newCurrentNote = findNoteByIndex(
            newView.currentSection,
            newView.currentLine,
        );
        if (newCurrentNote) {
            newCurrentNote.classList.add(DocCurrentLineClass);
            newCurrentNote.style.zIndex = `${++nextNoteZIndex}`;
        }

        const scrollView = getScrollView();
        if (!scrollView) {
            return false;
        }

        // Scroll the current line to visible
        const { scrollTop, scrollBottom } = scrollView;
        const containerOffsetY = getScrollContainerOffsetY(DocContainer);
        const { scrollTop: currentLineTop, scrollBottom: currentLineBottom } =
            getLineScrollView(newCurrentLine, containerOffsetY);

        const scrollViewHeight = scrollBottom - scrollTop;
        const currentLineHeight = currentLineBottom - currentLineTop;
        const scrollEdgeSize = getScrollEdgeSize();

        // There are 3 modes:
        // 1. current height < scroll view height - edge size: scroll edge if needed
        // 2. current height < scroll view height: scroll to middle
        // 3. current height >= scroll view height: scroll to top

        let scrollUpdated = false;
        if (currentLineHeight < scrollViewHeight - scrollEdgeSize) {
            if (currentLineTop < scrollTop + scrollEdgeSize) {
                const newScrollTop = currentLineTop - scrollEdgeSize;
                scrollUpdated = setScrollView(newScrollTop);
            } else if (
                currentLineTop + currentLineHeight >
                scrollBottom - scrollEdgeSize
            ) {
                const newScrollTop =
                    currentLineBottom + scrollEdgeSize - scrollViewHeight;
                scrollUpdated = setScrollView(newScrollTop);
            }
        } else if (currentLineHeight < scrollViewHeight) {
            const edge = (scrollViewHeight - currentLineHeight) / 2;
            const newScrollTop = currentLineTop - edge;
            scrollUpdated = setScrollView(newScrollTop);
        } else {
            scrollUpdated = setScrollView(currentLineTop);
        }

        const shouldCancel = () => {
            return this.isEventObsolete(eventId);
        };

        const { forceAnchorNotes } = settingsSelector(this.store.getState());
        if (forceAnchorNotes) {
            await updateNotePositionsAnchored(shouldCancel);
        } else {
            await updateNotePositions(newCurrentLine, shouldCancel);
        }
        return scrollUpdated;
    }
}

/// Set the scroll
const setScrollView = (scrollTop: number): boolean => {
    const scrollElement = DocScroll.get();
    if (!scrollElement) {
        return false;
    }
    if (scrollElement.scrollTop !== scrollTop) {
        scrollElement.scrollTop = scrollTop;
        return true;
    }
    return false;
};

/// Get the scroll edge size
///
/// This is calculated based on the container height
const getScrollEdgeSize = (): number => {
    const scrollElement = DocScroll.get();
    if (!scrollElement) {
        return 0;
    }
    // 20% of the container height
    return scrollElement.getBoundingClientRect().height * 0.2;
};
