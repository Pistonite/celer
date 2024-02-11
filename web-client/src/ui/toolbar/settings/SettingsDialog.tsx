//! Settings dialog entry point

import "./SettingsDialog.css";

import { useEffect, useMemo, useState } from "react";
import { useSelector } from "react-redux";
import {
    TabList,
    Tab,
    Divider,
    Dropdown,
    Option,
    Field,
    mergeClasses,
} from "@fluentui/react-components";
import {
    Document20Regular,
    Map20Regular,
    Code20Regular,
    Info20Regular,
    Wrench20Regular,
} from "@fluentui/react-icons";

import { useWindowSize } from "ui/shared";
import { viewActions, viewSelector } from "core/store";
import { SettingsTab } from "core/stage";
import { useActions } from "low/store";

import { MapSettings } from "./MapSettings";
import { DocSettings } from "./DocSettings";
import { EditorSettings } from "./EditorSettings";
import { PluginSettings } from "./PluginSettings";
import { MetaSettings } from "./MetaSettings";

type TabData = {
    id: SettingsTab;
    text: string;
    Icon: React.ComponentType;
    Page: React.ComponentType;
};

export const SettingsDialog: React.FC = () => {
    const { windowWidth } = useWindowSize();
    const { setEditingKeyBinding, setSettingsTab } = useActions(viewActions);
    const { stageMode, settingsTab } = useSelector(viewSelector);
    const verticalTabs = windowWidth > 480;

    const tabs: TabData[] = useMemo(() => {
        return [
            {
                id: "doc",
                text: "Document",
                Icon: Document20Regular,
                Page: DocSettings,
            },
            {
                id: "map",
                text: "Map",
                Icon: Map20Regular,
                Page: MapSettings,
            },
            ...(stageMode === "edit"
                ? [
                      {
                          id: "editor",
                          text: "Editor",
                          Icon: Code20Regular,
                          Page: EditorSettings,
                      } as const,
                  ]
                : []),
            {
                id: "plugin",
                text: "Plugins",
                Icon: Wrench20Regular,
                Page: PluginSettings,
            },
            {
                id: "meta",
                text: "Meta",
                Icon: Info20Regular,
                Page: MetaSettings,
            },
        ] satisfies TabData[];
    }, [stageMode]);

    const [selectedText, setSelectedText] = useState("");

    // Refresh tab selection when switching to small screen display
    /* eslint-disable react-hooks/exhaustive-deps*/
    useEffect(() => {
        if (!verticalTabs) {
            const text = tabs.find(({ id }) => id === settingsTab)?.text ?? "";
            setSelectedText(text);
        }
    }, [verticalTabs]);
    /* eslint-enable react-hooks/exhaustive-deps*/

    const switchTab = (tab: SettingsTab) => {
        // cancel editing key binding when switching tabs
        setEditingKeyBinding(undefined);
        setSettingsTab(tab);
    };

    return (
        <div
            id="settings-dialog"
            className={mergeClasses(
                verticalTabs ? "vertical-tabs" : "horizontal-tabs",
            )}
        >
            {verticalTabs ? (
                <TabList
                    vertical
                    selectedValue={settingsTab}
                    onTabSelect={(_, data) => {
                        switchTab(data.value as SettingsTab);
                    }}
                >
                    {tabs.map(({ id, text, Icon }) => (
                        <Tab key={id} id={id} value={id} icon={<Icon />}>
                            {text}
                        </Tab>
                    ))}
                </TabList>
            ) : (
                <Field label="Category">
                    <Dropdown
                        value={selectedText}
                        selectedOptions={[settingsTab]}
                        onOptionSelect={(_, data) => {
                            switchTab(data.selectedOptions[0] as SettingsTab);
                            setSelectedText(data.optionText ?? "");
                        }}
                    >
                        {tabs.map(({ id, text, Icon }) => (
                            <Option key={id} text={text} value={id}>
                                <Icon />
                                {text}
                            </Option>
                        ))}
                    </Dropdown>
                </Field>
            )}
            <Divider
                id="settings-separator"
                className={verticalTabs ? "vertical-tabs" : "horizontal-tabs"}
                vertical={verticalTabs}
            />
            <div id="settings-panel">
                {tabs
                    .filter(({ id }) => id === settingsTab)
                    .map(({ id, Page }) => (
                        <Page key={id} />
                    ))}
            </div>
        </div>
    );
};
