import {
    FluentProvider,
    webDarkTheme,
    webLightTheme,
} from "@fluentui/react-components";
import { PropsWithChildren } from "react";

export type FluentProviderWrapperProps = PropsWithChildren<{
    isDarkMode: boolean;
}>;

export const FluentProviderWrapper: React.FC<FluentProviderWrapperProps> = ({
    isDarkMode,
    children,
}) => {
    return (
        <FluentProvider theme={isDarkMode ? webDarkTheme : webLightTheme}>
            {children}
        </FluentProvider>
    );
};
