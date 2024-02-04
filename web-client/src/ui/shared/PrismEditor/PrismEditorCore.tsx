import { makeStyles, shorthands } from "@fluentui/react-components";
import ReactSimpleCodeEditor from "react-simple-code-editor";
import { highlight, languages } from "prismjs";

// load languages
/* eslint-disable import/no-internal-modules */
import "prismjs/components/prism-yaml";
/* eslint-enable import/no-internal-modules */

import { console, isInDarkMode } from "low/utils";

import { PrismEditorProps } from "./types";

console.info("loading prism editor");

function initStyles() {
    const dark = isInDarkMode();
    if (dark) {
        // eslint-disable-next-line import/no-internal-modules
        import("prismjs/themes/prism-okaidia.css");
    } else {
        // eslint-disable-next-line import/no-internal-modules
        import("prismjs/themes/prism.css");
    }
    return makeStyles({
        inner: {
            maxHeight: "300px",
            minHeight: "100px",
            overflowY: "auto",
            "& *": {
                minHeight: "100px",
                fontFamily: "monospace",
                fontSize: "12px",
            },
            "& textarea": {
                ...shorthands.outline("initial", "none"),
            },
        },
        outer: {
            ...shorthands.margin("10px", "0"),
            ...shorthands.border("1px", "solid", "#888"),
            ...shorthands.borderRadius("4px"),
            ...shorthands.padding("4px"),
            ":focus-within": {
                ...shorthands.outline("1px", "solid", dark ? "white" : "black"),
            },
            backgroundColor: dark ? "#111" : "#eee",
        },
    });
}
const useStyles = initStyles();

const PrismEditorCore: React.FC<PrismEditorProps> = ({
    language,
    value,
    setValue,
}) => {
    const styles = useStyles();
    return (
        <div className={styles.outer}>
            <div className={styles.inner}>
                <ReactSimpleCodeEditor
                    value={value}
                    onValueChange={(code) => setValue(code)}
                    highlight={(code) =>
                        highlight(code, languages[language], language)
                    }
                    padding={4}
                    onKeyDown={(e) => {
                        // prevent dialog from closing if the editor is
                        // inside a dialog
                        if (e.key === "Escape") {
                            e.preventDefault();
                            e.currentTarget.blur();
                        }
                    }}
                />
            </div>
        </div>
    );
};

export default PrismEditorCore;
