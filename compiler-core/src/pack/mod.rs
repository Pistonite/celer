//! # Pack phase
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
use std::collections::BTreeSet;
use std::ops::{Deref, DerefMut};

use instant::Instant;

use crate::env::{join_futures, yield_budget};
use crate::json::RouteBlob;
use crate::plugin::{PluginRuntime, PluginOptions};
use crate::prep::{self, CompilerMetadata, PrepDoc, PreparedContext, RouteConfig, Setting};
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
    pub async fn create_plugin_runtimes(&self, options: Option<PluginOptions>) -> PackResult<Vec<Box<dyn PluginRuntime>>> {
        let add_size = options.as_ref().map(|x| x.add.len()).unwrap_or_default();
        let mut output = Vec::with_capacity(self.meta.plugins.len() + add_size);
        // we don't know how many plugins the users specify
        // so using a set to check for remove
        // even it's less efficient for small sizes
        let mut remove = BTreeSet::new();
        if let Some(options) = &options {
            remove.extend(options.remove.iter().map(|x| x.as_str()));
        }
        let mut seen = BTreeSet::new();
        for plugin in &self.meta.plugins {
            yield_budget(4).await;

            let id = plugin.get_id();

            let id_ref: &str = id.as_ref(); // need type annotation for contains below
            if remove.contains(id_ref) {
                continue;
            }

            if !plugin.allow_duplicate && !seen.insert(id) {
                return Err(PackError::DuplicatePlugin(plugin.get_display_name().into_owned()));
            }

            // TODO #175: plugin dependencies
            
            let runtime = plugin
                .create_runtime(self)
                .map_err(PackError::PluginInitError)?;
            output.push(runtime);
        }

        if let Some(options) = &options {
            for plugin in &options.add {
                yield_budget(4).await;

                let id = plugin.get_id();

                if !plugin.allow_duplicate && !seen.insert(id) {
                    return Err(PackError::DuplicatePlugin(plugin.get_display_name().into_owned()));
                }

                // TODO #175: plugin dependencies

                let runtime = plugin
                    .create_runtime(self)
                    .map_err(PackError::PluginInitError)?;
                output.push(runtime);
            }
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
    pub async fn create_compiler(
        &self,
        reset_start_time: Option<Instant>,
        plugin_options: Option<PluginOptions>,
    ) -> PackResult<Compiler<'_>> {
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

        let ctx = self.create_compile_context(reset_start_time);

        let plugin_runtimes_future = ctx.create_plugin_runtimes(plugin_options);

        let (route, plugin_runtimes) = join_futures!(route_future, plugin_runtimes_future);
        let plugin_runtimes = plugin_runtimes?;

        let compiler = Compiler {
            ctx,
            route,
            plugin_runtimes,
        };

        Ok(compiler)
    }

    pub fn create_compile_context(&self, reset_start_time: Option<Instant>) -> CompileContext<'_> {
        CompileContext {
            start_time: reset_start_time.unwrap_or(self.start_time),
            config: Cow::Borrowed(&self.config),
            meta: Cow::Borrowed(&self.meta),
            setting: &self.setting,
        }
    }
}
