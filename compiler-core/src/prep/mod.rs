//! # Prepare (prep) phase
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

use std::collections::BTreeMap;
use std::ops::Deref;

use derivative::Derivative;
use instant::Instant;
use serde_json::{Map, Value};

use crate::env::{join_futures, RefCounted};
use crate::json::{Cast, Coerce, RouteBlob};
use crate::lang::Preset;
use crate::macros::derive_wasm;
use crate::plugin;
use crate::prop;
use crate::res::{Loader, ResPath, Resource, Use, ValidUse};
use crate::util::StringMap;

mod error;
pub use error::*;
mod entry_point;
pub use entry_point::*;
mod config;
pub use config::*;
mod route;
pub use route::*;

/// Output of the prep phase
#[derive(Debug, Clone)]
pub struct PrepCtx<L>
where
    L: Loader,
{
    pub project_res: Resource<'static, L>,
    data: RefCounted<PrepCtxData>,
}

impl<L> Deref for PrepCtx<L>
where
    L: Loader,
{
    type Target = PrepCtxData;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<L> PrepCtx<L>
where
    L: Loader,
{
    /// Hydrate a context from its data with a loader
    pub fn from_data(data: RefCounted<PrepCtxData>, loader: RefCounted<L>) -> Self {
        Self {
            project_res: Resource::new(data.res_path.clone(), loader),
            data,
        }
    }

    /// Get the ref-counted inner data
    #[inline]
    pub fn get_data(&self) -> &RefCounted<PrepCtxData> {
        &self.data
    }
}

/// The loader-independent data of prep context that
/// is safe to be cached
#[derive(Debug, Clone)]
pub struct PrepCtxData {
    pub res_path: ResPath<'static>,
    pub entry_path: Option<String>,
    pub config: RouteConfig,
    pub meta: CompilerMetadata,
    pub prep_doc: PrepDoc,
    pub start_time: Instant,
    pub setting: Setting,
    pub plugins: Vec<plugin::Instance>,
    // TODO #173: add a option to make ContextBuilder output a dependency list for PreparedContext
}

/// The route blob in the output of the prep phase, either built (`use`s resolved or raw (JSON).
#[derive(Debug, Clone)]
pub enum PrepDoc {
    Raw(Value),
    Built(RouteBlob),
}

/// Config of the route project
#[derive(PartialEq, Default, Debug, Clone)]
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

#[derive(PartialEq, Default, Debug, Clone)]
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
    /// Loaded presets
    pub presets: BTreeMap<String, Preset>,
    pub default_icon_priority: i64,
}

// prep phase entry points
#[derive(Debug)]
pub struct ContextBuilder<L>
where
    L: Loader,
{
    source: String,
    project_res: Resource<'static, L>,
    setting: Setting,
    entry_point: Option<String>,
    build_route: bool,
}

impl<L> ContextBuilder<L>
where
    L: Loader,
{
    /// Entry point for the prep phase.
    ///
    /// # Arguments
    /// `source` - A description for where the project is loaded from
    /// `project_res` - Points to the `project.yaml` file to load
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
    pub async fn build_context(mut self) -> PrepResult<PrepCtx<L>> {
        let start_time = Instant::now();
        let mut project = self.resolve_entry_point().await?;
        let metadata = self.load_metadata(&mut project)?;

        let config = match project.remove(prop::CONFIG) {
            Some(config) => config
                .try_into_array()
                .map_err(|_| PrepError::InvalidMetadataPropertyType(prop::CONFIG, "array"))?,
            None => vec![],
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
                default_icon_priority: prep_config.default_icon_priority,
            };

            PrepResult::Ok((config, meta, prep_config.plugins))
        };

        let (config_and_meta, prep_doc) = join_futures!(config_future, route_future);
        let (config, meta, plugins) = config_and_meta?;

        Ok(PrepCtx {
            data: RefCounted::new(PrepCtxData {
                res_path: self.project_res.path().clone(),
                entry_path: self.entry_point,
                config,
                meta,
                prep_doc,
                start_time,
                setting: self.setting,
                plugins,
            }),
            project_res: self.project_res,
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
    /// Also sets self.entry_point to the resolved entry path.
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
                return match Use::new(redirect_path.clone()) {
                    Use::Valid(valid) if matches!(valid, ValidUse::Absolute(_)) => {
                        // since the path is absolute, we can just use the project_res to resolve
                        // it
                        self.entry_point = Some(redirect_path.to_string());
                        self.project_res = self.project_res.resolve(&valid)?;
                        let mut project_obj = self.load_project().await?;
                        // remove and ignore the entry points in the redirected project
                        project_obj.remove(prop::ENTRY_POINTS);
                        Ok(project_obj)
                    }
                    _ => {
                        // this shouldn't happen
                        // since load_entry_points checks for if the path is valid
                        Err(PrepError::InvalidEntryPoint(
                            self.entry_point.as_ref().cloned().unwrap_or_default(),
                            "unreachable".to_string(),
                        ))
                    }
                };
            }
        }

        // no entry point redirection
        Ok(project_obj)
    }

    async fn load_project(&self) -> PrepResult<Map<String, Value>> {
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
            }
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
            }
            None => "(unspecified)".into(),
        };
        let source = match &self.entry_point {
            Some(entry_point) => format!("{} ({})", self.source, entry_point),
            None => self.source.clone(),
        };

        Ok(RouteMetadata {
            source,
            title,
            version,
        })
    }
}

/// Compilation settings
#[derive(Debug, Clone, Derivative)]
pub struct Setting {
    /// The maximum depth of `use` properties in route
    pub max_use_depth: usize,

    /// The maximum depth of `use` properties in config
    pub max_config_depth: usize,

    /// The maximum depth of object/array levels in the route
    pub max_ref_depth: usize,

    /// The maximum depth of preset namespaces in config
    pub max_preset_namespace_depth: usize,

    /// The maximum depth of preset references in route
    pub max_preset_ref_depth: usize,

    /// The maximum aliasing depth of entry points
    pub max_entry_point_depth: usize,
}

impl Setting {
    pub const fn const_default() -> Self {
        Self {
            max_use_depth: 8,
            max_config_depth: 16,
            max_ref_depth: 32,
            max_preset_namespace_depth: 16,
            max_preset_ref_depth: 8,
            max_entry_point_depth: 16,
        }
    }
}

impl Default for Setting {
    fn default() -> Self {
        Self::const_default()
    }
}
