use serde_json::Value;

use crate::json::Cast;
use crate::macros::derive_wasm;
use crate::res::{Loader, Resource};

use super::{PluginError, PluginInstance, PluginResult};

/// Options for users to alter plugins defined in the route
#[derive(Debug, Clone, Default)]
pub struct PluginOptions {
    /// List of plugin ids to remove
    pub remove: Vec<String>,
    /// List of plugins to add
    pub add: Vec<PluginInstance>,
}

#[derive(Debug, Clone, Default)]
#[derive_wasm]
pub struct PluginOptionsRaw {
    /// List of plugin ids to remove
    pub remove: Vec<String>,
    /// List of plugins to add
    pub add: Vec<Value>,
}

impl PluginOptionsRaw {
    pub async fn parse<L>(self, res: &Resource<'_, L>) -> PluginResult<PluginOptions>
    where
        L: Loader,
    {
        let mut options = PluginOptions {
            remove: self.remove,
            add: Vec::with_capacity(self.add.len()),
        };
        for (i, v) in self.add.into_iter().enumerate() {
            let v = match v.try_into_object() {
                Ok(v) => v,
                Err(_) => {
                    return Err(PluginError::InvalidAddPlugin(
                        i,
                        "option must be a mapping object".to_string(),
                    ))
                }
            };
            let plugin = match super::parse_plugin_instance(v, res).await {
                Ok(plugin) => plugin,
                Err(e) => {
                    return Err(PluginError::InvalidAddPlugin(i, e.to_string()));
                }
            };
            options.add.push(plugin);
        }

        Ok(options)
    }
}
