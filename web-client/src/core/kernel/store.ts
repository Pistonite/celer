import reduxWatch from "redux-watch";

import { isRecompileNeeded } from "core/doc";
import { AppState, AppStore, SettingsState, initStore, saveSettings, settingsSelector } from "core/store";
import { consoleKernel as console } from "low/utils";

import { Kernel } from "./Kernel";

/// Create the store and bind listeners to the kernel
export const createAndBindStore = (kernel: Kernel): AppStore => {
    console.info("initializing store...");
    const store = initStore();

    const watchSettings = reduxWatch(() =>
        settingsSelector(store.getState()),
    );

    store.subscribe(
        watchSettings((newVal: SettingsState, _oldVal: SettingsState) => {
            // save settings to local storage
            console.info("saving settings...");
            saveSettings(newVal);
        }),
    );

    const watchAll = reduxWatch(() => store.getState());
    store.subscribe(
        watchAll(async (newVal: AppState, oldVal: AppState) => {
            if (await isRecompileNeeded(newVal, oldVal)) {
                console.info("reloading document due to state change...");
                await kernel.reloadDocument();
            }
        }),
    );

    return store;
}
