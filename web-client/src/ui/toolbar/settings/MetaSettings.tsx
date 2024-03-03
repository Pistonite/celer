//! Meta tab of the settings dialog

import { Button, Field } from "@fluentui/react-components";
import { useEffect, useState } from "react";
import { useSelector } from "react-redux";
import { documentSelector, settingsActions, viewSelector } from "core/store";
import { fetchAsString, getApiUrl } from "low/fetch";

import { useActions } from "low/store";
import { SettingsSection } from "./SettingsSection";
import { InfoField } from "./InfoField";

declare global {
    interface Window {
        __CELER_VERSION: string;
    }
}

export const MetaSettings: React.FC = () => {
    const { stageMode } = useSelector(viewSelector);
    const { document } = useSelector(documentSelector);
    const { resetAllSettings } = useActions(settingsActions);
    const project = document?.project;

    const [serverVersion, setServerVersion] = useState("Loading...");
    useEffect(() => {
        const fetchVersion = async () => {
            const version = await fetchAsString(getApiUrl("/version"));
            if ("err" in version) {
                console.error(version.err);
                setServerVersion("Cannot read version");
                return;
            }
            const { val } = version;
            if (val.split(" ", 3).length === 3) {
                setServerVersion("Cannot read version");
                return;
            }
            setServerVersion(val);
        };
        fetchVersion();
    }, [stageMode]);
    return (
        <>
            {!!project && (
                <SettingsSection title="Document">
                    <InfoField label="Title" value={project?.title || ""} />
                    <InfoField label="Version" value={project?.version || ""} />
                    <InfoField label="Source" value={project?.source || ""} />
                    {project &&
                        Object.entries(project.stats).map(([key, value], i) => (
                            <InfoField label={key} value={value} key={i} />
                        ))}
                </SettingsSection>
            )}
            <SettingsSection title="Build">
                <InfoField
                    label="Client Version"
                    value={window.__CELER_VERSION || "Cannot read version"}
                />
                <InfoField label="Server Version" value={serverVersion} />
                <InfoField label="Stage Mode" value={stageMode.toUpperCase()} />
            </SettingsSection>
            <SettingsSection title="Settings">
                <Field
                    label="Reset all settings to default"
                    hint="Click the button to reset all settings. This action is NOT REVERSIBLE!"
                >
                    <Button
                        appearance="primary"
                        onClick={() => resetAllSettings()}
                    >
                        Reset
                    </Button>
                </Field>
            </SettingsSection>
        </>
    );
};
