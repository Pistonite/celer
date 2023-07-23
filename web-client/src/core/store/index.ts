//! core/store
//!
//! Core store for application state. Uses redux.
//!
//! The global state consists of:
//! - SettingsState - values persisted in localStorage
//! - ViewState     - values that are not persisted
//! - DocumentState - state of the current route document
//!
//! Each state slice is comprised of:
//! - The (combined) reducer for setting up the redux state
//! - The actions for updating the state
//! - The selector for picking out the state from the redux state
export * from "./document";
export * from "./init";
export * from "./settings";
export * from "./view";
