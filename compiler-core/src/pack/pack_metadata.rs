use celerctypes::RouteMetadata;
use serde_json::Value;

use super::Resource;

/// Parse the metadata from project resource
pub async fn pack_metadata(project: &dyn Resource) -> Result<PackMetadata, MetadataError> {
    todo!()
}

pub async fn inspect_metadata(project: &dyn Resource) -> Result<PackMetadata, MetadataError> {
    todo!()
}

/// Packed metadata
#[derive(Debug, Clone, PartialEq)]
pub struct PackMetadata {
    /// The metadata
    pub project: RouteMetadata,
    /// The unparsed route blob
    pub route: Value,
}

/// Metadata inspect result
pub struct InspectMetadata {
    pub title: String,
    pub version: String,
}

/// Error when reading metadata of a route
#[derive(Debug, Clone, PartialEq)]
pub enum MetadataError {
    /// Missing a required property
    MissingProperty(String)
}
