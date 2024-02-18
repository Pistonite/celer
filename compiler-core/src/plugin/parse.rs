//! Parsing plugin configurations

use serde_json::{json, Map, Value};

use crate::env::yield_budget;
use crate::json::Coerce;
use crate::prop;
use crate::res::{Loader, ResError, Resource, Use};

use super::{BuiltInPlugin, Plugin, PluginInstance, ScriptPlugin};

#[derive(Debug, thiserror::Error)]
pub enum PluginParseError {
    #[error("Missing `use` property in plugin config")]
    MissingPlugin,
    #[error("The `use` property for the plugin is invalid: {0}")]
    InvalidPlugin(String),
    #[error("Unused property in plugin config: {0}")]
    UnusedProperty(String),
    #[error("Error loading plugin: {0}")]
    LoadError(#[from] ResError),
}

/// Parse a plugin instance (one element in the `plugins` array)
pub async fn parse_plugin_instance<L>(
    value: Map<String, Value>,
    res: &Resource<'_, L>,
) -> Result<PluginInstance, PluginParseError>
where
    L: Loader,
{
    let mut plugin = None;
    let mut props = json!(null);
    let mut allow_duplicate = false;

    // parse properties
    for (key, value) in value.into_iter() {
        yield_budget(16).await;
        match key.as_ref() {
            prop::USE => {
                plugin = Some(parse_plugin_use(res, value).await?);
            }
            prop::WITH => {
                props = value;
            }
            prop::ALLOW_DUPLICATE => {
                allow_duplicate = value.coerce_truthy();
            }
            _ => {
                return Err(PluginParseError::UnusedProperty(key));
            }
        }
    }

    // check if `use` was specified
    let plugin = plugin.ok_or(PluginParseError::MissingPlugin)?;

    Ok(PluginInstance {
        plugin,
        allow_duplicate,
        props,
    })
}

/// Parse the `use` property
async fn parse_plugin_use<L>(
    res: &Resource<'_, L>,
    value: Value,
) -> Result<Plugin, PluginParseError>
where
    L: Loader,
{
    let use_path_string = value.coerce_to_string();
    let plugin = match serde_json::from_value::<BuiltInPlugin>(value) {
        Ok(built_in) => Plugin::BuiltIn(built_in),
        Err(_) => {
            // it's a script path, parse as use
            match Use::new(use_path_string) {
                Use::Invalid(path) => {
                    return Err(PluginParseError::InvalidPlugin(path));
                }
                Use::Valid(valid_use) => {
                    // load the script
                    let script_resource = res.resolve(&valid_use)?;
                    let script = script_resource.load_utf8().await?;
                    Plugin::Script(ScriptPlugin {
                        id: script_resource.path().to_string(),
                        script
                    })
                }
            }
        }
    };

    Ok(plugin)
}
