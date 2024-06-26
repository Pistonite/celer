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
use std::ops::{Deref, DerefMut};

use instant::Instant;

use crate::env::yield_budget;
use crate::json::RouteBlob;
use crate::plugin;
use crate::prep::{self, CompilerMetadata, PrepCtx, PrepDoc, RouteConfig, Setting};
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
    pub plugin_runtimes: Vec<plugin::BoxedRuntime>,
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
    /// Plugin instances, does not include disabled plugins
    pub plugins: Vec<plugin::Instance>,
    /// Plugin metadata, including disabled plugins
    pub plugin_meta: Vec<plugin::Metadata>,
    /// Compiler settings
    pub setting: &'p Setting,
}

impl<'p> CompileContext<'p> {
    /// Configure the plugin list according to the options
    pub async fn configure_plugins(&mut self, options: Option<plugin::Options>) -> PackResult<()> {
        // apply options
        let plugin::OptionsApply {
            metadata,
            user_plugins,
        } = match options {
            None => plugin::Options::apply_none(&self.plugins),
            Some(options) => options.apply(&self.plugins),
        };

        // take the plugins out for processing
        // new plugins will be put into self.plugins
        let old_instances = std::mem::take(&mut self.plugins);

        // temporary storage of plugins
        // plugins will load actual instances into this list
        let mut list = plugin::LoadList::default();

        // load plugins from route into the list
        // disabled plugins are removed
        for (meta, plugin) in metadata.iter().zip(old_instances.into_iter()) {
            yield_budget(4).await;
            if meta.is_enabled {
                let early_rt = plugin.create_early_runtime()?;
                early_rt.on_load_plugin(plugin, &mut list).await?;
            }
        }
        // load user plugins
        // disabled plugins are already removed
        for plugin in user_plugins {
            yield_budget(4).await;
            let early_rt = plugin.create_early_runtime()?;
            early_rt.on_load_plugin(plugin, &mut list).await?;
        }

        // transfer new plugin list and meta
        self.plugins.extend(list);
        self.plugin_meta = metadata;

        Ok(())
    }

    pub async fn create_plugin_runtimes(&self) -> PackResult<Vec<plugin::BoxedRuntime>> {
        let mut output = Vec::with_capacity(self.plugins.len());
        for plugin in &self.plugins {
            yield_budget(4).await;

            let runtime = plugin
                .create_runtime(self)
                .map_err(PackError::PluginInitError)?;
            output.push(runtime);
        }
        Ok(output)
    }
}

impl<L> PrepCtx<L>
where
    L: Loader,
{
    /// Entry point to the pack phase.
    ///
    /// The returned compile context should be configured, then [`create_compiler`] is called to
    /// create the compiler to proceed to the next phase.
    pub async fn new_compilation(&self, reset_start_time: Option<Instant>) -> CompileContext<'_> {
        let ctx = CompileContext {
            start_time: reset_start_time.unwrap_or(self.start_time),
            config: Cow::Borrowed(&self.config),
            meta: Cow::Borrowed(&self.meta),
            plugins: self.plugins.clone(),
            plugin_meta: vec![],
            setting: &self.setting,
        };
        ctx
    }

    /// Create the compiler to continue to the next phase
    ///
    /// If this fails, the CompileContext is returned to the caller along with the error
    pub async fn create_compiler<'p>(
        &'p self,
        context: CompileContext<'p>,
    ) -> Result<Compiler<'p>, (PackError, CompileContext<'p>)> {
        let route = match &self.prep_doc {
            PrepDoc::Built(route) => Cow::Borrowed(route),
            PrepDoc::Raw(route) => {
                let route =
                    prep::build_route(&self.project_res, route.clone(), &self.setting).await;
                Cow::Owned(route)
            }
        };
        let plugin_runtimes = match context.create_plugin_runtimes().await {
            Ok(x) => x,
            Err(e) => return Err((e, context)),
        };
        let compiler = Compiler {
            ctx: context,
            route,
            plugin_runtimes,
        };
        Ok(compiler)
    }
}
