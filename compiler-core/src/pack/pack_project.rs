use celerctypes::RouteMetadata;
use serde_json::Value;

use crate::api::{CompilerMetadata, Setting};
use crate::comp::prop;
use crate::json::{Cast, Coerce};
use crate::util::async_for;

use super::{
    pack_config, pack_route, ConfigBuilder, PackerError, PackerResult, PackerValue, ResourceLoader,
    ResourcePath, ResourceResolver, Resource,
};

macro_rules! check_metadata_not_array_or_object {
    ($property:expr, $value:ident) => {{
        if $value.is_array() || $value.is_object() {
            Err(PackerError::InvalidMetadataPropertyType(
                $property.to_string(),
            ))
        } else {
            Ok($value.coerce_to_string())
        }
    }};
}

macro_rules! check_metadata_required_property {
    ($property:expr, $obj:ident) => {
        match $obj.remove($property) {
            Some(v) => Ok(v),
            None => Err(PackerError::MissingMetadataProperty($property.to_string())),
        }
    };
}

/// Result of packing a project
pub struct PackedProject {
    pub route_metadata: RouteMetadata,
    pub compiler_metadata: CompilerMetadata,
    pub route: PackerValue,
}

/// Entry point for parsing project.yaml
///
/// Returns the metadata and the route blob with all uses resolved
pub async fn pack_project(
    project_resource: &Resource,
    setting: &Setting,
) -> PackerResult<PackedProject> {
    let mut project_obj = match project_resource.load_structured().await? {
        Value::Object(o) => o,
        _ => {
            return Err(PackerError::InvalidResourceType(
                project_resource.name().to_string(),
                "object".to_string(),
            ))
        }
    };

    let title = check_metadata_required_property!(prop::TITLE, project_obj)?;
    let version = check_metadata_required_property!(prop::VERSION, project_obj)?;
    let route = check_metadata_required_property!(prop::ROUTE, project_obj)?;
    let config = check_metadata_required_property!(prop::CONFIG, project_obj)?;

    if let Some(k) = project_obj.keys().next() {
        return Err(PackerError::UnusedMetadataProperty(k.to_string()));
    }

    let title = check_metadata_not_array_or_object!(prop::TITLE, title)?;
    let version = check_metadata_not_array_or_object!(prop::VERSION, version)?;
    let config = config
        .try_into_array()
        .map_err(|_| PackerError::InvalidMetadataPropertyType(prop::CONFIG.to_string()))?;

    let mut builder = ConfigBuilder::default();
    async_for!((i, config) in config.into_iter().enumerate(), {
        pack_config(&mut builder, project_resource, config, i, setting).await?;
    });

    let route_metadata = RouteMetadata {
        name: project_resource.name().to_string(),
        title,
        version,
        map: builder.map.ok_or(PackerError::MissingMap)?,
        icons: builder.icons,
        tags: builder.tags,
    };

    let compiler_metadata = CompilerMetadata {
        presets: builder.presets,
        default_icon_priority: builder.default_icon_priority.unwrap_or(2),
    };

    let route = pack_route(
        project_resource,
        route,
        setting.max_use_depth,
        setting.max_ref_depth,
    )
    .await;

    Ok(PackedProject {
        route_metadata,
        compiler_metadata,
        route,
    })
}
