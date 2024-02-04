//! Logic for updating the width of banners upon updates

import { DOMStyleInject } from "low/utils";
import { DocContainer, DocLog } from "./utils";

export const SectionBannerWidthClass = "section-banner-width-injected";
export const BannerWidthClass = "banner-width-injected";
export const BannerTextWidthClass = "banner-text-width-injected";
export const BannerTextWithIconWidthClass =
    "banner-text-width-with-icon-injected";

const BannerWidthStyles = new DOMStyleInject("banner-width");

export const updateBannerWidths = (): void => {
    const container = DocContainer.get();
    if (!container) {
        return;
    }
    const containerWidth = container.getBoundingClientRect().width;
    const bannerWidth = containerWidth - 64; // subtract the header
    const textWidth = bannerWidth - 4; // subtract the padding

    // const styleTag = getInjectedStyleTag("banner-width");
    let style = `.${SectionBannerWidthClass}{width:${containerWidth}px !important;}`;
    style += `.${BannerWidthClass}{width:${bannerWidth}px !important;}`;
    style += `.${BannerTextWidthClass}{width:${textWidth}px !important;}`;
    style += `.${BannerTextWithIconWidthClass}{width:${
        textWidth - 50
    }px !important;}`;
    BannerWidthStyles.setStyle(style);
    // styleTag.innerHTML = style;
    DocLog.info("banner width css updated.");
};
