import { ExecDoc } from "low/celerc"
import { DocMainContainer } from "./dom";
import { useDocStyles } from "./styles";
import { DocSection } from "./DocSection";

export type DocMainPanelProps = {
    document: ExecDoc;
    splitTypes: Set<string>;
}

export const DocMainPanel: React.FC<DocMainPanelProps> = ({ document, splitTypes }) => {
    const styles = useDocStyles();
    return (
        <div id={DocMainContainer.id} className={styles.docMainContainer} >
            {document.route.map((_, i) => (
                <DocSection 
                    index={i} 
                    key={i}
                    document={document} 
                    splitTypes={splitTypes} 
                />
            ))}
        </div>
    );
};
