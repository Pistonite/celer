//! Controller of the doc that handles doc view update, store updates, etc.
//!
//! The controll works directly with the DOM.
//! We need this because the react component is only used to render the doc, and does not
//! rerender when the view updates.

import reduxWatch from "redux-watch";

import {
    AppStore,
    documentSelector,
    settingsSelector,
    viewActions,
    viewSelector,
} from "core/store";
import { Debouncer } from "low/utils";
import { GameCoord } from "low/celerc";

import {
    DocContainerId,
    DocLog,
    DocScrollId,
    findLineByIndex,
    findSectionByIndex,
    getLineLocationFromElement,
    getLineScrollView,
    getScrollContainerOffsetY,
    getScrollView,
    updateDocTagsStyle,
} from "./utils";
import { findVisibleLines } from "./findVisibleLines";
import { updateNotePositions } from "./updateNotePositions";
import { updateBannerWidths } from "./updateBannerWidths";

/// Storing map state as window global because HMR will cause the map to be recreated
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

/// Controller class
///
/// The document DOM can call the controller to update the view.
export class DocController {
    /// Reference to the app store
    private store: AppStore;
    /// The update handle
    private updateHandle: number | null = null;
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
            watchStore((newState, oldState) => {
                const newDoc = documentSelector(newState);
                const newDocSerial = newDoc.serial;
                const oldDocSerial = documentSelector(oldState).serial;
                const newView = viewSelector(newState);
                const oldView = viewSelector(oldState);
                if (newDocSerial !== oldDocSerial) {
                    // Also update the rich text styles
                    if (newDoc.document) {
                        // If document changed, reset the view
                        // TODO: can load from local storage to pick up from where you left
                        // store.dispatch(
                        //     viewActions.setDocLocation({ section: 0, line: 0 }),
                        // );
                        // also update the current line and note positions, and trigger a scroll update
                        // to layout the initial view
                        setTimeout(() => {
                            this.updateViewAsync(true);
                        }, 0);
                        updateDocTagsStyle(newDoc.document.project.tags);
                    }
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
                setTimeout(() => {
                    this.updateViewAsync(false);
                }, 0);
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

    /// Update after scrolling
    public onScroll() {
        // if the current line is not visible, re-get the current line
        this.scrollUpdateDebouncer.dispatch();
    }

    /// Update the view after scrolling
    private onScrollUpdate() {
        updateBannerWidths();
        const view = viewSelector(this.store.getState());
        const scrollView = getScrollView();
        if (!scrollView) {
            return;
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
            const containerOffsetY = getScrollContainerOffsetY(DocContainerId);
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
                return;
            }
            // make center line current
            const centerLine =
                visibleLines[Math.floor(visibleLines.length / 2)];
            const [section, line] = getLineLocationFromElement(centerLine);
            this.store.dispatch(viewActions.setDocLocation({ section, line }));
            updateNotePositions(centerLine);
        } else {
            // Update notes based on current line
            updateNotePositions(currentLine as HTMLElement);
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
    }

    private removeCurrentLineIndicator(section: number, line: number) {
        const lineElement = findLineByIndex(section, line);
        if (lineElement) {
            lineElement.classList.remove(DocCurrentLineClass);
        }
    }

    /// Update wrapper that retries until the view is updated
    private updateViewAsync(forceScrollUpdate: boolean) {
        if (this.updateHandle) {
            // already trying
            return;
        }
        if (this.onViewUpdate(forceScrollUpdate)) {
            return;
        }
        DocLog.warn("Fail to update document view. Will retry in 1s");
        this.updateHandle = window.setTimeout(() => {
            this.updateHandle = null;
            this.updateViewAsync(forceScrollUpdate);
        }, 1000);
    }

    /// Update after store change
    ///
    /// For example, when current line position changes.
    /// If forceScrollUpdate, will also call scroll update even if scroll didn't change.
    private onViewUpdate(forceScrollUpdate: boolean): boolean {
        const newView = viewSelector(this.store.getState());
        // update current line indicator
        let newCurrentLine = findLineByIndex(
            newView.currentSection,
            newView.currentLine,
        );
        if (newCurrentLine) {
            newCurrentLine.classList.add(DocCurrentLineClass);
        }
        if (!newCurrentLine) {
            // Try to scroll to the section instead if the line is not found
            newCurrentLine = findSectionByIndex(newView.currentSection);
            if (!newCurrentLine) {
                DocLog.warn(
                    `cannot find current line: section=${newView.currentSection}, line=${newView.currentLine}`,
                );
                return false;
            }
        }

        const scrollView = getScrollView();
        if (!scrollView) {
            return false;
        }

        // Scroll the current line to visible
        const { scrollTop, scrollBottom } = scrollView;
        const containerOffsetY = getScrollContainerOffsetY(DocContainerId);
        const { scrollTop: currentLineTop, scrollBottom: currentLineBottom } =
            getLineScrollView(newCurrentLine, containerOffsetY);

        const scrollViewHeight = scrollBottom - scrollTop;
        const currentLineHeight = currentLineBottom - currentLineTop;
        const scrollEdgeSize = getScrollEdgeSize();
        // There are 3 modes:
        // 1. current height < scroll view height - edge size: scroll edge if needed
        // 2. current height < scroll view height: scroll to middle
        // 3. current height >= scroll view height: scroll to top
        if (currentLineHeight < scrollViewHeight - scrollEdgeSize) {
            if (currentLineTop < scrollTop + scrollEdgeSize) {
                const newScrollTop = currentLineTop - scrollEdgeSize;
                setScrollView(newScrollTop);
            } else if (
                currentLineTop + currentLineHeight >
                scrollBottom - scrollEdgeSize
            ) {
                const newScrollTop =
                    currentLineBottom + scrollEdgeSize - scrollViewHeight;
                setScrollView(newScrollTop);
            }
        } else if (currentLineHeight < scrollViewHeight) {
            const edge = (scrollViewHeight - currentLineHeight) / 2;
            const newScrollTop = currentLineTop - edge;
            setScrollView(newScrollTop);
        } else {
            setScrollView(currentLineTop);
        }

        if (forceScrollUpdate) {
            this.scrollUpdateDebouncer.dispatch();
        }

        return true;
    }
}

/// Set the scroll
const setScrollView = (scrollTop: number) => {
    const scrollElement = document.getElementById(DocScrollId);
    if (!scrollElement) {
        return;
    }
    scrollElement.scrollTop = scrollTop;
};

/// Get the scroll edge size
///
/// This is calculated based on the container height
const getScrollEdgeSize = (): number => {
    const scrollElement = document.getElementById(DocScrollId);
    if (!scrollElement) {
        return 0;
    }
    // 20% of the container height
    return scrollElement.getBoundingClientRect().height * 0.2;
};
