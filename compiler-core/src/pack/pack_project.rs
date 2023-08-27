use celerctypes::RouteMetadata;
use serde_json::Value;

use super::{Resource, PackerError, PackerResult, ResourceResolver, ResourceLoader};

use crate::comp::prop;
use crate::json::Coerce;

macro_rules! check_metadata_not_array_or_object {
    ($property:expr, $value:ident) => {
        {
            if $value.is_array() || $value.is_object() {
                Err(PackerError::InvalidMetadataPropertyType($property.to_string()))
            } else {
                Ok($value.coerce_to_string())
            }
        }
    }
}

macro_rules! check_metadata_required_property {
    ($property:expr, $value:ident) => {
        match $value {
            Some(v) => Ok(v),
            None => Err(PackerError::MissingMetadataProperty($property.to_string())),
        }
    }
}

/// Entry point for parsing project.yaml
/// 
/// Returns the metadata and the route blob with all uses resolved
pub async fn pack_project(
    project: &dyn Resource,
    resolver: &dyn ResourceResolver,
    loader: &dyn ResourceLoader,
) -> PackerResult<(RouteMetadata, Value)> {
    let project_json = project.load_json(loader).await?;
    let project_obj = match project_json {
        Value::Object(o) => o,
        _ => return Err(PackerError::InvalidResourceType(project.name(), "object".to_string())),
    };

    let mut title = None;
    let mut version = None;
    let mut route = None;
    let mut config = None;

    for (key, value) in project_obj.into_iter() {
        match key.as_str() {
            prop::TITLE => {
                title = Some(check_metadata_not_array_or_object!(prop::TITLE, value)?);
            }
            prop::VERSION => {
                version = Some(check_metadata_not_array_or_object!(prop::VERSION, value)?);
            }
            prop::ROUTE => {
                route = match value {
                    Value::Array(a) => Some(a),
                    _ => return Err(PackerError::InvalidMetadataPropertyType(prop::ROUTE.to_string())),
                }
            }
            prop::CONFIG => {
                config = match value {
                    Value::Array(a) => Some(a),
                    _ => return Err(PackerError::InvalidMetadataPropertyType(prop::CONFIG.to_string())),
                }
            }
            _ => return Err(PackerError::UnusedMetadataProperty(key)),
        }
    }

    let title = check_metadata_required_property!(prop::TITLE, title)?;
    let version = check_metadata_required_property!(prop::VERSION, version)?;
    let route = check_metadata_required_property!(prop::ROUTE, route)?;
    let config = check_metadata_required_property!(prop::CONFIG, config)?;

    todo!()
}
