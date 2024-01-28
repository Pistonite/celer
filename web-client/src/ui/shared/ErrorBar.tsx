//! An error bar that displays preformatted error messages

import { PropsWithChildren } from "react";
import {
    MessageBar,
    MessageBarBody,
    MessageBarTitle,
} from "@fluentui/react-components";
import { DOMClass } from "low/utils";

const ErrorBarClass = new DOMClass("error-bar");
ErrorBarClass.style({
    "font-family": "monospace",
    "white-space": "pre-wrap",
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
    if (typeof error !== "string" || !error) {
        return null;
    }
    const lines = error.split("\n").map((line, i) => {
        if (!line) {
            return (
                <div className={ErrorBarClass.className} key={i}>
                    &nbsp;
                </div>
            );
        }
        return (
            <div className={ErrorBarClass.className} key={i}>
                {line || ""}
            </div>
        );
    });
    return <div style={{ marginTop: 4, marginBottom: 4 }}> {lines} </div>;
};
