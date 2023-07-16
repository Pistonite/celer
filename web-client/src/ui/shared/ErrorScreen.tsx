//! The error screen component

import "./ErrorScreen.css";
import { Subtitle1 } from "@fluentui/react-components";

type ErrorScreenProps = {
    /// The error message to display
    message: string;
};

export const ErrorScreen: React.FC<ErrorScreenProps> = ({ message }) => {
    return (
        <div className="error-container">
            <img className="errro-logo" src={"/static/celer-red.svg"} />
            <div className="error-bar">
                <Subtitle1>{message}</Subtitle1>
            </div>
        </div>
    );
};
