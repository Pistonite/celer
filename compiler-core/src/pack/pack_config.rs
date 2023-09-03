use std::collections::HashMap;
use tokio_stream::StreamExt;

use celerctypes::{DocTag, MapMetadata};
use serde_json::{Map, Value};

use crate::comp::prop;
use crate::lang::Preset;

use super::{pack_map, PackerError, PackerResult, ResourceLoader, ResourceResolver, Use};

pub struct RouteMetadataBuilder {
    pub map: Option<MapMetadata>,
    pub icons: HashMap<String, String>,
    pub tags: HashMap<String, DocTag>,
    pub presets: HashMap<String, Preset>,
}

/// Pack a config json blob and apply the values to the [`RouteMetadataBuilder`]
pub async fn pack_config(
    builder: &mut RouteMetadataBuilder,
    resolver: &dyn ResourceResolver,
    loader: &dyn ResourceLoader,
    config: Value,
    index: usize,
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
    let mut config_iter = tokio_stream::iter(config_value.into_iter());
    while let Some((key, value)) = config_iter.next().await {
        match key.as_ref() {
            prop::MAP => {
                if builder.map.is_some() {
                    return Err(PackerError::DuplicateMap(index));
                }
                builder.map = Some(pack_map(value, index).await?);
            }
            _ => todo!(),
        }
    }

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

    let mut icons_iter = tokio_stream::iter(icons.values_mut());
    while let Some(value) = icons_iter.next().await {
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
    }

    Ok(config_obj)
}
