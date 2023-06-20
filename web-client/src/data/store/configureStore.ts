//! Setup the store

import { configureStore } from "@reduxjs/toolkit";
import reduxWatch from "redux-watch";
import { settingsReducer, settingsSelector } from "./settings";
import { toolbarReducer } from "./toolbar";

/// The store
export const store = configureStore({
    reducer: {
        ...settingsReducer,
        ...toolbarReducer
    }
});

const watchSettings = reduxWatch(store.getState, "settings", (a,b)=>false);
store.subscribe(watchSettings((newVal, oldVal) => {
    console.log({
        "message": "settings changed",
        "new": newVal,
        "old": oldVal
    });
}));
// TODO need a way for outsiders to subscribe easily