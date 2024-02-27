//! Logic for updating the width of banners upon updates

import { injectDOMStyle, consoleDoc as console } from "low/utils";
import { DocContainer, DocContainerWidthVariable } from "./components";

export const updateBannerWidths = (): void => {
    const container = DocContainer.get();
    if (!container) {
        return;
    }
    const containerWidth = container.getBoundingClientRect().width;
    const style = `:root {${DocContainerWidthVariable.name}:${containerWidth}px;}`;
    injectDOMStyle("dynamic-banner-width", style);
    console.info("banner width css updated.");
};
