import { DocNote, DocTagMap } from "low/compiler";

import { Rich } from "./Rich";
import { resolveTags } from "./utils";
/// Component for displaying note blocks
export type DocNoteBlockProps = {
    /// Section index of the line, used for tracking line position
    sectionIndex: number;
    /// Line index within the section, used for tracking line position
    lineIndex: number;
    /// The note blocks to display
    notes: DocNote[];
    /// Tag map used for resolving the tags
    tagMap: DocTagMap;
};

export const DocNoteBlock: React.FC<DocNoteBlockProps> = ({
    sectionIndex,
    lineIndex,
    notes,
    tagMap,
}) => {
    return (
        <div
            className="docnote-container"
            data-section={sectionIndex}
            data-line={lineIndex}
        >
            {notes.map((note, i) => {
                if (note.type === "text") {
                    return (
                        <div key={i} className="docnote-block">
                            <Rich
                                size={400}
                                content={resolveTags(tagMap, note.content)}
                            />
                        </div>
                    );
                }
                return (
                    <div key={i} className="docnote-block">
                        TODO not supported yet
                    </div>
                );
            })}
        </div>
    );
};
