use std::borrow::Cow;

use serde_json::{json, Map, Value};

use crate::CompileContext;
use crate::env::yield_budget;
use crate::json::Coerce;
use crate::prop;
use crate::res::{Loader, ResError, Resource, Use};

use super::{BoxedEarlyRuntime, PluginResult, BoxedRuntime};
use super::native::Native;
use super::script::Script;

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

/// An instance of a plugin read from the config file, with a source where the plugin can be loaded
/// from and properties to pass into the plugin
#[derive(Debug, Clone)]
pub struct Instance {
    /// The plugin definition
    plugin: PluginUse,
    /// Props passed to the plugin
    props: Value,
    /// The `allow-duplicate` property. By default, plugins are not allowed to be duplicated
    /// unless this is false. However, plugins don't need to respect this property
    /// and can implement their own load logic in an early plugin
    pub allow_duplicate: bool,
}

impl Instance {
    pub fn new(plugin: PluginUse, props: Value, allow_duplicate: bool) -> Self {
        Self {
            plugin,
            props,
            allow_duplicate,
        }
    }

    pub fn create_early_runtime(&self) -> PluginResult<BoxedEarlyRuntime> {
        match &self.plugin {
            PluginUse::Native(p) => p.create_early_runtime(),
            PluginUse::Script(p) => p.create_early_runtime(),
        }
    }

    pub fn create_runtime(&self, ctx: &CompileContext<'_>) -> PluginResult<BoxedRuntime> {
        match &self.plugin {
            PluginUse::Native(p) => p.create_runtime(ctx, &self.props),
            PluginUse::Script(p) => p.create_runtime(ctx, &self.props),
        }
    }

    pub fn get_id(&self) -> Cow<'_, str> {
        match &self.plugin {
            PluginUse::Native(p) => Cow::Owned(p.id()),
            PluginUse::Script(p) => Cow::Borrowed(&p.id),
        }
    }

    pub fn get_display_id(&self) -> Cow<'_, str> {
        match &self.plugin {
            PluginUse::Native(p) => Cow::Owned(p.id()),
            PluginUse::Script(p) => Cow::Owned(p.get_display_name()),
        }
    }

/// Parse a plugin instance (one element in the `plugins` array)
pub async fn parse<L>(
    value: Map<String, Value>,
    res: &Resource<'_, L>,
) -> Result<Self, PluginParseError>
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
                plugin = Some(PluginUse::parse(res, value).await?);
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

    Ok(Self {
        plugin,
        allow_duplicate,
        props,
    })
}
}

/// Definition of a plugin read from the `use` property in the config file
#[derive(Debug, Clone)]
pub enum PluginUse {
    /// A native plugin
    Native(Native),
    /// A script that is downloaded but not parsed.
    Script(Script),
}

impl PluginUse {
    /// Parse from the `use` property
    pub async fn parse<L>(
        res: &Resource<'_, L>,
        value: Value,
    ) -> Result<Self, PluginParseError>
where
        L: Loader,
    {
        let use_path_string = value.coerce_to_string();
        let plugin = match serde_json::from_value::<Native>(value) {
            Ok(native) => Self::Native(native),
            Err(_) => {
                // it's a script path, parse as use
                match Use::new(use_path_string) {
                    Use::Invalid(path) => {
                        return Err(PluginParseError::InvalidPlugin(path));
                    }
                    Use::Valid(valid_use) => {
                        // load the script
                        // TODO #24: put this in the script module
                        let script_resource = res.resolve(&valid_use)?;
                        let script = script_resource.load_utf8().await?;
                        Self::Script(Script {
                            id: script_resource.path().to_string(),
                            script: script.to_string(),
                        })
                    }
                }
            }
        };

        Ok(plugin)
    }
}
