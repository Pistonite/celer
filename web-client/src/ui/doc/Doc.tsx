//! The doc component

import "./Doc.css";
import { documentSelector } from "data/store";
import { useSelector } from "react-redux";

import { LoadScreen } from "ui/shared";

import { DocSection } from "./DocSection";
import { DocRichText, DocTagMap, RichText } from "data/model";
import { DocLine } from "./DocLine";

/// Main doc viewer component
export const Doc: React.FC = () => {
    const { document } = useSelector(documentSelector);
    if (!document.loaded) {
        return <LoadScreen color="yellow" />;
    }
    const tagMap = document.project.tags;
    return (
        <div id="doc-container">
            {
                document.route.map(({ name, lines }, i) => (
                    <DocSection key={i} name={name}>
                        {
                            lines.map((line, j) => (
                                <DocLine
                                    key={j}
                                    selected={false} //TODO
                                    mode={"normal"} //TODO
                                    lineColor={line.lineColor}
                                    text={resolveTags(tagMap, line.text)}
                                    iconUrl={document.project.icons[line.icon]}
                                    secondaryText={resolveTags(tagMap, line.secondaryText)}
                                    counterText={line.counterText ? resolveTag(tagMap, line.counterText) : undefined}
                                />
                            ))
                        }
                    </DocSection>
                ))
            }
        </div>
    );
};

/// Helper function to resolve tag names to the tag definition
const resolveTags = (tagMap: DocTagMap, docRichTexts: DocRichText[]): RichText[] => {
    return docRichTexts.map(docRichText => resolveTag(tagMap, docRichText));
}

const resolveTag = (tagMap: DocTagMap, docRichText: DocRichText): RichText => {
    const { tag, text } = docRichText;
    if (!tag) {
        return { text };
    }

    const tagDef = tagMap[tag];
    if (!tagDef) {
        // Silently ignore unknown tag because compiler will add a warning (TODO: make sure you actually you taht)
        return { text };
    }
    return {
        text,
        tag: tagDef
    };
}
