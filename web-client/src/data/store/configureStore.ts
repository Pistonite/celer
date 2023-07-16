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

/// TODO this should be in the kernel
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

/// Class to listen to the store
export class StoreListener<TDeps extends Array<unknown>> {
    /// Previous dependencies
    private previousDeps: TDeps;
    /// Function to get the dependencies
    private getDeps: (state: StoreState) => TDeps;
    /// Function to call when the dependencies change
    private onChange: (newDeps: TDeps, oldDeps: TDeps) => void;
    /// The unsubscribe function
    private unsubscribe: () => void;

    constructor(getDeps: () => TDeps, onChange: (newDeps: TDeps, oldDeps: TDeps) => void) {
        this.getDeps = getDeps;
        this.previousDeps = getDeps();
        this.onChange = onChange;
        this.unsubscribe = store.subscribe(() => {
            const newDeps = this.getDeps(store.getState());
            if (import.meta.env.DEV) {
                if (newDeps.length !== this.previousDeps.length) {
                    console.error("Store dependency length changed. This is not allowed! You will not see this message in production.")
                }
            }
            if (newDeps.every((val, index) => val === this.previousDeps[index])) {
                return;
            }
            this.onChange(newDeps, this.previousDeps);
        });
    }

    /// Unsubscribe from the store
    public delete() {
        this.unsubscribe();
    }
}
