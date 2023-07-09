//! Shared component for setting sections

import { PropsWithChildren } from "react";
import { Divider, Subtitle2 } from "@fluentui/react-components";

/// Props for SettingsSection
export type SettingsSectionProps = {
    /// Title of the section
    title: string;
};

/// The component
export const SettingsSection: React.FC<
    PropsWithChildren<SettingsSectionProps>
> = ({ title, children }) => {
    return (
        <div className="settings-section">
            <Subtitle2>{title}</Subtitle2>
            <Divider />
            <div className="settings-section-inner">{children}</div>
        </div>
    );
};
