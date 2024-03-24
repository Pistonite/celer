//! Plugin options are for users to alter plugins defined in the route
//!
//! These options are fed to the compiler in the pack phase, and
//! merged with the existing plugins in the route.

use serde_json::Value;

use crate::json::Cast;
use crate::macros::derive_wasm;
use crate::res::{Loader, Resource};

use super::{Instance, Metadata, PluginError, PluginResult};

/// Raw options value passed in from the client
#[derive(Debug, Clone, Default)]
#[derive_wasm]
#[serde(rename = "PluginOptions")]
pub struct OptionsRaw {
    /// Expected plugin display ids to apply the options to
    #[serde(rename = "routePluginIds")]
    pub route_plugin_ids: Vec<String>,

    /// Indices of plugins to remove.
    ///
    /// The indices should be conceptually to a list of route plugins + list of user plugins
    pub remove: Vec<u32>,

    /// List of user plugins to add. Same spec as the `plugins` section in config
    pub add: Vec<Value>,
}

impl OptionsRaw {
    pub async fn parse<L>(self, res: &Resource<'_, L>) -> PluginResult<Options>
    where
        L: Loader,
    {
        let mut add = Vec::with_capacity(self.add.len());
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
            let plugin = match Instance::parse(v, res).await {
                Ok(plugin) => plugin,
                Err(e) => {
                    return Err(PluginError::InvalidAddPlugin(i, e.to_string()));
                }
            };
            add.push(plugin);
        }

        Ok(Options {
            route_plugin_ids: self.route_plugin_ids,
            remove: self.remove,
            add,
        })
    }
}

/// Parsed plugin options
#[derive(Debug, Clone, Default)]
pub struct Options {
    route_plugin_ids: Vec<String>,
    remove: Vec<u32>,
    add: Vec<Instance>,
}

impl Options {
    pub fn new(route_plugin_ids: Vec<String>, remove: &[u32], add: Vec<Instance>) -> Self {
        Self {
            route_plugin_ids,
            remove: remove.to_vec(),
            add,
        }
    }

    pub fn apply_none(route_plugins: &[Instance]) -> OptionsApply {
        OptionsApply {
            metadata: route_plugins.iter().map(Metadata::new).collect(),
            user_plugins: vec![],
        }
    }

    pub fn apply(self, route_plugins: &[Instance]) -> OptionsApply {
        // if expected ids and actual ids are not equal, we don't remove any route plugins
        // but we still remove user plugins
        let can_remove_route_plugins = self.are_ids_equal(route_plugins);

        // pre allocate upper bound
        let mut metadata = Vec::with_capacity(route_plugins.len() + self.add.len());

        // populate metadata
        for p in route_plugins {
            metadata.push(Metadata::new(p));
        }
        for p in &self.add {
            metadata.push(Metadata::new_from_user(p));
        }

        let offset = self.route_plugin_ids.len();
        for i in self.remove.iter().copied() {
            if (i as usize) < offset && !can_remove_route_plugins {
                continue;
            }
            if let Some(m) = metadata.get_mut(i as usize) {
                m.disable();
            }
        }

        let mut user_plugins = Vec::with_capacity(self.add.len());
        for (i, p) in self.add.into_iter().enumerate() {
            if let Some(m) = metadata.get(i + offset) {
                if !m.is_enabled {
                    continue;
                }
            }
            user_plugins.push(p);
        }

        OptionsApply {
            metadata,
            user_plugins,
        }
    }

    fn are_ids_equal(&self, route_plugins: &[Instance]) -> bool {
        if self.route_plugin_ids.len() != route_plugins.len() {
            return false;
        }
        for (actual, expected) in route_plugins
            .iter()
            .map(|p| p.get_display_id())
            .zip(self.route_plugin_ids.iter())
        {
            let actual: &str = actual.as_ref();
            if actual != expected {
                return false;
            }
        }

        true
    }
}

/// Options that are checked and applied
#[derive(Debug, Clone, Default)]
pub struct OptionsApply {
    /// Metadata of all plugins, including route and user plugins and disabled ones
    pub metadata: Vec<Metadata>,
    /// User plugins to add, excluding disabled ones
    pub user_plugins: Vec<Instance>,
}
