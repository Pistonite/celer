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

export type Store = typeof store;
export type StoreState = ReturnType<typeof store.getState>;

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

