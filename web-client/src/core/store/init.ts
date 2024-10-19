//! Setup the store
import { configureStore } from "@reduxjs/toolkit";
import { useStore as useReduxStore } from "react-redux";

import { settingsReducer } from "./settings.ts";
import { viewReducer } from "./view.ts";
import { documentReducer } from "./document.ts";

/// Create the store and return it
export const initStore = () => {
    const store = configureStore({
        reducer: {
            ...settingsReducer,
            ...viewReducer,
            ...documentReducer,
        },
    });

    return store;
};

/// Convenience types for the store
export type AppStore = ReturnType<typeof initStore>;
export type AppState = ReturnType<AppStore["getState"]>;
/// Interface for only dispatching actions
export type AppDispatcher = {
    dispatch: AppStore["dispatch"];
};

export const useStore: () => AppStore = useReduxStore;
