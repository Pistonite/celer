//! Prepare (prep) phase
//!
//! # Input
//! This is the 0-th phase of the compiler that prepares the metadata for the compilation.
//! It is also responsible for inspecting the project properties such as entry points and
//! title/version.
//!
//! It takes (from the outside) a [`Resource`](crate::res::Resource) that is the root project,
//! as well as the entry point
//!
//! # Work
//! 1. Loading the entry point config (project.yaml). If the entry point contains redirection
//!    through `entrypoints` property, it finds the correct entry point config to load.
//! 2. Build the configuration object
//! 3. Optimize configuration and compile plugins to be cached
//!
//! # Output
//! The output of this phase is a [`PreparedContext`] object that can be used to create
//! the compiler with additional (and optional) plugins.

use std::borrow::Cow;
use std::collections::BTreeMap;

use derivative::Derivative;
use serde_json::{Value, Map};

use crate::json::Coerce;
use crate::lang::Preset;
use crate::plugin::PluginInstance;
use crate::res::{ResError, Loader, Resource, Use, ValidUse};
use crate::prop;

mod entry_point;
pub use entry_point::*;
mod config;
pub use config::*;
mod metadata;
pub use metadata::*;
mod route;
pub use route::*;

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum PrepError {
    #[error("Failed to load resource: {0}")]
    Res(#[from] ResError),

    #[error("The project file `{0}` should be a mapping object")]
    InvalidProjectResourceType(String),

    #[error("Project config ({0}) should be a mapping object")]
    InvalidConfigType(ConfigTrace),

    #[error("Project config ({0}): property `{1}` has an invalid type (expected {2})")]
    InvalidConfigPropertyType(ConfigTrace, Cow<'static, str>, Cow<'static,str>),

    #[error("Project config ({0}): property `{1}` is missing")]
    MissingConfigProperty(ConfigTrace, Cow<'static, str>),
    
    #[error("Project config ({0}): the `{1}` property is unused")]
    UnusedConfigProperty(ConfigTrace, Cow<'static, str>),

    #[error("Project config ({0}): cannot find tag `{1}`")]
    TagNotFound(ConfigTrace, Cow<'static, str>),

    #[error("Project config ({0}): `{1}` is not a valid built-in plugin or reference to a plugin script")]
    InvalidPlugin(ConfigTrace, String),

    #[error("Project config ({0}): config is nesting too deep! Check that you don't have circular dependency, or simplify the config structure")]
    MaxConfigDepthExceeded(ConfigTrace),

    #[error("Project config ({0}): defining map when a previous config already defines one")]
    DuplicateMap(ConfigTrace),

    #[error("Max preset namespace depth of {0} levels is reached. There might be a formatting error in your project files. If this is intentional, consider making the namespaces less complex.")]
    MaxPresetNamespaceDepthExceeded(usize),

    #[error("Project config ({0}): preset {1} is invalid")]
    InvalidPreset(ConfigTrace, String),

    #[error("Project metadata property `{0}` has invalid type (expecting {1})")]
    InvalidMetadataPropertyType(&'static str, &'static str),

    #[error("Project metadata has extra unused property: {0}")]
    UnusedMetadataProperty(String),

    #[error("Entry point `{0}` is invalid: `{1}` is neither an absolute path, nor a name of another entry point.")]
    InvalidEntryPoint(String, String),

    #[error("Entry point `{0}` is nesting too deep! Do you have a recursive loop?")]
    MaxEntryPointDepthExceeded(String),
}

pub type PrepResult<T> = Result<T, PrepError>;



pub struct PreparedContext {
    pub config: RouteConfig,
    pub meta: CompilerMetadata,
    prep_doc: PrepDoc,
}

pub enum PrepDoc {
    Raw(Value),
    Built(RouteBlob),
}


/// Metadata of the compiler
///
/// This is information needed during compilation,
/// but not needed to render the route.
/// IDEs may also find this useful to provide auto-complete, etc.
#[derive(Default, Debug, Clone)]
pub struct CompilerMetadata {
    pub presets: BTreeMap<String, Preset>,
    pub plugins: Vec<PluginInstance>,
    pub default_icon_priority: i64,
}

static DEFAULT_SETTING: Setting = Setting::default();

// prep phase entry points
#[derive(Debug)]
pub struct ContextBuilder<L> where L: Loader {
    source: String,
    project_res: Resource<'static, L>,
    setting: Option<Setting>,
    entry_point: Option<String>,
    build_route: bool,
}

impl<L> ContextBuilder<L> where L: Loader {
    pub fn new(source: String, project_res: Resource<'static, L>) -> Self {
        Self {
            source,
            project_res,
            setting: None,
            entry_point: None,
            build_route: false,
        }
    }

    /// Set setting to something different from default
    pub fn setting(mut self, setting: Setting) -> Self {
        self.setting = Some(setting);
        self
    }

    /// Set the entry point. An entry point starting with `/`
    /// is treated as a path, and otherwise it is treated as an alias
    pub fn entry_point(mut self, entry_point: Option<String>) -> Self {
        self.entry_point = entry_point;
        self
    }

    /// Build the route when creating the context. This allows the built
    /// route to be cached along with the context
    pub fn with_route_built(mut self) -> Self {
        self.build_route = true;
        self
    }

    /// Load the project and parse config and (optionally) route
    pub async fn build_context(self) -> PrepResult<PreparedContext> {
        todo!()
    }

    /// Load the project, but only parse the metadata, not the entire config or the route
    pub async fn get_metadata(mut self) -> PrepResult<RouteMetadata> {
        let mut project_obj = self.resolve_entry_point().await?;
        self.load_metadata(&mut project_obj)
    }

    /// Load the entry points from the `entry-points` property of the project
    pub async fn get_entry_points(&self) -> PrepResult<EntryPoints> {
        let mut project_obj = self.load_project().await?;

        let entry_points_value = match project_obj.remove(prop::ENTRY_POINTS) {
            Some(v) => v,
            None => return Ok(Default::default()),
        };

        entry_point::load_entry_points(entry_points_value, self.get_setting()).await
    }

    /// Load the project and switch the project resource to the entry point resource.
    /// Returns the loaded project object with the `entry-points` property removed
    ///
    /// If the entry point is None, it will attempt to redirect to the "default" entry point
    async fn resolve_entry_point(&mut self) -> PrepResult<Map<String, Value>> {
        let mut project_obj = self.load_project().await?;

        if let Some(entry_points) = project_obj.remove(prop::ENTRY_POINTS) {
            let setting = self.get_setting();
            let entry_points = entry_point::load_entry_points(entry_points, setting).await?;

            let path = match &self.entry_point {
                None => {
                    // try redirecting to default
                    entry_points.resolve_alias(prop::DEFAULT, setting).ok()
                }
                Some(entry_point) => {
                    // try resolve it
                    Some(entry_points.resolve_alias(entry_point, setting)?)
                }
            };

            if let Some(redirect_path) = path {
                match Use::new(redirect_path) {
                    Use::Valid(valid) if matches!(valid, ValidUse::Absolute(_)) => {
                        self.project_res = self.project_res.resolve(&valid)?;
                        let mut project_obj = self.load_project().await?;
                        // remove and ignore the entry points in the redirected project
                        project_obj.remove(prop::ENTRY_POINTS);
                        return Ok(project_obj);
                    }
                    _ => {
                        // this shouldn't happen
                        // since load_entry_points checks for if the path is valid
                        return Err(PrepError::InvalidEntryPoint(self.entry_point.as_ref().cloned().unwrap_or_default(), "unreachable".to_string()));
                    }
                }
            }
        }

        // no entry point redirection
        Ok(project_obj)
    }

    async fn load_project(&self) -> PrepResult<Map<String, Value>> 
    {
        match self.project_res.load_structured().await? {
            Value::Object(o) => Ok(o),
            _ => Err(PrepError::InvalidProjectResourceType(
                self.project_res.path().to_string(),
            )),
        }
    }


    /// Load the metadata from the project value. The metadata properties are removed
    fn load_metadata(&self, project: &mut Map<String, Value>) -> PrepResult<RouteMetadata> {
        let title = match project.remove(prop::TITLE) {
            Some(title) => {
                if title.is_array() || title.is_object() {
                    return Err(PrepError::InvalidMetadataPropertyType(
                        prop::TITLE,
                        "string",
                    ));
                }
                title.coerce_to_string()
            },
            None => "Untitled Project".into(),
        };
        let version = match project.remove(prop::VERSION) {
            Some(version) => {
                if version.is_array() || version.is_object() {
                    return Err(PrepError::InvalidMetadataPropertyType(
                        prop::VERSION,
                        "string",
                    ));
                }
                version.coerce_to_string()
            },
            None => "(unspecified)".into(),
        };

        Ok(RouteMetadata {
            source: self.source.clone(),
            title,
            version,
        })

    }

    fn get_setting(&self) -> &Setting {
        self.setting.as_ref().unwrap_or(&DEFAULT_SETTING)
    }
}

/// Compilation settings
#[derive(Debug, Derivative)]
#[derivative(Default)]
pub struct Setting {
    /// The maximum depth of `use` properties in route
    #[derivative(Default(value = "8"))]
    pub max_use_depth: usize,

    /// The maximum depth of `use` properties in config
    #[derivative(Default(value = "16"))]
    pub max_config_depth: usize,

    /// The maximum depth of object/array levels in the route
    #[derivative(Default(value = "32"))]
    pub max_ref_depth: usize,

    /// The maximum depth of preset namespaces in config
    #[derivative(Default(value = "16"))]
    pub max_preset_namespace_depth: usize,

    /// The maximum depth of preset references in route
    #[derivative(Default(value = "8"))]
    pub max_preset_ref_depth: usize,

    /// The maximum aliasing depth of entry points
    #[derivative(Default(value = "16"))]
    pub max_entry_point_depth: usize,

}
