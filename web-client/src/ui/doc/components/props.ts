import { DocDiagnostic, DocRichText, DocRichTextBlock } from "low/celerc";

/// Props for line components
export type DocLineProps = {
    /// Section index of the line, used for tracking line position
    sectionIndex: number;
    /// Line index within the section, used for tracking line position
    lineIndex: number;
    /// Color of the line
    lineColor: string;
    /// The text to display
    text: DocRichText;
    /// Url of the icon to display
    iconUrl?: string;
    /// Secondary text
    secondaryText: DocRichText;
    /// Counter properties
    counterText?: DocRichTextBlock;
    /// Counter type if any
    counterType?: string;
    /// Diagnostic messages
    diagnostics: DocDiagnostic[];
    /// If the line is a banner
    isBanner: boolean;
    /// If the line is a split. Will be false if disabled in the settings
    isSplit: boolean;
    /// The split type (display string). Should be present even if isSplit is false
    splitType: string | undefined;
};
