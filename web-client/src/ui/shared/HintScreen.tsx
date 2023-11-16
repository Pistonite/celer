import { PropsWithChildren } from "react";
import "./HintScreen.css";
import { Caption1 } from "@fluentui/react-components";

export const HintScreen: React.FC<PropsWithChildren> = ({ children }) => {
    return (
        <div className="hint-container">
            <Caption1>{children}</Caption1>
        </div>
    );
};
