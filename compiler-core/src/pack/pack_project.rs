use serde_json::{Value, Map};

use crate::api::{CompilerMetadata, Setting};
use crate::json::{Cast, Coerce};
use crate::prop;
use crate::types::{RouteMetadata, EntryPoints};
use crate::util::async_for;

use super::{ConfigBuilder, PackerError, PackerResult, Resource};

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
pub struct Phase0 {
    pub project: RouteMetadata,
    pub meta: CompilerMetadata,
    pub route: Value,
}

/// Entry point for parsing project.yaml
///
/// Returns the metadata and the route blob with all uses resolved
pub async fn pack_phase0(
    source: &str,
    project_resource: &Resource,
    setting: &Setting,
) -> PackerResult<Phase0> {
    let mut project_obj = load_project_object(project_resource).await?;
    // if entry points is found, also emit error if it is wrong
    if let Some(entry_points) = project_obj.remove(prop::ENTRY_POINTS) {
        super::pack_entry_points(entry_points).await?;
    }

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
    let _ = async_for!((i, config) in config.into_iter().enumerate(), {
        super::pack_config(&mut builder, project_resource, config, i, setting).await?;
    });

    let project = RouteMetadata {
        source: source.to_string(),
        title,
        version,
        stats: Default::default(),
        map: builder.map.ok_or(PackerError::MissingMap)?,
        icons: builder.icons,
        tags: builder.tags,
    };

    let meta = CompilerMetadata {
        presets: builder.presets,
        plugins: builder.plugins,
        default_icon_priority: builder.default_icon_priority.unwrap_or(2),
    };

    Ok(Phase0 {
        project,
        meta,
        route,
    })
}

/// Load the project object and only read the `entry-points` property
pub async fn pack_project_entry_points(project_resource: &Resource) -> PackerResult<EntryPoints> {
    let mut project_obj = load_project_object(project_resource).await?;

    let entry_points_value = match project_obj.remove(prop::ENTRY_POINTS) {
        Some(v) => v,
        None => return Ok(Default::default()),
    };

    super::pack_entry_points(entry_points_value).await
}

async fn load_project_object(project_resource: &Resource) -> PackerResult<Map<String, Value>> {
    match project_resource.load_structured().await? {
        Value::Object(o) => Ok(o),
        _ => {
            Err(PackerError::InvalidResourceType(
                project_resource.name().to_string(),
                "object".to_string(),
            ))
        }
    }
}
