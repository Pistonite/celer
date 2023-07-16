import "./HintScreen.css";
import { Caption1 } from "@fluentui/react-components";

type HintScreenProps = {
    /// The hint message to display
    message: string;
};

export const HintScreen: React.FC<HintScreenProps> = ({ message }) => {
    // <div className="hint-message">
    // </div>
    return (
        <div className="hint-container">
            <Caption1>{message}</Caption1>
        </div>
    );
};
