use crate::macros::derive_wasm;

use super::Instance;

/// Metadata of a plugin. This is exported and can be
/// used to display plugin information after compilation
#[derive(Debug, Clone, PartialEq)]
#[derive_wasm]
#[serde(rename = "PluginMetadata")]
pub struct Metadata {
    /// Id displayed in the settings to identify the plugin
    pub display_id: String,
    /// If the plugin is from a user plugin config
    pub is_from_user: bool,
    /// If the plugin is enabled. Plugins can be disabled by the user
    pub is_enabled: bool,
}

impl Metadata {
    pub fn new(plugin: &Instance) -> Self {
        Self {
            display_id: plugin.get_display_id().into_owned(),
            is_from_user: false,
            is_enabled: true,
        }
    }

    pub fn new_from_user(plugin: &Instance) -> Self {
        Self {
            display_id: plugin.get_display_id().into_owned(),
            is_from_user: true,
            is_enabled: true,
        }
    }

    pub fn disable(&mut self) {
        self.is_enabled = false;
    }
}
