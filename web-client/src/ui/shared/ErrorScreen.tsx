//! The error screen component

import "./ErrorScreen.css";
import { Body1, Button, Subtitle1 } from "@fluentui/react-components";

type ErrorScreenProps = {
    /// The error message to display
    message: string;
};

export const ErrorScreen: React.FC<ErrorScreenProps> = ({ message }) => {
    return (
        <div className="error-container">
            <img className="error-logo" src={"/static/celer-red.svg"} />
            <div className="error-title">
                <Subtitle1>Oops, something went wrong</Subtitle1>
            </div>
            <div className="error-message">
                <Body1>{message}</Body1>
            </div>
            <Button appearance="primary">Copy logs</Button>
        </div>
    );
};
