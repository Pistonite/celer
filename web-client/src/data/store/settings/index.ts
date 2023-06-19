//! Settings store slice is used for user settings, such as theme, layout, map settings, etc.

import { ReducerDecl, configureSlice } from 'data/store/util';
import { useSelector } from 'react-redux';
import { LayoutSettings, initialLayoutSettings, LayoutReducers } from './layout';

/// Local storage key
const LOCAL_STORAGE_KEY = "Celer.Settings";

type SettingsStore = LayoutSettings;

/// Try loading initial state from local storage
const loadState = (): SettingsStore => {
    const state = localStorage.getItem(LOCAL_STORAGE_KEY);
    const loadedState = state ? JSON.parse(state) : {};
    return {
        ...initialLayoutSettings,
        ...loadedState,
    };
}


/// TODO remove this
const setCurrentViewingLayoutTest: ReducerDecl<SettingsStore> = (state) => {
    console.log(state)
};

/// The setting state slice
export const {
    settingsReducer,
    settingsActions,
    settingsSelector
} = configureSlice({
    name: "settings",
    initialState: loadState(),
    reducers: {
        ...LayoutReducers,
        setCurrentViewingLayoutTest
    }
});

export const useSettingsStore = () => {
    useSelector(settingsSelector);
}