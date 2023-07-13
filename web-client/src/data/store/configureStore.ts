//! Setup the store

import { configureStore } from "@reduxjs/toolkit";
import reduxWatch from "redux-watch";
import { SettingsStore, settingsReducer, settingsSelector } from "./settings";
import { viewReducer } from "./view";
import { documentReducer } from "./document";
import { switchTheme } from "data/util";

/// The store
export const store = configureStore({
    reducer: {
        ...settingsReducer,
        ...viewReducer,
        ...documentReducer,
    },
});

switchTheme(settingsSelector(store.getState()).theme);

const watchSettings = reduxWatch(() => settingsSelector(store.getState()));
store.subscribe(
    watchSettings((newVal: SettingsStore, oldVal: SettingsStore) => {
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
// TODO need a way for outsiders to subscribe easily
