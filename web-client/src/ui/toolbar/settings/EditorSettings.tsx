//! Editor tab of the settings dialog

import { Dropdown, Field, Switch, Option } from "@fluentui/react-components";
import { useSelector } from "react-redux";

import {
    settingsActions,
    settingsSelector,
    viewActions,
    viewSelector,
} from "core/store";
import { useActions } from "low/store";

import { SettingsSection } from "./SettingsSection";

export const EditorSettings: React.FC = () => {
    const { supportsSave } = useSelector(viewSelector);
    const {
        showFileTree,
        autoSaveEnabled,
        autoLoadEnabled,
        deactivateAutoLoadAfterMinutes,
        compilerUseCache,
    } = useSelector(settingsSelector);
    const {
        setShowFileTree,
        setAutoSaveEnabled,
        setAutoLoadEnabled,
        setDeactivateAutoLoadAfterMinutes,
        setCompilerUseCache,
    } = useActions(settingsActions);
    const { setAutoLoadActive } = useActions(viewActions);
    const deactivateAutoLoadMinutesOptions = [5, 10, 15, 30, 60];
    return (
        <>
            <SettingsSection title="Appearance">
                <Field label="Show file tree">
                    <Switch
                        checked={!!showFileTree}
                        onChange={(_, data) => setShowFileTree(data.checked)}
                    />
                </Field>
            </SettingsSection>
            <SettingsSection title="Save">
                <Field
                    label="Enable auto-save"
                    hint="Automatically save changes made in the web editor to the file system on idle. May override changes made to the file in the file system while the file is opened in the web editor."
                    validationState={supportsSave ? undefined : "error"}
                    validationMessage={
                        supportsSave
                            ? undefined
                            : "Saving is not supported by your browser."
                    }
                >
                    <Switch
                        disabled={!supportsSave}
                        checked={!!autoSaveEnabled}
                        onChange={(_, data) => setAutoSaveEnabled(data.checked)}
                    />
                </Field>
            </SettingsSection>
            <SettingsSection title="Load">
                <Field
                    label="Enable auto-load"
                    hint="Automatically load changes made in the file system to the web editor. Will not load a file if the file is opened in the web editor and has unsaved changes. If auto-save is also enabled, changes are always saved first, then loaded."
                >
                    <Switch
                        checked={!!autoLoadEnabled}
                        onChange={(_, data) => {
                            const enabled = data.checked;
                            setAutoLoadEnabled(enabled);
                            if (enabled) {
                                setAutoLoadActive(true);
                            }
                        }}
                    />
                </Field>
                <Field
                    label="Deactivate auto-load after:"
                    hint="Automatically turn off auto-load after a certain time of inactivity to save resources. It will reactivate after manually pressing the load button from the toolbar."
                >
                    <Dropdown
                        disabled={!autoLoadEnabled}
                        value={
                            deactivateAutoLoadAfterMinutes > 0
                                ? `${deactivateAutoLoadAfterMinutes} minutes`
                                : "Never"
                        }
                        selectedOptions={[
                            deactivateAutoLoadAfterMinutes.toString(),
                        ]}
                        onOptionSelect={(_, data) => {
                            setDeactivateAutoLoadAfterMinutes(
                                parseInt(data.optionValue ?? "-1") || -1,
                            );
                        }}
                    >
                        {deactivateAutoLoadMinutesOptions.map((minutes) => (
                            <Option
                                key={minutes}
                                text={`${minutes} minutes`}
                                value={`${minutes}`}
                            >
                                {minutes} minutes
                            </Option>
                        ))}
                        <Option text="Never" value={"-1"}>
                            Never
                        </Option>
                    </Dropdown>
                </Field>
            </SettingsSection>
            <SettingsSection title="Compiler">
                <Field
                    label="Cache remote resources"
                    hint="Cache resources loaded from the internet to speed up compilation. The cache has a TTL of 5 minutes. You may want to disable this for debugging purposes."
                >
                    <Switch
                        checked={!!compilerUseCache}
                        onChange={(_, data) => {
                            setCompilerUseCache(data.checked);
                        }}
                    />
                </Field>
            </SettingsSection>
        </>
    );
};
