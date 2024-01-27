//! Plugin tab of the settings dialog

import { useSelector } from "react-redux";
import { Body1, Checkbox, CheckboxProps } from "@fluentui/react-components";

import { settingsActions, settingsSelector } from "core/store";
import { AppPluginType } from "core/doc";
import { useActions } from "low/store";

import { SettingsSection } from "./SettingsSection";

export const PluginSettings: React.FC = () => {
    return (
        <>
            <SettingsSection title="App Plugins">
                <Body1 block>
                    Enable or disable plugins pre-configured by Celer
                </Body1>

                <AppPluginCheckbox 
                    type="export-split"
                    label="Export to LiveSplit"
                />
            </SettingsSection>
            <SettingsSection title="Route Plugins">
            </SettingsSection>
            <SettingsSection title="User Plugins">
            </SettingsSection>
        </>
    );
};

const AppPluginCheckbox: React.FC<CheckboxProps & {type: AppPluginType}> = ({type, ...props}) => {
    const { enabledAppPlugins } = useSelector(settingsSelector);
    const { setAppPluginEnabled } = useActions(settingsActions);

    return (
        <PluginCheckbox 
            {...props}
            checked={!!enabledAppPlugins[type]}
            onChange={(_, data) => {
                setAppPluginEnabled({
                    type,
                    enabled: !!data.checked,
                });
            }}
        />
    );
};

const PluginCheckbox: React.FC<CheckboxProps> = (props) => {
    return (
        <Checkbox
            className="settings-checkbox-block"
            {...props}
        />
    );
};
