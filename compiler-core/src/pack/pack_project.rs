use serde_json::{Map, Value};

use crate::api::{CompilerMetadata, Setting};
use crate::json::{Cast, Coerce};
use crate::prop;
use crate::resource::{Resource, Loader};
use crate::types::{EntryPoints, RouteMetadata};
use crate::util::yield_budget;

use super::{ConfigBuilder, ConfigTrace, PackerError, PackerResult};

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
/// Returns the metadata and the raw route blob
///
/// If `redirect_default_entry_point` is true, the function will redirect to the default entry point
/// if it is defined. The redirect will only happen once, and the redirected project will have its
/// `entry-points` property removed and ignored.
pub async fn pack_phase0<L>(
    source: &str,
    project_resource: &Resource<'_, '_, L>,
    setting: &Setting,
    redirect_default_entry_point: bool,
) -> PackerResult<Phase0> 
where L: Loader
{
    let mut project_obj = load_project_object(project_resource).await?;

    // if entry points is found, redirect to the default entry point if it exists,
    // if entry points are invalid, also emit the error
    if let Some(entry_points) = project_obj.remove(prop::ENTRY_POINTS) {
        let mut entry_points = super::pack_entry_points(entry_points).await?;
        if redirect_default_entry_point {
            if let Some(entry_path) = entry_points.0.remove(prop::DEFAULT) {
                let redirect_resource =
                    crate::resolve_absolute(project_resource, entry_path).await?;
                let mut project_obj = load_project_object(&redirect_resource).await?;
                // remove and ignore the entry points in the redirected project
                project_obj.remove(prop::ENTRY_POINTS);
                return pack_project(source, &redirect_resource, project_obj, setting).await;
            }
        }
    }

    pack_project(source, project_resource, project_obj, setting).await
}

/// Pack the project after loading the project object
async fn pack_project<L>(
    source: &str,
    project_resource: &Resource<'_, '_, L>,
    mut project_obj: Map<String, Value>,
    setting: &Setting,
) -> PackerResult<Phase0>
where L: Loader
{
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
    let mut config_trace = ConfigTrace::default();
    for (i, config) in config.into_iter().enumerate() {
        yield_budget(64).await;
        config_trace.0.push(i);
        super::pack_config(
            &mut builder,
            project_resource,
            config,
            &mut config_trace,
            setting,
        )
        .await?;
        config_trace.0.pop();
    }

    let project = RouteMetadata {
        source: source.to_string(),
        title,
        version,
        stats: Default::default(),
        map: builder.map.ok_or(PackerError::MissingMap)?,
        icons: builder.icons,
        tags: builder.tags,
        splits: builder.splits,
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
pub async fn pack_project_entry_points<L>( project_resource: &Resource<'_, '_, L>) -> PackerResult<EntryPoints> 
where L: Loader
{
    let mut project_obj = load_project_object(project_resource).await?;

    let entry_points_value = match project_obj.remove(prop::ENTRY_POINTS) {
        Some(v) => v,
        None => return Ok(Default::default()),
    };

    super::pack_entry_points(entry_points_value).await
}

async fn load_project_object<L>(project_resource: &Resource<'_, '_, L>) -> PackerResult<Map<String, Value>> 
where L: Loader
{
    match project_resource.load_structured().await? {
        Value::Object(o) => Ok(o),
        _ => Err(PackerError::InvalidResourceType(
            project_resource.path().to_string(),
            "object".to_string(),
        )),
    }
}
