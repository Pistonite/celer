//! Settings dialog entry point

import "./SettingsDialog.css";

import clsx from "clsx";
import { useState } from "react";
import { TabList, Tab, Divider } from "@fluentui/react-components";
import { Map20Regular } from "@fluentui/react-icons";

import { useWindowSize } from "core/utils";

import { MapSettings } from "./MapSettings";

const Tabs = {
    map: "map",
    info: "info",
};

export const SettingsDialog: React.FC = () => {
    const [selectedTab, setSelectedTab] = useState<string>(Tabs.map);
    const { windowWidth } = useWindowSize();
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
                    setSelectedTab(data.value as string);
                }}
            >
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
                {selectedTab === Tabs.map && <MapSettings />}
            </div>
        </div>
    );
};
