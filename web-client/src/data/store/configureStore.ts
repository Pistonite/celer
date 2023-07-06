//! Setup the store

import { configureStore } from "@reduxjs/toolkit";
import reduxWatch from "redux-watch";
import { settingsReducer, settingsSelector } from "./settings";
import { viewReducer } from "./view";
import { documentReducer } from "./document";

/// The store
export const store = configureStore({
    reducer: {
        ...settingsReducer,
        ...viewReducer,
        ...documentReducer,
    },
});

const watchSettings = reduxWatch(() => settingsSelector(store.getState()));
store.subscribe(
    watchSettings((newVal, oldVal) => {
        console.log({
            message: "settings changed",
            new: newVal,
            old: oldVal,
        });
    }),
);
// TODO need a way for outsiders to subscribe easily
