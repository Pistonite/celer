import { DocEnd } from "./dom";
import { useDocStyles } from "./styles";

export const DocEndComp: React.FC = () => {
    const styles = useDocStyles();
    return (
        <div id={DocEnd.id} className={styles.docEnd}>
            There's nothing more to see past this point.
        </div>
    );
};
