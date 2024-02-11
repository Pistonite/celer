//! Logic for updating the width of banners upon updates

import { DOMStyleInject } from "low/utils";
import { DocLog } from "./utils";
import { DocContainerWidthVariable } from "./styles";
import { DocContainer } from "./dom";

const BannerWidthStyles = new DOMStyleInject("dynamic-banner-width");

export const updateBannerWidths = (): void => {
    const container = DocContainer.get();
    if (!container) {
        return;
    }
    const containerWidth = container.getBoundingClientRect().width;
    const style = `:root {${DocContainerWidthVariable.name}:${containerWidth}px;}`;
    BannerWidthStyles.setStyle(style);
    DocLog.info("banner width css updated.");
};
