//! Logic for updating the width of banners upon updates

import { injectDOMStyle } from "low/utils";
import { DocLog } from "./utils";
import { DocContainer, DocContainerWidthVariable } from "./components";

export const updateBannerWidths = (): void => {
    const container = DocContainer.get();
    if (!container) {
        return;
    }
    const containerWidth = container.getBoundingClientRect().width;
    const style = `:root {${DocContainerWidthVariable.name}:${containerWidth}px;}`;
    injectDOMStyle("dynamic-banner-width", style);
    DocLog.info("banner width css updated.");
};
