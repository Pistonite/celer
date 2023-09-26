//! # Packer
//!
//! The packer is the first step of compiling a route.
//! It takes a project from a resource context, processes the metadata,
//! and resolves any `use` property defined in the route or metadata.
//!
//! The output of the packer is a [`RouteMetadata`](celerctypes::RouteMetadata)
//! and a json blob of the route.

use std::collections::BTreeMap;
use std::convert::Infallible;
use serde_json::{Map, Value};

use celerctypes::DocDiagnostic;
mod pack_config;
pub use pack_config::*;
mod pack_coord_map;
pub use pack_coord_map::*;
mod pack_map;
pub use pack_map::*;
mod pack_map_layer;
pub use pack_map_layer::*;
mod pack_preset;
pub use pack_preset::*;
mod pack_project;
pub use pack_project::*;
mod pack_route;
pub use pack_route::*;
mod pack_use;
pub use pack_use::*;
mod resource;
pub use resource::*;

use crate::json::Cast;
use crate::lang::parse_poor;
#[cfg(feature = "wasm")]
use crate::util::{WasmError, Path};

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum PackerError {
    #[error("The project file (project.yaml) is missing or invalid.")]
    InvalidProject,

    #[error("Invalid `use` value: {0}. If you are specifying a relative path, make sure to start with ./ or ../")]
    InvalidUse(String),

    #[error("Invalid path: {0}")]
    InvalidPath(String),

    #[error("Max depth of {0} levels of `use` is reached. Please make sure there are no circular dependencies.")]
    MaxUseDepthExceeded(usize),

    #[error("Max reference depth of {0} levels is reached. There might be a formatting error in your project files.")]
    MaxRefDepthExceeded(usize),

    #[error("Max preset namespace depth of {0} levels is reached. There might be a formatting error in your project files. If this is intentional, consider making the namespaces less complex.")]
    MaxPresetNamespaceDepthExceeded(usize),

    #[error("The format of resource {0} cannot be determined")]
    UnknownFormat(String),

    #[error("Cannot load file: {0}")]
    LoadFile(String),

    #[error("Cannot load url: {0}")]
    LoadUrl(String),

    #[error("Error when parsing structured data in file {0}: {1}")]
    InvalidFormat(String, String),

    #[error("")]
    InvalidIcon,

    #[error("Resource type is invalid: {0} should be of type {1}")]
    InvalidResourceType(String, String),

    #[error("Project metadata is missing a required property: {0}")]
    MissingMetadataProperty(String),

    #[error("Project property {0} has invalid type")]
    InvalidMetadataPropertyType(String),

    #[error("Project metadata has extra unused property: {0}")]
    UnusedMetadataProperty(String),

    #[error("Project config at index {0} has an invalid type")]
    InvalidConfigType(usize),

    #[error("Project config at index {0}: the `{1}` property is invalid")]
    InvalidConfigProperty(usize, String),

    #[error("Project config at index {0}: the required `{1}` property is missing")]
    MissingConfigProperty(usize, String),

    #[error("Project config at index {0}: the `{1}` property is unused")]
    UnusedConfigProperty(usize, String),

    #[error("Project config at index {0}: The preset {1} is invalid")]
    InvalidPreset(usize, String),

    #[error(
        "Project config at index {0}: defining map when a previous config already defines one"
    )]
    DuplicateMap(usize),

    #[error("No map defined in project config")]
    MissingMap,

    #[error("Image resource {0} has exceeded the size limit of {1}")]
    ImageTooBig(String, String),

    #[error("{0}")]
    NotImpl(String),

    #[cfg(feature = "wasm")]
    #[error("Wasm execution error: {0}")]
    Wasm(#[from] WasmError),
}

impl PackerError {
    pub fn add_to_diagnostics(&self, output: &mut Vec<DocDiagnostic>) {
        output.push(DocDiagnostic {
            msg: parse_poor(&self.to_string()),
            msg_type: "error".to_string(),
            source: "celerc/packer".to_string(),
        });
    }

    pub fn is_cancel(&self) -> bool {
        #[cfg(feature = "wasm")]
        let x = matches!(self, Self::Wasm(WasmError::Cancel));
        #[cfg(not(feature = "wasm"))]
        let x = false;
        x
    }
}

impl From<Infallible> for PackerError {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}

pub type PackerResult<T> = Result<T, PackerError>;

/// JSON value with an Err variant
///
/// This is used to expose errors to the compiler, so it can be displayed
/// using the diagnostics API
#[derive(Debug, PartialEq)]
pub enum PackerValue {
    Ok(Value),
    Err(PackerError),
    Array(Vec<PackerValue>),
    Object(BTreeMap<String, PackerValue>),
}

impl Cast for PackerValue {
    type Object = BTreeMap<String, PackerValue>;

    fn try_into_object(self) -> Result<<PackerValue as Cast>::Object, Self> {
        match self {
            Self::Ok(v) => match v.try_into_object() {
                Ok(v) => {
                    let mut new_obj = BTreeMap::new();
                    for (key, value) in v.into_iter() {
                        new_obj.insert(key, Self::Ok(value));
                    }
                    Ok(new_obj)
                }
                Err(v) => Err(Self::Ok(v)),
            },
            Self::Object(v) => Ok(v),
            _ => Err(self),
        }
    }

    fn try_into_array(self) -> Result<Vec<Self>, Self> {
        match self {
            Self::Ok(v) => match v.try_into_array() {
                Ok(v) => {
                    let mut new_arr = vec![];
                    for x in v.into_iter() {
                        new_arr.push(Self::Ok(x));
                    }
                    Ok(new_arr)
                }
                Err(v) => Err(Self::Ok(v)),
            },
            Self::Array(v) => Ok(v),
            _ => Err(self),
        }
    }
}

impl PackerValue {
    pub fn is_object(&self) -> bool {
        match self {
            Self::Object(_) => true,
            Self::Ok(v) => v.is_object(),
            _ => false,
        }
    }

    pub fn is_array(&self) -> bool {
        match self {
            Self::Array(_) => true,
            Self::Ok(v) => v.is_array(),
            _ => false,
        }
    }

    /// Flatten the errors.
    ///
    /// If an array contains error, the entry is removed.
    /// If an object contains error, the key is removed.
    pub async fn flatten(self) -> Result<Value, Vec<PackerError>> {
        let mut errors = vec![];
        let flattened = self.flatten_internal(&mut errors).await;

        if errors.is_empty() {
            match flattened {
                Some(x) => Ok(x),
                _ => Err(errors),
            }
        } else {
            Err(errors)
        }
    }

    #[cfg_attr(not(feature = "wasm"), async_recursion::async_recursion)]
    #[cfg_attr(feature = "wasm", async_recursion::async_recursion(?Send))]
    async fn flatten_internal(self, output_errors: &mut Vec<PackerError>) -> Option<Value> {
        match self {
            Self::Ok(x) => Some(x),
            Self::Err(x) => {
                output_errors.push(x);
                None
            }
            Self::Array(v) => {
                let mut new_arr = vec![];
                for x in v.into_iter() {
                    if let Some(x) = x.flatten_internal(output_errors).await {
                        new_arr.push(x);
                    }
                }
                Some(Value::Array(new_arr))
            }
            Self::Object(o) => {
                let mut new_obj = Map::new();
                for (key, value) in o.into_iter() {
                    if let Some(x) = value.flatten_internal(output_errors).await {
                        new_obj.insert(key, x);
                    }
                }
                Some(Value::Object(new_obj))
            }
        }
    }
}
