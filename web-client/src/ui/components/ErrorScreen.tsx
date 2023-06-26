//! The error screen component

import "./ErrorScreen.css";
import { Subtitle1, Text } from "@fluentui/react-components";

type ErrorProps = {
    /// The error message to display
    message: string;
}

export const ErrorScreen: React.FC<ErrorProps> = ({ message }) => {
    return (
        <div className="error-container">
            <img className="errro-logo" src={"/static/celer-red.svg"} />
            <div className="error-bar" >
                <Subtitle1>{message}</Subtitle1>
            </div>
        </div>
    );
};
