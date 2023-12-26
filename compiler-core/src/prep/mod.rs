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
use instant::Instant;
use serde::{Serialize, Deserialize};
use serde_json::{Value, Map};

use crate::json::{RouteBlob, Cast, Coerce};
use crate::lang::Preset;
use crate::plugin::PluginInstance;
use crate::res::{ResError, Loader, Resource, Use, ValidUse};
use crate::prop;
use crate::env::join_futures;
use crate::macros::derive_wasm;
use crate::util::StringMap;

mod entry_point;
pub use entry_point::*;
mod config;
pub use config::*;
mod route;
pub use route::*;

/// Error during the prep phase
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

/// Output of the prep phase
#[derive(Debug, Clone)]
pub struct PreparedContext<L> where L: Loader {
    pub project_res: Resource<'static, L>,
    pub config: RouteConfig,
    pub meta: CompilerMetadata,
    pub prep_doc: PrepDoc,
    pub start_time: Instant,
    pub setting: Setting,
}

/// The route blob in the output of the prep phase, either built (`use`s resolved or raw (JSON).
#[derive(Debug, Clone)]
pub enum PrepDoc {
    Raw(Value),
    Built(RouteBlob),
}

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

// prep phase entry points
#[derive(Debug)]
pub struct ContextBuilder<L> where L: Loader {
    source: String,
    project_res: Resource<'static, L>,
    setting: Setting,
    entry_point: Option<String>,
    build_route: bool,
}

impl<L> ContextBuilder<L> where L: Loader {
    pub fn new(source: String, project_res: Resource<'static, L>) -> Self {
        Self {
            source,
            project_res,
            setting: Setting::default(),
            entry_point: None,
            build_route: false,
        }
    }

    /// Set setting to something different from default
    pub fn setting(mut self, setting: Setting) -> Self {
        self.setting = setting;
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
    pub async fn build_context(mut self) -> PrepResult<PreparedContext<L>> {
        let start_time = Instant::now();
        let mut project = self.resolve_entry_point().await?;
        let metadata = self.load_metadata(&mut project)?;

        let config = match project.remove(prop::CONFIG) {
            Some(config) => {
                config.try_into_array()
                    .map_err(|_| PrepError::InvalidMetadataPropertyType(prop::CONFIG, "array"))?
            },
            None => vec![]
        };

        let route = project.remove(prop::ROUTE).unwrap_or_default();

        if let Some(k) = project.keys().next() {
            return Err(PrepError::UnusedMetadataProperty(k.clone()));
        }

        let route_future = async {
            if self.build_route {
                let route = route::build_route(&self.project_res, route, &self.setting).await;
                PrepDoc::Built(route)
            } else {
                PrepDoc::Raw(route)
            }
        };

        let config_future = async {
            let mut prep_config = PreparedConfig::new(&self.setting);
            prep_config.load_configs(&self.project_res, config).await?;
            let config = RouteConfig {
                meta: metadata,
                map: prep_config.map,
                icons: prep_config.icons.into(),
                tags: prep_config.tags.into(),
                splits: prep_config.splits,
                stats: Default::default(),
            };
            // optimize presets
            let mut unoptimized_presets = prep_config.presets;
            let mut optimized_presets = BTreeMap::new();
            while let Some((name, mut preset)) = unoptimized_presets.pop_first() {
                preset
                    .optimize(&mut unoptimized_presets, &mut optimized_presets)
                .await;
                optimized_presets.insert(name, preset);
            }

            let meta = CompilerMetadata {
                presets: optimized_presets,
                plugins: prep_config.plugins,
                default_icon_priority: prep_config.default_icon_priority,
            };

            PrepResult::Ok((config, meta))
        };

        let (config_and_meta, prep_doc) = join_futures!(config_future, route_future);
        let (config, meta) = config_and_meta?;

        Ok(PreparedContext {
            project_res: self.project_res,
            config,
            meta,
            prep_doc,
            start_time,
            setting: self.setting,
        })
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

        entry_point::load_entry_points(entry_points_value, &self.setting).await
    }

    /// Load the project and switch the project resource to the entry point resource.
    /// Returns the loaded project object with the `entry-points` property removed
    ///
    /// If the entry point is None, it will attempt to redirect to the "default" entry point
    async fn resolve_entry_point(&mut self) -> PrepResult<Map<String, Value>> {
        let mut project_obj = self.load_project().await?;

        if let Some(entry_points) = project_obj.remove(prop::ENTRY_POINTS) {
            let setting = &self.setting;
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
}

/// Compilation settings
#[derive(Debug, Clone, Derivative)]
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
