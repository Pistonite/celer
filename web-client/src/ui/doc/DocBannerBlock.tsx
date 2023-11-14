import { DocRichText } from "low/celerc";
import { Rich } from "./Rich";

export type DocBannerBlockProps = {
    /// Section index of the line, used for tracking line position
    sectionIndex: number;
    /// Line index within the section, used for tracking line position
    lineIndex: number;
    /// The note blocks to display
    content: DocRichText;
}

export const DocBannerBlock: React.FC<DocBannerBlockProps> = ({
    content
}) => {
    return (
        <div className="docbanner-container">
            <Rich size={500} content={content} />
        </div>
    );
}
