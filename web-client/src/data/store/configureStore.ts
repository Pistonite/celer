//! Setup the store

import { configureStore } from "@reduxjs/toolkit";
import reduxWatch from "redux-watch";
import { settingsReducer, settingsSelector } from "./settings";

/// The store
export const store = configureStore({
    reducer: {
        ...settingsReducer
    }
});

const watchSettings = reduxWatch(() => settingsSelector(store.getState()));
store.subscribe(watchSettings((newVal, oldVal) => {
    console.log({
        "message": "settings changed",
        "new": newVal,
        "old": oldVal
    });
}));
// TODO need a way for outsiders to subscribe easily