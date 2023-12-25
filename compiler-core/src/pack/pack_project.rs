use serde_json::{Map, Value};

// use crate::api::{CompilerMetadata, Setting};
use crate::json::{Cast, Coerce};
use crate::prop;
// use crate::resource::{Resource, Loader};
// use crate::types::{EntryPoints, RouteMetadata};
// use crate::util::yield_budget;

// use super::{ConfigBuilder, ConfigTrace, PackerError, PackerResult};

// macro_rules! check_metadata_not_array_or_object {
//     ($property:expr, $value:ident) => {{
//         if $value.is_array() || $value.is_object() {
//             Err(PackerError::InvalidMetadataPropertyType(
//                 $property.to_string(),
//             ))
//         } else {
//             Ok($value.coerce_to_string())
//         }
//     }};
// }
//
//
// /// Result of packing a project
// pub struct Phase0 {
//     pub project: RouteMetadata,
//     pub meta: CompilerMetadata,
//     pub route: Value,
// }

// /// Pack the project after loading the project object
// async fn pack_project<L>(
//     source: &str,
//     project_resource: &Resource<'_, '_, L>,
//     mut project_obj: Map<String, Value>,
//     setting: &Setting,
// ) -> PackerResult<Phase0>
// where L: Loader
// {
//     let title = check_metadata_required_property!(prop::TITLE, project_obj)?;
//     let version = check_metadata_required_property!(prop::VERSION, project_obj)?;
//     let route = check_metadata_required_property!(prop::ROUTE, project_obj)?;
//     let config = check_metadata_required_property!(prop::CONFIG, project_obj)?;
//
//     if let Some(k) = project_obj.keys().next() {
//         return Err(PackerError::UnusedMetadataProperty(k.to_string()));
//     }
//
//     let title = check_metadata_not_array_or_object!(prop::TITLE, title)?;
//     let version = check_metadata_not_array_or_object!(prop::VERSION, version)?;
//     let config = config
//         .try_into_array()
//         .map_err(|_| PackerError::InvalidMetadataPropertyType(prop::CONFIG.to_string()))?;
//
// }
