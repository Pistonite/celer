use std::collections::BTreeMap;

use serde_json::{json, Value};

use crate::api::Setting;
use crate::json::{Cast, Coerce};
use crate::lang::Preset;
use crate::macros::async_recursion;
use crate::plugin::{BuiltInPlugin, Plugin, PluginInstance};
use crate::prop;
use crate::types::{DocTag, MapMetadata};
use crate::env::yield_budget;
use crate::res::{Use, ValidUse, Loader, Resource};

use super::{ConfigTrace, PackerError, PackerResult};

const MAX_CONFIG_DEPTH: usize = 16;


/// Pack a config json blob and apply the values to the [`ConfigBuilder`]
#[async_recursion(auto)]
pub async fn pack_config<L>(
    builder: &mut ConfigBuilder,
    project_resource: &Resource<'_, '_, L>,
    config: Value,
    trace: &mut ConfigTrace,
    setting: &Setting,
) -> PackerResult<()>  where L: Loader
{
    if trace.len() > MAX_CONFIG_DEPTH {
        return Err(PackerError::MaxConfigDepthExceeded(trace.clone()));
    }
    match Use::try_from(config) {
        Ok(Use::Invalid(path)) => Err(PackerError::InvalidUse(path)),
        Ok(Use::Valid(valid_use)) => {
            // load a config from top-level use object
            process_config_from_use(builder, project_resource, valid_use, trace, setting).await
        }
        Err(v) => {
            // load a config directly from the object
            process_config(builder, project_resource, v, trace, setting).await
        }
    }
}

/// Load a top-level `use`
async fn process_config_from_use<L>(
    builder: &mut ConfigBuilder,
    project_resource: &Resource<'_, '_, L>,
    use_prop: ValidUse,
    trace: &mut ConfigTrace,
    setting: &Setting,
) -> PackerResult<()> where L: Loader
{
    let config_resource = project_resource.resolve(&use_prop)?;
    let config = config_resource.load_structured().await?;
    // process this config with the config resource context instead of the project context
    // so `use`'s inside are resolved correctly
    process_config(builder, &config_resource, config, trace, setting).await
}

/// Process a config, adding values to Builder and use the resource to resolve `use`'s
async fn process_config<L>(
    builder: &mut ConfigBuilder,
    resource: &Resource<'_, '_, L>,
    config: Value,
    trace: &mut ConfigTrace,
    setting: &Setting,
) -> PackerResult<()> where L: Loader
{
    let config = config
        .try_into_object()
        .map_err(|_| PackerError::InvalidConfigType(trace.clone()))?;

    // add values to builder
    for (key, value) in config.into_iter() {
        match key.as_ref() {
            prop::MAP => {
                if builder.map.is_some() {
                    return Err(PackerError::DuplicateMap(trace.clone()));
                }
                builder.map = Some(super::pack_map(value, trace)?);
            }
            prop::ICONS => {
                process_icons_config(builder, resource, value, trace).await?;
            }
            prop::TAGS => {
                process_tags_config(builder, value, trace).await?;
            }
            prop::SPLITS => {
                let splits = value.try_into_array().map_err(|_| {
                    PackerError::InvalidConfigProperty(trace.clone(), prop::SPLITS.to_string())
                })?;
                for split in splits.into_iter() {
                    yield_budget(256).await;
                    builder.splits.push(split.coerce_to_string());
                }
            }
            prop::PRESETS => {
                let presets =
                    super::pack_presets(value, trace, setting.max_preset_namespace_depth).await?;
                for (key, value) in presets.into_iter() {
                    yield_budget(256).await;
                    builder.presets.insert(key, value);
                }
            }
            prop::DEFAULT_ICON_PRIORITY => {
                let priority = value.try_coerce_to_i64().ok_or_else(|| {
                    PackerError::InvalidConfigProperty(
                        trace.clone(),
                        prop::DEFAULT_ICON_PRIORITY.to_string(),
                    )
                })?;
                builder.default_icon_priority = Some(priority);
            }
            prop::PLUGINS => {
                process_plugins_config(builder, resource, value, trace).await?;
            }
            prop::INCLUDES => {
                let config = value.try_into_array().map_err(|_| {
                    PackerError::InvalidConfigProperty(trace.clone(), prop::INCLUDES.to_string())
                })?;
                for (i, config) in config.into_iter().enumerate() {
                    trace.push(i);
                    pack_config(builder, resource, config, trace, setting).await?;
                    trace.pop();
                }
            }
            _ => return Err(PackerError::UnusedConfigProperty(trace.clone(), key)),
        }
    }

    Ok(())
}

/// Process the `plugins` property
///
/// Resolves `use`'s using the resource context and add the plugins to the builder
async fn process_plugins_config<L>(
    builder: &mut ConfigBuilder,
    resource: &Resource<'_, '_, L>,
    plugins: Value,
    trace: &mut ConfigTrace,
) -> PackerResult<()> where L: Loader
{
    let plugins = plugins.try_into_array().map_err(|_| {
        PackerError::InvalidConfigProperty(trace.clone(), prop::PLUGINS.to_string())
    })?;

    for (i, v) in plugins.into_iter().enumerate() {
        let v = v.try_into_object().map_err(|_| {
            PackerError::InvalidConfigProperty(trace.clone(), format!("{}[{}]", prop::PLUGINS, i))
        })?;
        let mut plugin = None;
        let mut props = json!(null);
        for (key, value) in v.into_iter() {
            match key.as_ref() {
                prop::USE => {
                    let use_path_string = value.coerce_to_string();
                    plugin = match serde_json::from_value::<BuiltInPlugin>(value) {
                        Ok(built_in) => Some(Plugin::BuiltIn(built_in)),
                        Err(_) => {
                            // it's a script path, parse as use
                            match Use::from(use_path_string) {
                                Use::Invalid(path) => return Err(PackerError::InvalidPlugin(path)),
                                Use::Valid(valid_use) => {
                                    // load the script
                                    let script_resource = resource.resolve(&valid_use)?;
                                    let script = script_resource.load_utf8().await?;
                                    // TODO #24: do we want to clone here?
                                    Some(Plugin::UncompiledScript(script.into_owned()))
                                }
                            }
                        }
                    };
                }
                prop::WITH => {
                    props = value;
                }
                _ => {
                    return Err(PackerError::UnusedConfigProperty(
                        trace.clone(),
                        format!("{}[{}].{}", prop::PLUGINS, i, key),
                    ))
                }
            }
        }
        let plugin = match plugin {
            Some(v) => v,
            None => {
                return Err(PackerError::MissingConfigProperty(
                    trace.clone(),
                    format!("{}[{}].{}", prop::PLUGINS, i, prop::USE),
                ))
            }
        };
        builder.plugins.push(PluginInstance { plugin, props });
    }

    Ok(())
}

