//! Editor tab of the settings dialog

import { useEffect, useMemo, useState } from "react";
import { Dropdown, Field, Switch, Option } from "@fluentui/react-components";
import { useSelector } from "react-redux";

import { useKernel } from "core/kernel";
import {
    settingsActions,
    settingsSelector,
    viewActions,
    viewSelector,
} from "core/store";
import { EntryPointsSorted } from "low/celerc";
import { useActions } from "low/store";

import { SettingsSection } from "./SettingsSection";

const DEFAULT_ENTRY_POINT = "default";
const DEFAULT_ENTRY_PATH = "/project.yaml";

export const EditorSettings: React.FC = () => {
    const { supportsSave } = useSelector(viewSelector);
    const {
        showFileTree,
        autoSaveEnabled,
        autoLoadEnabled,
        deactivateAutoLoadAfterMinutes,
        compilerEntryPath,
        compilerUseCachePack0,
        editorMode,
    } = useSelector(settingsSelector);
    const {
        setShowFileTree,
        setAutoSaveEnabled,
        setAutoLoadEnabled,
        setDeactivateAutoLoadAfterMinutes,
        setCompilerEntryPath,
        setCompilerUseCachePack0,
        setEditorMode,
    } = useActions(settingsActions);
    const { setAutoLoadActive } = useActions(viewActions);
    const deactivateAutoLoadMinutesOptions = [5, 10, 15, 30, 60];

    const kernel = useKernel();
    const [entryPoints, setEntryPoints] = useState<EntryPointsSorted>([]);
    useEffect(() => {
        (async () => {
            const compiler = kernel.getCompiler();
            if (!compiler) {
                setEntryPoints([]);
                return;
            }
            const result = await compiler.getEntryPoints();
            if (!result.isOk()) {
                setEntryPoints([]);
                return;
            }
            setEntryPoints(result.inner());
        })();
    }, [kernel]);

    const selectedEntryPoint = useMemo(() => {
        if (!compilerEntryPath) {
            return DEFAULT_ENTRY_POINT;
        }
        const selected = entryPoints.find(
            ([_, path]) => path === compilerEntryPath,
        );
        return selected ? selected[0] : DEFAULT_ENTRY_POINT;
    }, [entryPoints, compilerEntryPath]);

    // [name, path] pairs
    const entryPointOptions = useMemo(() => {
        const options = [[DEFAULT_ENTRY_POINT, DEFAULT_ENTRY_PATH]];
        entryPoints.forEach(([name, path]) => {
            if (name === DEFAULT_ENTRY_POINT) {
                options[0][1] = path;
            } else {
                options.push([name, path]);
            }
        });
        return options;
    }, [entryPoints]);

    return (
        <>
            <SettingsSection title="General">
                <Field
                    label="Workflow"
                    hint="Web editor lets you "
                >
                </Field>
            </SettingsSection>
            <SettingsSection title="Editor">
                <Field label="Show file tree">
                    <Switch
                        checked={!!showFileTree}
                        onChange={(_, data) => setShowFileTree(data.checked)}
                    />
                </Field>
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
                    label="Entry point"
                    hint={
                        <>
                            Choose which entry point to compile from. Entry
                            points are defined with the{" "}
                            <code>entry-points</code> property.{" "}
                            <a
                                target="_blank"
                                href="/docs/route/file-structure#multiple-projects-in-the-same-repo"
                            >
                                Learn more
                            </a>
                        </>
                    }
                    validationState={
                        entryPoints.length === 0 ? "warning" : undefined
                    }
                    validationMessage={
                        entryPoints.length === 0
                            ? "No custom entry points found. If you updated the config externally, close and reopen the dialog to refresh"
                            : undefined
                    }
                >
                    <Dropdown
                        value={formatCompilerEntryText(
                            selectedEntryPoint,
                            compilerEntryPath || DEFAULT_ENTRY_PATH,
                        )}
                        selectedOptions={[
                            compilerEntryPath || DEFAULT_ENTRY_PATH,
                        ]}
                        onOptionSelect={(_, data) => {
                            setCompilerEntryPath(data.selectedOptions[0]);
                        }}
                    >
                        {entryPointOptions.map(([name, path]) => {
                            const text = formatCompilerEntryText(name, path);
                            return (
                                <Option key={path} text={text} value={path}>
                                    {text}
                                </Option>
                            );
                        })}
                    </Dropdown>
                </Field>
                <Field
                    label="Cache Config"
                    hint="Allow the compiler to cache certain configurations such as presets and plugins to speed up compilation."
                >
                    <Switch
                        checked={!!compilerUseCachePack0}
                        onChange={(_, data) => {
                            setCompilerUseCachePack0(data.checked);
                        }}
                    />
                </Field>
            </SettingsSection>
        </>
    );
};

const formatCompilerEntryText = (name: string, path: string) => {
    return `${name} (${path})`;
};
