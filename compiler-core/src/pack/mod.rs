//! Pack phase
//!
//! This phase ensures the `use`s in the route are resolved, and creates
//! the plugin runtimes and the compiler
//!
//! # Input
//! It takes [`PreparedContext`] as input, either cached or newly created.
//! If cached, the start_time of the compiler is set to the current time.
//!
//! It also takes options to modify the plugin list.
//!
//! # Work
//! 1. Resolve the `use`s in the route, if not already resolved
//! 2. Modify the plugin list according to the options
//! 3. Create the plugin runtimes, calling `onInit` for script plugins
//!
//! # Output
//! The output is a [`Compiler`]

use std::borrow::Cow;
use std::ops::{Deref, DerefMut};

use instant::Instant;

use crate::env::{join_futures, yield_budget};
use crate::json::RouteBlob;
use crate::plugin::{PluginError, PluginRuntime};
use crate::prep::{
    self, CompilerMetadata, GameCoord, PrepDoc, PreparedContext, RouteConfig, Setting,
};
use crate::res::Loader;

mod error;
pub use error::*;

/// Output of the pack phase.
///
/// The compiler keeps a reference to data in the prepared context to avoid copying.
/// Data that are allowed to be changed use copy-on-write.
///
/// The compiler is also stateful, as it tracks the current location and color of the route
pub struct Compiler<'p> {
    pub ctx: CompileContext<'p>,

    /// Reference to the built route
    pub route: Cow<'p, RouteBlob>,
    /// Current color of the map line
    pub color: String,
    /// Current position on the map
    pub coord: GameCoord,
    /// Runtime of the plugins
    pub plugin_runtimes: Vec<Box<dyn PluginRuntime>>,
}

impl<'p> Deref for Compiler<'p> {
    type Target = CompileContext<'p>;

    fn deref(&self) -> &Self::Target {
        &self.ctx
    }
}

impl<'p> DerefMut for Compiler<'p> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ctx
    }
}

impl<'p> AsRef<CompileContext<'p>> for Compiler<'p> {
    fn as_ref(&self) -> &CompileContext<'p> {
        &self.ctx
    }
}

impl<'p> AsMut<CompileContext<'p>> for Compiler<'p> {
    fn as_mut(&mut self) -> &mut CompileContext<'p> {
        &mut self.ctx
    }
}

/// Intermediate data when the compiler is being constructed
pub struct CompileContext<'p> {
    /// The start time of the compilation (not current phase)
    pub start_time: Instant,
    /// The config of the route
    pub config: Cow<'p, RouteConfig>,
    /// The metadata
    pub meta: Cow<'p, CompilerMetadata>,
    /// Compiler settings
    pub setting: &'p Setting,
}

impl<'p> CompileContext<'p> {
    /// Create the plugin runtimes from the plugin list
    pub async fn create_plugin_runtimes(&self) -> PackResult<Vec<Box<dyn PluginRuntime>>> {
        let mut output = Vec::with_capacity(self.meta.plugins.len());
        for plugin in &self.meta.plugins {
            yield_budget(4).await;
            let runtime = plugin
                .create_runtime(&self)
                .map_err(PackError::PluginInitError)?;
            output.push(runtime);
        }
        Ok(output)
    }
}

impl<L> PreparedContext<L>
where
    L: Loader,
{
    /// Entry point to the pack phase. Creates a [`Compiler`] that can be used to compile the route
    /// JSON to a document
    pub async fn create_compiler(&self, reset_start_time: bool) -> PackResult<Compiler<'_>> {
        let start_time = if reset_start_time {
            Instant::now()
        } else {
            self.start_time.clone()
        };

        let route_future = async {
            match &self.prep_doc {
                PrepDoc::Built(route) => Cow::Borrowed(route),
                PrepDoc::Raw(route) => {
                    let route =
                        prep::build_route(&self.project_res, route.clone(), &self.setting).await;
                    Cow::Owned(route)
                }
            }
        };

        let ctx = CompileContext {
            start_time,
            config: Cow::Borrowed(&self.config),
            meta: Cow::Borrowed(&self.meta),
            setting: &self.setting,
        };

        // TODO #24 plugin options

        let plugin_runtimes_future = ctx.create_plugin_runtimes();

        let (route, plugin_runtimes) = join_futures!(route_future, plugin_runtimes_future);
        let plugin_runtimes = plugin_runtimes?;

        let compiler = Compiler {
            ctx,
            route,
            plugin_runtimes,
            color: self
                .config
                .map
                .as_ref()
                .map(|m| m.initial_color.clone())
                .unwrap_or("#fff".to_string()),
            coord: self
                .config
                .map
                .as_ref()
                .map(|m| m.initial_coord.clone())
                .unwrap_or_default(),
        };

        Ok(compiler)
    }
}
