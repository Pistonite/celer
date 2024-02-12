import { ExecDoc } from "low/celerc";

import { DocNoteBlock, DocNoteBlockProps } from "./DocNoteBlock";
import { DocNoteContainer } from "./dom";
import { useDocStyles } from "./styles";

export type DocNotePanelProps = {
    document: ExecDoc;
};

export const DocNotePanel: React.FC<DocNotePanelProps> = ({ document }) => {
    const styles = useDocStyles();
    const flatNotes = document.route.reduce(
        (acc: DocNoteBlockProps[], section, i) => {
            section.lines.forEach((line, j) => {
                if (line.notes.length > 0) {
                    acc.push({
                        sectionIndex: i,
                        lineIndex: j,
                        notes: line.notes,
                    });
                }
            });
            return acc;
        },
        [],
    );
    return (
        <div id={DocNoteContainer.id} className={styles.docNoteContainer}>
            {flatNotes.map((props, i) => (
                <DocNoteBlock key={i} {...props} />
            ))}
        </div>
    );
};
