//! Plugin tab of the settings dialog

import { useMemo, useState } from "react";
import { useSelector } from "react-redux";
import type { CheckboxProps } from "@fluentui/react-components";
import {
    Body1,
    Button,
    Checkbox,
    Field,
    Link,
    MessageBar,
    MessageBarBody,
    Switch,
} from "@fluentui/react-components";
import { produce } from "immer";

import { ErrorBar, PrismEditor } from "ui/shared";
import {
    documentSelector,
    settingsActions,
    settingsSelector,
} from "core/store";
import type { AppPluginType } from "core/doc";
import { parseUserConfigOptions, useDocPluginMetadata } from "core/doc";
import type { Kernel } from "core/kernel";
import { useKernel } from "core/kernel";
import { useActions } from "low/store";
import type { ExecDoc, PluginMetadata } from "low/celerc";
import { console } from "low/utils";

import { SettingsSection } from "./SettingsSection";

export const PluginSettings: React.FC = () => {
    const { document } = useSelector(documentSelector);
    const pluginMetadata = useDocPluginMetadata();

    const routePluginMetadata = useMemo(() => {
        return pluginMetadata.filter((x) => !x.isFromUser);
    }, [pluginMetadata]);
    const userPluginMetadata = useMemo(() => {
        return pluginMetadata.filter((x) => x.isFromUser);
    }, [pluginMetadata]);
    const { enableUserPlugins, userPluginConfig } =
        useSelector(settingsSelector);
    const { setPluginMetadata, setUserPluginEnabled, setUserPluginConfig } =
        useActions(settingsActions);

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
                    {getRoutePluginMessage(!!document, routePluginMetadata)}
                </Body1>
                {routePluginMetadata?.map((plugin, i) => (
                    <PluginCheckbox
                        key={i}
                        label={plugin.displayId}
                        checked={plugin.isEnabled}
                        onChange={(_, data) => {
                            if (!document) {
                                return;
                            }
                            setPluginMetadata({
                                title: document.project.title,
                                metadata: produce(pluginMetadata, (draft) => {
                                    draft[i].isEnabled = !!data.checked;
                                }),
                            });
                        }}
                    />
                ))}
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
                {userPluginMetadata?.length > 0 && (
                    <Body1 block>
                        The following plugins are loaded from your user config
                        or app config for this route, you can enable or disable
                        them below.
                    </Body1>
                )}
                {userPluginMetadata?.map((plugin, i) => (
                    <PluginCheckbox
                        key={i}
                        label={plugin.displayId}
                        checked={plugin.isEnabled}
                        onChange={(_, data) => {
                            if (!document) {
                                return;
                            }
                            setPluginMetadata({
                                title: document.project.title,
                                metadata: produce(pluginMetadata, (draft) => {
                                    draft[
                                        i + routePluginMetadata.length
                                    ].isEnabled = !!data.checked;
                                }),
                            });
                        }}
                    />
                ))}
            </SettingsSection>
        </>
    );
};

const getRoutePluginMessage = (
    documentLoaded: boolean,
    pluginMetadata: PluginMetadata[] | undefined,
) => {
    if (!documentLoaded || pluginMetadata === undefined) {
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
    let { err } = parseUserConfigOptions(config, document?.project.title);
    while (true) {
        const response = await kernel.alertMgr.showRich({
            title: "User Plugins",
            component: () => {
                return (
                    <UserPluginConfigEditor
                        initialError={err}
                        initialValue={config}
                        onChange={(x) => {
                            kernel.alertMgr.modifyActions({
                                extraActions: [],
                            });
                            config = x;
                        }}
                    />
                );
            },
            okButton: "Save",
            cancelButton: "Cancel",
            extraActions: err
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
        ({ err } = parseUserConfigOptions(config, document?.project.title));
        if (!err) {
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
                <Link href="/docs/plugin/settings" target="_blank">
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
