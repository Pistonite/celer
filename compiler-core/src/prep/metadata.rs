use serde::{Serialize, Deserialize};

use crate::macros::derive_wasm;
use crate::util::StringMap;

use super::{DocTag, MapMetadata, PrepResult};

/// Config of the route project
#[derive(PartialEq, Default, Serialize, Deserialize, Debug, Clone)]
#[derive_wasm]
pub struct RouteConfig {
    #[serde(flatten)]
    pub meta: RouteMetadata,
    pub map: Option<MapMetadata>,

    /// Arbitrary key-value pairs that can be used for statistics or any other value
    pub stats: StringMap<String>,
    /// Icon id to url map
    pub icons: StringMap<String>,
    /// Tag id to tag
    pub tags: StringMap<DocTag>,
    /// Default tags to split
    pub splits: Vec<String>,
}

#[derive(PartialEq, Default, Serialize, Deserialize, Debug, Clone)]
#[derive_wasm]
pub struct RouteMetadata {
    /// Source of the route, could be a URL or any string
    pub source: String,
    /// Version of the project
    pub version: String,
    /// Display title of the project
    pub title: String,
}

macro_rules! check_metadata_required_property {
    ($property:expr, $obj:ident) => {
        match $obj.remove($property) {
            Some(v) => Ok(v),
            None => Err(PackerError::MissingMetadataProperty($property.to_string())),
        }
    };
}


