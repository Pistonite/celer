import {
    FluentProvider,
    webDarkTheme,
    webLightTheme,
} from "@fluentui/react-components";
import { PropsWithChildren } from "react";

import { isInDarkMode } from "low/utils";

export const FluentProviderWrapper: React.FC<PropsWithChildren> = ({
    children,
}) => {
    const dark = isInDarkMode();
    return (
        <FluentProvider theme={dark ? webDarkTheme : webLightTheme}>
            {children}
        </FluentProvider>
    );
};
