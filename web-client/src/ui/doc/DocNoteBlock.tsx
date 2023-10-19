import { useRef } from "react";

import { DocNote, DocTag } from "low/celerc";

import { Rich } from "./Rich";
import { DocScrollId } from "./utils";

/// Class name for expanded note blocks
export const DocNoteExpandedClass = "docnote-container-expanded";

/// Component for displaying note blocks
export type DocNoteBlockProps = {
    /// Section index of the line, used for tracking line position
    sectionIndex: number;
    /// Line index within the section, used for tracking line position
    lineIndex: number;
    /// The note blocks to display
    notes: DocNote[];
    // /// Tag map used for resolving the tags
    // tagMap: Record<string, DocTag>;
};

export const DocNoteBlock: React.FC<DocNoteBlockProps> = ({
    sectionIndex,
    lineIndex,
    notes,
    //tagMap,
}) => {
    const ref = useRef<HTMLDivElement>(null);
    return (
        <div
            ref={ref}
            className="docnote-container"
            data-section={sectionIndex}
            data-line={lineIndex}
            onMouseEnter={(e) => {
                if (!ref.current) {
                    return;
                }
                const target = ref.current;
                const { width } = target.getBoundingClientRect();
                if (width <= 100) {
                    target.classList.add(DocNoteExpandedClass);
                    const docWidth = document.querySelector(`#${DocScrollId}`)
                        ?.clientWidth;
                    if (docWidth) {
                        target.style.width = `${docWidth}px`;
                    } else {
                        target.style.width = "100vw";
                    }
                }
                e.stopPropagation();
            }}
            onMouseLeave={(e) => {
                if (!ref.current) {
                    return;
                }
                const target = ref.current;
                target.classList.remove(DocNoteExpandedClass);
                target.style.width = "";
                e.stopPropagation();
            }}
        >
            {notes.map((note, i) => {
                if (note.type === "text") {
                    return (
                        <div key={i} className="docnote-block">
                            <Rich
                                size={400}
                                content={note.content}
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
