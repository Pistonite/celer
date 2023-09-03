use std::collections::HashMap;

use celerctypes::{DocTag, MapMetadata};
use serde_json::{Map, Value};

use crate::comp::prop;
use crate::json::{Cast, Coerce};
use crate::lang::Preset;
use crate::util::async_for;
use crate::Setting;

use super::{
    pack_map, pack_presets, PackerError, PackerResult, ResourceLoader, ResourceResolver, Use,
};

#[derive(Default, Debug)]
pub struct ConfigBuilder {
    pub map: Option<MapMetadata>,
    pub icons: HashMap<String, String>,
    pub tags: HashMap<String, DocTag>,
    pub presets: HashMap<String, Preset>,
}

/// Pack a config json blob and apply the values to the [`RouteMetadataBuilder`]
pub async fn pack_config(
    builder: &mut ConfigBuilder,
    resolver: &dyn ResourceResolver,
    loader: &dyn ResourceLoader,
    config: Value,
    index: usize,
    setting: &Setting,
) -> PackerResult<()> {
    // Load and resolve top-level `use` properties
    let config_value = match Use::from(config) {
        Use::Invalid(path) => return Err(PackerError::InvalidUse(path)),
        Use::NotUse(v) => v,
        other => load_config_from_use(resolver, loader, other, index).await?,
    };

    // Resolve `use`s inside the properties
    let config_value = process_config(resolver, loader, config_value, index).await?;

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
                });
            }
            prop::TAGS => {
                let tags = value.try_into_object().map_err(|_| PackerError::InvalidConfigProperty(index, prop::TAGS.to_string()))?;
                async_for!((key, value) in tags.into_iter(), {
                    let tag = serde_json::from_value::<DocTag>(value).map_err(|_| PackerError::InvalidConfigProperty(index, format!("{}.{}", prop::TAGS, key)))?;
                    builder.tags.insert(key, tag);
                });
            }
            prop::PRESETS => {
                let presets = pack_presets(value, index, setting.max_preset_namespace_depth).await?;
                async_for!((key, value) in presets.into_iter(), {
                    builder.presets.insert(key, value);
                });
            }
            _ => return Err(PackerError::UnusedConfigProperty(index, key)),
        }
    });

    Ok(())
}

/// Load a top-level `use`
async fn load_config_from_use(
    resolver: &dyn ResourceResolver,
    loader: &dyn ResourceLoader,
    use_prop: Use,
    index: usize,
) -> PackerResult<Value> {
    let resource = resolver.resolve(&use_prop)?;
    let resource_json = resource.load_json(loader).await?;

    let inner_resolver = resolver.get_resolver(&use_prop)?;
    // Calling process_config here
    // because the config needs to be resolved by the inner resolver
    let config = process_config(inner_resolver.as_ref(), loader, resource_json, index).await?;
    Ok(Value::Object(config))
}

/// Process a config and resolve all `use`s inside
async fn process_config(
    resolver: &dyn ResourceResolver,
    loader: &dyn ResourceLoader,
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
            other => {
                let icon_resource = resolver.resolve(&other)?;
                let image_url = icon_resource.load_image_url(loader).await?;
                *value = Value::String(image_url);
            }
        }
    });

    Ok(config_obj)
}
