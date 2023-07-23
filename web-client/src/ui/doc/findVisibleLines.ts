import {
    DocContainerId,
    getScrollContainerOffsetY,
    getScrollView,
    getLineScrollView,
    DocLineContainerClass,
} from "./utils";

/// Find all the lines that are visible in the document container
///
/// Return a list of lines from top to bottom.
/// Lines that are partially visible are included.
///
/// Return maybe empty if there are exceptions, such as document container not found
export const findVisibleLines = (): HTMLElement[] => {
    const scrollView = getScrollView();
    if (!scrollView) {
        return [];
    }
    const { scrollTop, scrollBottom } = scrollView;

    const visibleLineElements: HTMLElement[] = [];
    const containerElement = document.getElementById(DocContainerId);
    if (!containerElement) {
        return [];
    }
    const containerOffsetY = getScrollContainerOffsetY();
    // get all lines
    // This is always in the right order because querySelectorAll uses pre-order traversal
    // Therefore we can optimize the search
    const lineElements = containerElement.querySelectorAll<HTMLElement>(
        `.${DocLineContainerClass}`,
    );
    // binary search to find first visible line
    let lo = 0;
    let hi = lineElements.length - 1;
    while (lo <= hi) {
        const mid = Math.floor((lo + hi) / 2);
        const { scrollBottom: lineBottom } = getLineScrollView(
            lineElements[mid],
            containerOffsetY,
        );
        if (lineBottom < scrollTop) {
            // line is above and not visible
            lo = mid + 1;
        } else {
            // Line maybe visible, but we need to find the first one
            hi = mid - 1;
        }
    }
    for (let i = lo; i < lineElements.length; i++) {
        const lineElement = lineElements[i];
        const { scrollTop: lineTop } = getLineScrollView(
            lineElements[i],
            containerOffsetY,
        );
        if (lineTop > scrollBottom) {
            // line is below and not visible
            break;
        }
        visibleLineElements.push(lineElement);
    }

    return visibleLineElements;
};
