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

use crate::env::yield_budget;
use crate::json::RouteBlob;
use crate::plugin::{PluginRuntime, PluginOptions, PluginMetadata, PluginInstance};
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
    /// Plugin instances, does not include disabled plugins
    pub plugins: Vec<PluginInstance>,
    /// Plugin metadata, including disabled plugins
    pub plugin_meta: Vec<PluginMetadata>,
    /// Compiler settings
    pub setting: &'p Setting,
}

impl<'p> CompileContext<'p> {
    /// Configure the plugin list according to the options
    pub async fn configure_plugins(&mut self, options: Option<PluginOptions>) -> PackResult<()> {
        let add_size = options.as_ref().map(|x| x.add.len()).unwrap_or_default();
        let old_instances = {
            let mut new_instances = Vec::with_capacity(self.plugins.len() + add_size);
            std::mem::swap(&mut self.plugins, &mut new_instances);
            new_instances
        };

        let mut seen = BTreeSet::new();
        let mut duplicates = Vec::new();

        self.plugin_meta.clear();

        // we don't know how many plugins the users specify
        // so using a set to check for remove
        // even it's less efficient for small sizes
        let mut remove = BTreeSet::new();
        let mut add = None;
        if let Some(options) = options {
            remove.extend(options.remove);
            add = Some(options.add);
        }
        for plugin in old_instances {
            yield_budget(4).await;

            let id = plugin.get_id().to_string();
            self.plugin_meta.push(PluginMetadata {
                id: id.clone(),
                name: plugin.get_display_name().to_string(),
                is_from_user: false,
            });

            if remove.contains(&id) {
                continue;
            }

            if !plugin.allow_duplicate && seen.contains(&id) {
                duplicates.push(id);
                continue;
            }
            seen.insert(id);

            // TODO #175: plugin dependencies
            self.plugins.push(plugin);
        }

        if let Some(add) = add {
            for plugin in add {
                yield_budget(4).await;

                let id = plugin.get_id().to_string();
                self.plugin_meta.push(PluginMetadata {
                    id: id.clone(),
                    name: plugin.get_display_name().to_string(),
                    is_from_user: true,
                });

                if !plugin.allow_duplicate && seen.contains(&id) {
                    duplicates.push(id);
                    continue;
                }
                seen.insert(id);

                // TODO #175: plugin dependencies
                self.plugins.push(plugin);
            }
        }

        if !duplicates.is_empty() {
            return Err(PackError::DuplicatePlugins(duplicates.join(", ")));
        }

        Ok(())
    }

    pub async fn create_plugin_runtimes(&self) -> PackResult<Vec<Box<dyn PluginRuntime>>> {
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

impl<L> PreparedContext<L>
where
    L: Loader,
{
    /// Entry point to the pack phase.
    ///
    /// The returned compile context should be configured, then [`create_compiler`] is called to
    /// create the compiler to proceed to the next phase.
    pub async fn new_compilation(&self, reset_start_time: Option<Instant>) -> PackResult<CompileContext<'_>> {
        let ctx = CompileContext {
            start_time: reset_start_time.unwrap_or(self.start_time),
            config: Cow::Borrowed(&self.config),
            meta: Cow::Borrowed(&self.meta),
            plugins: self.plugins.clone(),
            plugin_meta: vec![],
            setting: &self.setting,
        };
        Ok(ctx)
    }

    pub async fn create_compiler<'p>(&'p self, context: CompileContext<'p>) -> PackResult<Compiler<'p>> {
        let route= match &self.prep_doc {
            PrepDoc::Built(route) => Cow::Borrowed(route),
            PrepDoc::Raw(route) => {
                let route = prep::build_route(&self.project_res, route.clone(), &self.setting).await;
                Cow::Owned(route)
            }
        };
        let plugin_runtimes = context.create_plugin_runtimes().await?;
        let compiler = Compiler {
            ctx: context,
            route,
            plugin_runtimes,
        };
        Ok(compiler)
    }
}
