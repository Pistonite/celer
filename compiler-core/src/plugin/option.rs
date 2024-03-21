//! Plugin options are for users to alter plugins defined in the route
//!
//! These options are fed to the compiler in the pack phase, and
//! merged with the existing plugins in the route.


use log::Metadata;
use serde_json::Value;

use crate::json::Cast;
use crate::macros::derive_wasm;
use crate::res::{Loader, Resource};

use super::{PluginError, PluginInstance, PluginResult};

/// Raw options value passed in from the client
#[derive(Debug, Clone, Default)]
#[derive_wasm]
#[serde(rename = "PluginOptions")]
pub struct OptionsRaw {
    /// Expected plugin ids to apply the options to
    pub route_plugin_ids: Vec<String>,

    /// Indices of plugins to remove.
    ///
    /// The indices should be conceptually to a list of route plugins + list of user plugins
    pub remove: Vec<u32>,

    /// List of user plugins to add. Same spec as the `plugins` section in config
    pub add: Vec<Value>,
}

impl PluginOptionsRaw {
    pub async fn parse<L>(self, res: &Resource<'_, L>) -> PluginResult<PluginOptions>
    where
        L: Loader,
    {
        let mut options = PluginOptions {
            ids: self.remove_from,
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

/// Parsed options
#[derive(Debug, Clone, Default)]
pub struct Options {
    route_plugin_ids: Vec<String>,
    remove: Vec<u32>,
    add: Vec<PluginInstance>,
}

impl Options {
    pub fn apply(self, route_plugins: &[PluginInstance]) -> (Vec<PluginInstance>, Vec<Metadata>) {
        
    }
}
