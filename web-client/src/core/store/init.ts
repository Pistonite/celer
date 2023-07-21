//! Setup the store

import { configureStore } from "@reduxjs/toolkit";
import reduxWatch from "redux-watch";
import { SettingsState, settingsReducer, settingsSelector } from "./settings";
import { viewReducer } from "./view";
import { documentReducer } from "./document";
import { switchTheme } from "low/utils";

/// Create the store and return it
export const initStore = () => {
    const store = configureStore({
        reducer: {
            ...settingsReducer,
            ...viewReducer,
            ...documentReducer,
        },
    });

    // TODO: this should be in the kernel
    const watchSettings = reduxWatch(() => settingsSelector(store.getState()));
    store.subscribe(
        watchSettings((newVal: SettingsState, oldVal: SettingsState) => {
            console.log({
                message: "settings changed",
                new: newVal,
                old: oldVal,
            });

            // switch theme
            if (newVal.theme !== oldVal.theme) {
                switchTheme(newVal.theme);
            }
        }),
    );

    return store;
};

/// Convenience types for the store
export type AppStore = ReturnType<typeof initStore>;
export type AppState = ReturnType<AppStore["getState"]>;
/// Interface for only dispatching actions
export type AppDispatcher = {
    dispatch: AppStore["dispatch"];
}
