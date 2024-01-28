//! Plugin tab of the settings dialog

import { useEffect, useState, useTransition } from "react";
import { useSelector } from "react-redux";
import { Body1, Button, Checkbox, CheckboxProps, Field, Link, Switch, Textarea} from "@fluentui/react-components";

import { ErrorBar } from "ui/shared";
import { documentSelector, settingsActions, settingsSelector } from "core/store";
import { AppPluginType, parseUserConfigOptions, useDocDisabledPlugins } from "core/doc";
import { useActions } from "low/store";
import { PluginMetadata } from "low/celerc";
import { DOMId, useDebouncer } from "low/utils";

import { SettingsSection } from "./SettingsSection";

const UserPluginConfigTextarea = new DOMId("user-plugin-config-textarea");
UserPluginConfigTextarea.style({
    "font-family": "monospace",
    "font-size": "12px",
});
const adjustUserPluginConfigTextareaHeight = () => {
    const element = UserPluginConfigTextarea.get();
    if (!element) {
        return;
    }
    // shrink it
    element.style.height = "32px";
    const height = Math.max(32, Math.min(element.scrollHeight, 300));
    element.style.height = `${height}px`;
}

export const PluginSettings: React.FC = () => {
    const { pluginMetadata, document, serial } = useSelector(documentSelector);
    // cache the plugin metadata once the dialog shows up
    // so the UI doesn't jump around when the plugin metadata changes
    const [cachedPluginMetadata, setCachedPluginMetadata] = useState(pluginMetadata);
    const { enableUserPlugins, userPluginConfig } = useSelector(settingsSelector);
    const { setRoutePluginEnabled, setUserPluginEnabled, setUserPluginConfig } = useActions(settingsActions);
    const disabledPlugins = useDocDisabledPlugins();

    const [userPluginSyntaxError, setUserPluginSyntaxError] = useState<string | undefined>(undefined);
    const [configText, setConfigText] = useState(userPluginConfig);
    const dispatchDebouncer = useDebouncer(2000);
    const [_, startTransition] = useTransition();

    useEffect(adjustUserPluginConfigTextareaHeight, []);
    /* eslint-disable react-hooks/exhaustive-deps*/
    useEffect(() => {
        startTransition(() => {
            const [_, error] = parseUserConfigOptions(configText, document);
            setUserPluginSyntaxError(error);
        });
    }, [configText, serial]);
    /* eslint-enable react-hooks/exhaustive-deps*/
    return (
        <>
            <SettingsSection title="App Plugins">
                <Body1 block>
                    Enable or disable plugins pre-configured by Celer
                </Body1>

                <AppPluginCheckbox 
                    type="export-split"
                    label="Export to LiveSplit"
                    disabled={true}
                />
            </SettingsSection>
            <SettingsSection title="Route Plugins">
                <Body1 block>
                    {getRoutePluginMessage(cachedPluginMetadata)}
                </Body1>
                {
                    cachedPluginMetadata?.map((plugin, i) => 
                        <PluginCheckbox
                            key={i}
                            label={(plugin.isFromUser?"(user) ":"")+plugin.name}
                            checked={!disabledPlugins.includes(plugin.id)}
                            disabled={plugin.isFromUser}
                            onChange={(_, data) => {
                                if (!document) {
                                    return;
                                }
                                setRoutePluginEnabled({
                                    docTitle: document.project.title,
                                    plugin: plugin.id,
                                    enabled: !!data.checked,
                                });
                            }}
                        />
                    )
                }
                <Field>
                    <Button 
                        appearance="primary" 
                        disabled={cachedPluginMetadata === pluginMetadata}
                        onClick={() => {
                            setCachedPluginMetadata(pluginMetadata);
                        }}
                    >
                        Refresh route plugins
                    </Button>
                </Field>
            </SettingsSection>
            <SettingsSection title="User Plugins">
                <Field
                    label={<>
                        Configure extra plugins to use when loading route documents.{" "}
                        <Link href={`${window.location.origin}/docs/plugin/settings`} target="_blank">
                            Learn more
                        </Link>
                    </>}
                    hint={document ? `The current document title is "${document.project.title}"` : undefined}
                    validationState={userPluginSyntaxError ? "error" : "success"}
                    validationMessage={userPluginSyntaxError ? "There is a syntax error with your configuration" : "Configuration syntax is valid"}
                >
                    <Switch 
                        label="Enable user plugins"
                        checked={!!enableUserPlugins}
                        onChange={(_, data) => {
                            setUserPluginEnabled(!!data.checked);
                        }}
                    />
                </Field>
                <Field>
                    <Textarea 
                        spellCheck={false}
                        id={UserPluginConfigTextarea.id}
                        disabled={!enableUserPlugins}
                        value={configText}
                        onChange={(_, data) => {
                            setConfigText(data.value);
                            adjustUserPluginConfigTextareaHeight();
                            dispatchDebouncer(() => {
                                setUserPluginConfig(data.value);
                            });
                        }}

                    />
                </Field>
                <ErrorBar title="Syntax Error">
                    {userPluginSyntaxError}
                </ErrorBar>
            </SettingsSection>
        </>
    );
};

const getRoutePluginMessage = (pluginMetadata: PluginMetadata[] | undefined) => {
    if (pluginMetadata === undefined) {
        return "Once a route document is loaded, you can enable or disable plugins here.";
    }
    if (pluginMetadata.length === 0) {
        return "This route document does not load any plugins";
    }
    return "Enable or disable plugins loaded by the route document";
}

const AppPluginCheckbox: React.FC<CheckboxProps & {type: AppPluginType}> = ({type, ...props}) => {
    // const { enabledAppPlugins } = useSelector(settingsSelector);
    const { setAppPluginEnabled } = useActions(settingsActions);

    return (
        <PluginCheckbox 
            {...props}
            checked={false} // TODO #33: export splits
            // checked={!!enabledAppPlugins[type]}
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
            {...props}
            className="settings-checkbox-block"
        />
    );
};
