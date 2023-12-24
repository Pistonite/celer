
use serde_json::{Value, json};

use crate::json::Coerce;
use crate::plugin::{BuiltInPlugin, Plugin, ScriptPlugin, PluginInstance};
use crate::res::{Use, Resource, Loader};
use crate::prop;
use crate::prep::{PrepError, PreparedConfig, PrepResult};
use crate::env::yield_budget;

impl PreparedConfig {
    /// Process the `plugins` property
    ///
    /// `use`'s are resolved in the context of `res`
    pub async fn load_plugins<L>(&mut self, res: &Resource<'_, L>, value: Value) -> PrepResult<()> 
    where
        L: Loader,
    {
        let plugins = super::check_array!(self, value, prop::PLUGINS)?;

        // parse each plugin
        for (i, v) in plugins.into_iter().enumerate() {
            yield_budget(16).await;

            let v = super::check_map!(self, v, format!("{}[{}]", prop::PLUGINS, i))?;
            let mut plugin = None;
            let mut props = json!(null);

            // parse properties
            for (key, value) in v.into_iter() {
                yield_budget(16).await;
                match key.as_ref() {
                    prop::USE => {
                        plugin = Some(self.parse_plugin_use(res, value).await?);
                    }
                    prop::WITH => {
                        props = value;
                    }
                    _ => {
                        return Err(PrepError::UnusedConfigProperty(
                            self.trace.clone(),
                            format!("{}[{}].{}", prop::PLUGINS, i, key).into(),
                        ))
                    }
                }
            }

            // check if `use` was specified
            let plugin = plugin.ok_or_else(|| {
                PrepError::MissingConfigProperty(
                    self.trace.clone(),
                    format!("{}[{}].{}", prop::PLUGINS, i, prop::USE).into(),
                )
            })?;

            self.plugins.push(PluginInstance {
                plugin,
                props,
            });
        }

        Ok(())
    }

    /// Parse the `use` property
    async fn parse_plugin_use<L>(&self, res: &Resource<'_, L>, value: Value) -> PrepResult<Plugin> 
    where
        L: Loader,
    {
        let use_path_string = value.coerce_to_string();
        let plugin = match serde_json::from_value::<BuiltInPlugin>(value) {
            Ok(built_in) => Plugin::BuiltIn(built_in),
            Err(_) => {
                // it's a script path, parse as use
                match Use::from(use_path_string) {
                    Use::Invalid(path) => return Err(PrepError::InvalidPlugin(self.trace.clone(), path)),
                    Use::Valid(valid_use) => {
                        // load the script
                        let script_resource = res.resolve(&valid_use)?;
                        let script = script_resource.load_utf8().await?;
                        // TODO #24: do we want to clone here?
                        Plugin::Script(ScriptPlugin {
                            script: script.into_owned(),
                            source: valid_use.to_string(),
                        })
                    }
                }
            }
        };

        Ok(plugin)
        
    }
}

