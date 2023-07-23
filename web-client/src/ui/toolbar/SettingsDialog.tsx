//! Settings dialog entry point

import "./SettingsDialog.css";

import clsx from "clsx";
import { useState } from "react";
import { TabList, Tab, Divider } from "@fluentui/react-components";
import { Document20Regular, Map20Regular } from "@fluentui/react-icons";

import { useWindowSize } from "ui/shared";
import { useActions } from "low/store";
import { viewActions } from "core/store";

import { MapSettings } from "./MapSettings";
import { DocSettings } from "./DocSettings";

const Tabs = {
    doc: "doc",
    map: "map",
    info: "info",
};

export const SettingsDialog: React.FC = () => {
    const [selectedTab, setSelectedTab] = useState<string>(Tabs.doc);
    const { windowWidth } = useWindowSize();
    const { setEditingKeyBinding } = useActions(viewActions);
    const verticalTabs = windowWidth > 400;

    return (
        <div
            id="settings-dialog"
            className={clsx(verticalTabs ? "vertical-tabs" : "horizontal-tabs")}
        >
            <TabList
                vertical={verticalTabs}
                selectedValue={selectedTab}
                onTabSelect={(_, data) => {
                    // cancel editing key binding when switching tabs
                    setEditingKeyBinding(undefined);
                    setSelectedTab(data.value as string);
                }}
            >
                <Tab id={Tabs.doc} value={Tabs.doc} icon={<Document20Regular />}>
                    Document
                </Tab>
                <Tab id={Tabs.map} value={Tabs.map} icon={<Map20Regular />}>
                    Map
                </Tab>
                <Tab id={Tabs.info} value={Tabs.info} icon={<Map20Regular />}>
                    Info
                </Tab>
            </TabList>
            <Divider
                id="settings-separator"
                className={clsx(
                    verticalTabs ? "vertical-tabs" : "horizontal-tabs",
                )}
                vertical={verticalTabs}
            />
            <div id="settings-panel">
                {selectedTab === Tabs.doc && <DocSettings key={Tabs.doc} />}
                {selectedTab === Tabs.map && <MapSettings key={Tabs.map} />}
            </div>
        </div>
    );
};
