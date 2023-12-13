use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::{CompilerContext, CompilerMetadata};
use crate::comp::{CompDoc, Compiler};
use crate::lang::parse_poor;
use crate::macros::async_trait;
use crate::pack::PackerResult;
use crate::types::{DocDiagnostic, ExecDoc};

mod builtin;

mod botw_unstable;
mod compat;
mod link;
mod metrics;
mod operation;
mod variables;

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum PlugError {
    #[error("{0}")]
    NotImpl(String),
}
impl PlugError {
    pub fn add_to_diagnostics(&self, output: &mut Vec<DocDiagnostic>) {
        output.push(DocDiagnostic {
            msg: parse_poor(&self.to_string()),
            msg_type: "error".to_string(),
            // TODO #24 get plugin name dynamically
            source: "celerc/plugin".to_string(),
        });
    }
}

pub type PlugResult<T> = Result<T, PlugError>;

/// The plugin runtime trait
///
/// A runtime of a plugin can store states that the plugin needs during the compilation.
/// Each compilation will spawn a new runtime with [`PluginInstance::create_runtime`]
#[async_trait(?Send)]
pub trait PluginRuntime {
    /// Called before route is compiled, to make changes to the compiler
    async fn on_before_compile<'a>(&mut self, _compiler: &mut Compiler<'a>) -> PackerResult<()> {
        Ok(())
    }
    /// Called after the route is compiled, to transform the route
    async fn on_after_compile(&mut self, _meta: &CompilerMetadata, _doc: &mut CompDoc) -> PlugResult<()> {
        Ok(())
    }
    /// Called after the route is turned into ExecDoc
    async fn on_after_execute<'a>(
        &mut self,
        _doc: &mut ExecDoc<'a>,
    ) -> PlugResult<()> {
        Ok(())
    }
}

/// An instance of a plugin read from the config file, with a source where the plugin can be loaded
/// from and properties to pass into the plugin
pub trait PluginInstance {

    /// Prepare the instance in the prep phase.
    ///
    /// Any preparation that can be used to speed up the `create_runtime` call should happen here.
    /// For example, scripts can be compiled here and props can be parsed.
    fn prepare<'a>(boxed: Box<Self>, compiler: &Compiler<'a>) -> PlugResult<Box<dyn PluginInstance>>;

    fn create_runtime<'a>(&self, compiler: &Compiler<'a>) -> Box<dyn PluginRuntime>;
}

pub struct LoadedPluginInstance<TRt> 
where TRt: PluginRuntime + Clone,
    {
        runtime: TRt,
}

impl PluginInstance for LoadedPluginInstance<TRt> 
where TRt: PluginRuntime + Clone,
    {
        fn prepare<'a>(boxed: Box<Self>, compiler: &Compiler<'a>) -> PlugResult<Box<dyn PluginInstance>> {
            Ok(boxed)
        }

        fn create_runtime<'a>(&self, compiler: &Compiler<'a>) -> Box<dyn PluginRuntime> {
            Box::new(self.runtime.clone())
        }
    }

#[derive(Debug, Clone)]
pub struct PluginInstance {
    pub plugin: Plugin,
    pub props: Value,
}

impl PluginInstance {
    pub fn create_runtime<'a>(&self, compiler: &Compiler<'a>) -> Box<dyn PluginRuntime> {
        match &self.plugin {
            Plugin::BuiltIn(built_in) => match built_in {
                BuiltInPlugin::Link => Box::new(link::LinkPlugin),
                BuiltInPlugin::Metrics => Box::new(metrics::MetricsPlugin::from_props(
                    &self.props,
                    &compiler.start_time,
                )),
                BuiltInPlugin::Variables => {
                    Box::new(variables::VariablesPlugin::from_props(&self.props))
                }
                // BuiltInPlugin::Compat => Box::new(compat::CompatPlugin),
                BuiltInPlugin::BotwAbilityUnstable => Box::new(
                    botw_unstable::BotwAbilityUnstablePlugin::from_props(&self.props),
                ),
            },
            // TODO #24 implement JS plugin engine
            Plugin::Script(_) => Box::new(ScriptPluginRuntime),
        }
    }
}

struct ScriptPluginRuntime;
#[async_trait(?Send)]
impl PluginRuntime for ScriptPluginRuntime {
    async fn on_before_compile(&mut self, _: &CompilerMetadata, _: &mut CompDoc) -> PlugResult<()> {
        // TODO #24 implement JS plugin engine
        Err(PlugError::NotImpl(
            "Script plugins are not implemented yet".to_string(),
        ))
    }
}

/// The source of a plugin, from which a [`PluginInstance`] can be created
#[derive(Debug, Clone)]
pub enum PluginSource {
    /// A built-in plugin
    BuiltIn(BuiltInPlugin),
    /// A script that is downloaded but not parsed.
    UncompiledScript(String),
    // TODO #24 implement JS plugin engine
    CompiledScript,
}

