//! Logic for updating the width of banners upon updates

import { DocContainer, DocLog, getInjectedStyleTag } from "./utils";

export const SectionBannerWidthClass = "section-banner-width-injected";
export const BannerWidthClass = "banner-width-injected";
export const BannerTextWidthClass = "banner-text-width-injected";
export const BannerTextWithIconWidthClass =
    "banner-text-width-with-icon-injected";

export const updateBannerWidths = (): void => {
    const container = DocContainer.get();
    if (!container) {
        return;
    }
    const containerWidth = container.getBoundingClientRect().width;
    const bannerWidth = containerWidth - 64; // subtract the header
    const textWidth = bannerWidth - 4; // subtract the padding

    const styleTag = getInjectedStyleTag("banner-width");
    let style = `.${SectionBannerWidthClass}{width:${containerWidth}px !important;}`;
    style += `.${BannerWidthClass}{width:${bannerWidth}px !important;}`;
    style += `.${BannerTextWidthClass}{width:${textWidth}px !important;}`;
    style += `.${BannerTextWithIconWidthClass}{width:${
        textWidth - 50
    }px !important;}`;
    styleTag.innerHTML = style;
    DocLog.info("banner width css updated.");
};
