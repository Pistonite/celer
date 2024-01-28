//! Editor tab of the settings dialog

import { useEffect, useMemo, useState } from "react";
import { Dropdown, Field, Switch, Option } from "@fluentui/react-components";
import { useSelector } from "react-redux";

import { useKernel } from "core/kernel";
import { EditorMode } from "core/editor";
import { settingsActions, settingsSelector, viewSelector } from "core/store";
import { EntryPointsSorted } from "low/celerc";
import { useActions } from "low/store";

import { SettingsSection } from "./SettingsSection";

const DEFAULT_ENTRY_POINT = "default";
const DEFAULT_ENTRY_PATH = "/project.yaml";

const WORKFLOWS = {
    external: {
        name: "External editor",
        hint: "Make changes to the route files using an external program. The browser will watch for changes in the file system and automatically reload and recompile the route.",
    },
    web: {
        name: "Web editor",
        hint: "Make changes to the route files using an editor in the browser. The changes can be automatically saved to the file system.",
    },
} as const;

export const EditorSettings: React.FC = () => {
    const { rootPath } = useSelector(viewSelector);
    const {
        showFileTree,
        autoSaveEnabled,
        compilerEntryPath,
        compilerUseCachedPrepPhase: compilerUseCachePack0,
        editorMode,
    } = useSelector(settingsSelector);
    const {
        setShowFileTree,
        setAutoSaveEnabled,
        setCompilerEntryPath,
        setCompilerUseCachePack0,
        setEditorMode,
    } = useActions(settingsActions);

    const kernel = useKernel();
    const [entryPoints, setEntryPoints] = useState<EntryPointsSorted>([]);
    useEffect(() => {
        (async () => {
            if (!rootPath) {
                setEntryPoints([]);
                return;
            }
            const compiler = await kernel.getCompiler();
            const result = await compiler.getEntryPoints();
            if (!result.isOk()) {
                setEntryPoints([]);
                return;
            }
            setEntryPoints(result.inner());
        })();
    }, [kernel, rootPath]);

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
                    validationState={
                        rootPath !== undefined ? "warning" : undefined
                    }
                    validationMessage={
                        rootPath !== undefined
                            ? "Cannot change workflow while a project is open"
                            : undefined
                    }
                    hint={WORKFLOWS[editorMode].hint}
                >
                    <Dropdown
                        disabled={rootPath !== undefined}
                        value={WORKFLOWS[editorMode].name}
                        selectedOptions={[editorMode]}
                        onOptionSelect={(_, data) => {
                            setEditorMode(
                                data.selectedOptions[0] as EditorMode,
                            );
                        }}
                    >
                        <Option text="External Editor" value="external">
                            External Editor
                        </Option>
                        <Option text="Web Editor" value="web">
                            Web Editor
                        </Option>
                    </Dropdown>
                </Field>
            </SettingsSection>
            <SettingsSection title="Web editor">
                <Field
                    label="Show file tree"
                    validationState={
                        editorMode !== "web" ? "warning" : undefined
                    }
                    validationMessage={
                        editorMode !== "web"
                            ? "Only available in web editor workflow"
                            : undefined
                    }
                >
                    <Switch
                        disabled={editorMode !== "web"}
                        checked={!!showFileTree}
                        onChange={(_, data) => setShowFileTree(data.checked)}
                    />
                </Field>
                <Field
                    label="Enable auto-save"
                    validationState={
                        editorMode !== "web" ? "warning" : undefined
                    }
                    validationMessage={
                        editorMode !== "web"
                            ? "Only available in web editor workflow"
                            : undefined
                    }
                    hint="Automatically save changes made in the web editor to the file system on idle. May override changes made to the file in the file system while the file is opened in the web editor."
                >
                    <Switch
                        disabled={editorMode !== "web"}
                        checked={!!autoSaveEnabled}
                        onChange={(_, data) => setAutoSaveEnabled(data.checked)}
                    />
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
