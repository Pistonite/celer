//! Setup the store
import { configureStore } from "@reduxjs/toolkit";
import { settingsReducer } from "./settings";
import { viewReducer } from "./view";
import { documentReducer } from "./document";

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
