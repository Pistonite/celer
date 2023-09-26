//! Settings dialog entry point

import "./SettingsDialog.css";

import clsx from "clsx";
import { useMemo } from "react";
import { useSelector } from "react-redux";
import { TabList, Tab, Divider } from "@fluentui/react-components";
import { Document20Regular, Map20Regular, Code20Regular } from "@fluentui/react-icons";

import { useWindowSize } from "ui/shared";
import { viewActions, viewSelector } from "core/store";
import { SettingsTab } from "core/stage";
import { useActions } from "low/store";

import { MapSettings } from "./MapSettings";
import { DocSettings } from "./DocSettings";
import { EditorSettings } from "./EditorSettings";

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
    const verticalTabs = windowWidth > 400;

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
            ...(stageMode === "edit" ? [
                {
                    id: "editor",
                    text: "Editor",
                    Icon: Code20Regular,
                    Page: EditorSettings,
                } as const,
            ] : []),
        ] satisfies TabData[];
    }, [stageMode]);

    return (
        <div
            id="settings-dialog"
            className={clsx(verticalTabs ? "vertical-tabs" : "horizontal-tabs")}
        >
            <TabList
                vertical={verticalTabs}
                selectedValue={settingsTab}
                onTabSelect={(_, data) => {
                    // cancel editing key binding when switching tabs
                    setEditingKeyBinding(undefined);
                    setSettingsTab(data.value as SettingsTab);
                }}
            >
                {
                    tabs.map(({id, text, Icon}) => (
                        <Tab
                        key={id}
                            id={id}
                            value={id}
                            icon={<Icon />}
                            >{text}</Tab>
                    ))
                }
            </TabList>
            <Divider
                id="settings-separator"
                className={clsx(
                    verticalTabs ? "vertical-tabs" : "horizontal-tabs",
                )}
                vertical={verticalTabs}
            />
            <div id="settings-panel">
                {
                    tabs.filter(({id}) => id === settingsTab).map(({id, Page}) => (
                        <Page key={id} />
                    ))
                }
            </div>
        </div>
    );
};
