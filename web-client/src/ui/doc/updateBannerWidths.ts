//! Logic for updating the width of banners upon updates

import { DocContainerId, DocLog, getInjectedStyleTag } from "./utils";

export const BannerWidthClass = "banner-width-injected";
export const BannerTextWidthClass = "banner-text-width-injected";

export const updateBannerWidths = (): void => {
    const container = document.getElementById(DocContainerId);
    if (!container) {
        return;
    }
    const containerWidth = container.getBoundingClientRect().width;
    const bannerWidth = containerWidth - 64; // subtract the header
    const textWidth = bannerWidth - 4; // subtract the padding

    const styleTag = getInjectedStyleTag("banner-width");
    styleTag.innerText=`.${BannerWidthClass}{width:${bannerWidth}px !important;}.${BannerTextWidthClass}{width:${textWidth}px !important;}`
    DocLog.info("banner width css updated.");

}
