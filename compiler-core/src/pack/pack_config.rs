use std::collections::HashMap;

use celerctypes::{DocTag, MapMetadata};
use serde_json::{Map, Value};

use crate::api::Setting;
use crate::comp::prop;
use crate::json::{Cast, Coerce};
use crate::lang::Preset;
use crate::util::async_for;

use super::{pack_map, pack_presets, PackerError, PackerResult, Resource, Use, ValidUse};

#[derive(Default, Debug)]
pub struct ConfigBuilder {
    pub map: Option<MapMetadata>,
    pub icons: HashMap<String, String>,
    pub tags: HashMap<String, DocTag>,
    pub presets: HashMap<String, Preset>,
    pub default_icon_priority: Option<i64>,
}

/// Pack a config json blob and apply the values to the [`RouteMetadataBuilder`]
pub async fn pack_config(
    builder: &mut ConfigBuilder,
    project_resource: &Resource,
    config: Value,
    index: usize,
    setting: &Setting,
) -> PackerResult<()> {
    // Load and resolve top-level `use` properties
    let config_value = match Use::from(config) {
        Use::Invalid(path) => return Err(PackerError::InvalidUse(path)),
        Use::NotUse(v) => v,
        Use::Valid(valid_use) => load_config_from_use(project_resource, valid_use, index).await?,
    };

    // Resolve `use`s inside the properties
    let config_value = process_config(project_resource, config_value, index).await?;

    // add values to builder
    async_for!((key, value) in config_value.into_iter(), {
        match key.as_ref() {
            prop::MAP => {
                if builder.map.is_some() {
                    return Err(PackerError::DuplicateMap(index));
                }
                builder.map = Some(pack_map(value, index).await?);
            }
            prop::ICONS => {
                let icons = value.try_into_object().map_err(|_| PackerError::InvalidConfigProperty(index, prop::ICONS.to_string()))?;
                async_for!((key, value) in icons.into_iter(), {
                    if value.is_array() || value.is_object() {
                        return Err(PackerError::InvalidConfigProperty(index, format!("{}.{}", prop::ICONS, key)));
                    }
                    builder.icons.insert(key, value.coerce_to_string());
                })?;
            }
            prop::TAGS => {
                let tags = value.try_into_object().map_err(|_| PackerError::InvalidConfigProperty(index, prop::TAGS.to_string()))?;
                async_for!((key, value) in tags.into_iter(), {
                    let tag = serde_json::from_value::<DocTag>(value).map_err(|_| PackerError::InvalidConfigProperty(index, format!("{}.{}", prop::TAGS, key)))?;
                    builder.tags.insert(key, tag);
                })?;
            }
            prop::PRESETS => {
                let presets = pack_presets(value, index, setting.max_preset_namespace_depth).await?;
                async_for!((key, value) in presets.into_iter(), {
                    builder.presets.insert(key, value);
                })?;
            }
            prop::DEFAULT_ICON_PRIORITY => {
                let priority = value.try_coerce_to_i64().ok_or_else(|| PackerError::InvalidConfigProperty(index, prop::DEFAULT_ICON_PRIORITY.to_string()))?;
                builder.default_icon_priority = Some(priority);
            }
            _ => return Err(PackerError::UnusedConfigProperty(index, key)),
        }
    })?;

    Ok(())
}

/// Load a top-level `use`
async fn load_config_from_use(
    project_resource: &Resource,
    use_prop: ValidUse,
    index: usize,
) -> PackerResult<Value> {
    let config_resource = project_resource.resolve(&use_prop).await?;
    let config = config_resource.load_structured().await?;
    // Calling process_config here
    // because any `use` inside the config needs to be resolved by the config resource
    // not the project resource
    let config = process_config(&config_resource, config, index).await?;
    Ok(Value::Object(config))
}

/// Process a config and resolve all `use`s inside
async fn process_config(
    resource: &Resource,
    config: Value,
    index: usize,
) -> PackerResult<Map<String, Value>> {
    let mut config_obj = match config {
        Value::Object(obj) => obj,
        _ => return Err(PackerError::InvalidConfigType(index)),
    };

    let icons = match config_obj.get_mut(prop::ICONS) {
        Some(v) => v,
        None => return Ok(config_obj),
    };

    let icons = match icons.as_object_mut() {
        Some(obj) => obj,
        // just returning ok here
        // the error will be caught later
        _ => return Ok(config_obj),
    };

    async_for!(value in icons.values_mut(), {
        let v = value.take();
        match Use::from(v) {
            Use::Invalid(path) => return Err(PackerError::InvalidUse(path)),
            Use::NotUse(v) => {
                *value = v;
            }
            Use::Valid(valid_use) => {
                let icon_resource = resource.resolve(&valid_use).await?;
                let image_url = icon_resource.load_image_url().await?;
                *value = Value::String(image_url);
            }
        }
    })?;

    Ok(config_obj)
}
