import { useRef } from "react";

import { DocNote } from "low/celerc";
import { smartMergeClasses } from "low/utils";

import { DocNoteBlockClass, DocNoteContainerClass, DocNoteContainerExpandedClass, DocScroll } from "./dom";
import { Rich } from "./Rich";
import { useDocStyles } from "./styles";

/// Component for displaying note blocks
export type DocNoteBlockProps = {
    /// Section index of the line, used for tracking line position
    sectionIndex: number;
    /// Line index within the section, used for tracking line position
    lineIndex: number;
    /// The note blocks to display
    notes: DocNote[];
};

export const DocNoteBlock: React.FC<DocNoteBlockProps> = ({
    sectionIndex,
    lineIndex,
    notes,
}) => {
    const ref = useRef<HTMLDivElement>(null);

    const styles = useDocStyles();
    const noteBlockClass = smartMergeClasses(styles, DocNoteBlockClass);

    return (
        <div
            ref={ref}
            className={smartMergeClasses(styles, DocNoteContainerClass)}
            data-section={sectionIndex}
            data-line={lineIndex}
            onMouseEnter={(e) => {
                if (!ref.current) {
                    return;
                }
                const target = ref.current;
                const { width } = target.getBoundingClientRect();
                if (width <= 100) {
                    DocNoteContainerExpandedClass.addTo(target);
                    const docWidth = DocScroll.get()?.clientWidth;
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
                DocNoteContainerExpandedClass.removeFrom(target);
                target.style.width = "";
                e.stopPropagation();
            }}
        >
            {notes.map((note, i) => {
                if (note.type === "text") {
                    return (
                        <div key={i} className={noteBlockClass}>
                            <Rich size={400} content={note.content} />
                        </div>
                    );
                }
                return (
                    <div key={i} className={noteBlockClass}>
                        TODO not supported yet
                    </div>
                );
            })}
        </div>
    );
};
