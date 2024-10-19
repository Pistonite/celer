//! An error bar that displays preformatted error messages

import type { PropsWithChildren } from "react";
import {
    MessageBar,
    MessageBarBody,
    MessageBarTitle,
    makeStyles,
} from "@fluentui/react-components";

const useErrorBarStyles = makeStyles({
    root: {
        fontFamily: "monospace",
        whiteSpace: "pre-wrap",
    },
});

type ErrorBarProps = {
    title: string;
};

export const ErrorBar: React.FC<PropsWithChildren<ErrorBarProps>> = ({
    children,
    title,
}) => {
    if (!children) {
        return null;
    }
    return (
        <MessageBar intent="error">
            <MessageBarBody>
                <MessageBarTitle>{title}</MessageBarTitle>
                <FormattedError error={children} />
            </MessageBarBody>
        </MessageBar>
    );
};

export const FormattedError: React.FC<{ error: unknown }> = ({ error }) => {
    const styles = useErrorBarStyles();
    if (typeof error !== "string" || !error) {
        return null;
    }
    const lines = error.split("\n").map((line, i) => {
        if (!line) {
            return (
                <div className={styles.root} key={i}>
                    &nbsp;
                </div>
            );
        }
        return (
            <div className={styles.root} key={i}>
                {line || ""}
            </div>
        );
    });
    return <div style={{ marginTop: 4, marginBottom: 4 }}> {lines} </div>;
};
