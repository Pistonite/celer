//! Plugin tab of the settings dialog

import { useState } from "react";
import { useSelector } from "react-redux";
import {
    Body1,
    Button,
    Checkbox,
    CheckboxProps,
    Field,
    Link,
    MessageBar,
    MessageBarBody,
    Switch,
} from "@fluentui/react-components";

import { ErrorBar, PrismEditor } from "ui/shared";
import {
    documentSelector,
    settingsActions,
    settingsSelector,
} from "core/store";
import {
    AppPluginType,
    parseUserConfigOptions,
    useDocDisabledPlugins,
} from "core/doc";
import { Kernel, useKernel } from "core/kernel";
import { useActions } from "low/store";
import { ExecDoc, PluginMetadata } from "low/celerc";
import { console } from "low/utils";

import { SettingsSection } from "./SettingsSection";

export const PluginSettings: React.FC = () => {
    const { pluginMetadata, document } = useSelector(documentSelector);
    // cache the plugin metadata once the dialog shows up
    // so the UI doesn't jump around when the plugin metadata changes
    const [cachedPluginMetadata, setCachedPluginMetadata] =
        useState(pluginMetadata);
    const { enableUserPlugins, userPluginConfig } =
        useSelector(settingsSelector);
    const { setRoutePluginEnabled, setUserPluginEnabled, setUserPluginConfig } =
        useActions(settingsActions);
    const disabledPlugins = useDocDisabledPlugins();

    const kernel = useKernel();

    return (
        <>
            <SettingsSection title="App Plugins">
                <Body1 block>
                    Enable or disable plugins pre-configured by Celer
                </Body1>

                <AppPluginCheckbox
                    type="export-split"
                    label="Export split files"
                />
            </SettingsSection>
            <SettingsSection title="Route Plugins">
                <Body1 block>
                    {getRoutePluginMessage(cachedPluginMetadata)}
                </Body1>
                {cachedPluginMetadata?.map((plugin, i) => (
                    <PluginCheckbox
                        key={i}
                        label={
                            (plugin.isFromUser ? "(user) " : "") + plugin.name
                        }
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
                ))}
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
                    label={
                        <>
                            Configure extra plugins to use when loading route
                            documents.{" "}
                            <Link href="/docs/plugin/settings" target="_blank">
                                Learn more
                            </Link>
                        </>
                    }
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
                    <Button
                        appearance="primary"
                        disabled={!enableUserPlugins}
                        onClick={() =>
                            editUserPluginConfig(
                                userPluginConfig,
                                kernel,
                                document,
                                setUserPluginConfig,
                            )
                        }
                    >
                        Edit Config
                    </Button>
                </Field>
            </SettingsSection>
        </>
    );
};

const getRoutePluginMessage = (
    pluginMetadata: PluginMetadata[] | undefined,
) => {
    if (pluginMetadata === undefined) {
        return "Once a route document is loaded, you can enable or disable plugins here.";
    }
    if (pluginMetadata.length === 0) {
        return "This route document does not load any plugins";
    }
    return "Enable or disable plugins loaded by the route document";
};

const AppPluginCheckbox: React.FC<CheckboxProps & { type: AppPluginType }> = ({
    type,
    ...props
}) => {
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
    return <Checkbox {...props} className="settings-checkbox-block" />;
};

const editUserPluginConfig = async (
    userPluginConfig: string,
    kernel: Kernel,
    document: ExecDoc | undefined,
    setUserPluginConfig: (x: string) => void,
): Promise<void> => {
    let config = userPluginConfig;
    let [_, error] = parseUserConfigOptions(config, document);
    // eslint-disable-next-line no-constant-condition
    while (true) {
        const response = await kernel.getAlertMgr().showRich({
            title: "User Plugins",
            component: () => {
                return (
                    <UserPluginConfigEditor
                        initialError={error}
                        initialValue={config}
                        onChange={(x) => {
                            kernel.getAlertMgr().modifyActions({
                                extraActions: [],
                            });
                            config = x;
                        }}
                    />
                );
            },
            okButton: "Save",
            cancelButton: "Cancel",
            extraActions: error
                ? [
                      {
                          id: "force",
                          text: "Save with this error",
                      } as const,
                  ]
                : [],
        });
        if (!response) {
            console.info("user cancelled user plugin config");
            return;
        }
        if (response === "force") {
            break;
        }
        [_, error] = parseUserConfigOptions(config, document);
        if (!error) {
            break;
        }
        console.error("user plugin config has errors");
    }
    console.info("saving new user plugin config");
    setUserPluginConfig(config);
};

type UserPluginConfigEditorProps = {
    initialValue: string;
    onChange: (value: string) => void;
    initialError: string | undefined;
};

const UserPluginConfigEditor: React.FC<UserPluginConfigEditorProps> = ({
    initialError,
    initialValue,
    onChange,
}) => {
    const { document } = useSelector(documentSelector);
    const [currentValue, setCurrentValue] = useState(initialValue);
    const [error, setError] = useState(initialError);
    return (
        <div>
            <Body1 block style={{ marginBottom: 4 }}>
                Please edit your plugin configuration below.{" "}
                <Link href="/docs/plugin/settine" target="_blank">
                    Learn more
                </Link>
            </Body1>
            {document !== undefined && (
                <MessageBar intent="info">
                    <MessageBarBody>
                        The current document title is "{document.project.title}"
                    </MessageBarBody>
                </MessageBar>
            )}
            {error !== undefined && (
                <ErrorBar title="Syntax Error">{error}</ErrorBar>
            )}
            <div>
                <PrismEditor
                    language="yaml"
                    value={currentValue}
                    setValue={(x) => {
                        setCurrentValue(x);
                        setError(undefined);
                        onChange(x);
                    }}
                />
            </div>
        </div>
    );
};
