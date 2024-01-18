use std::borrow::Cow;

use serde_json::Value;

use crate::comp::CompDoc;
use crate::pack::CompileContext;
use crate::exec::ExecDoc;

mod error;
pub use error::*;
mod builtin;
mod js;
mod operation;

pub use builtin::BuiltInPlugin;
pub use js::ScriptPlugin;


/// The plugin runtime trait
///
/// A runtime of a plugin can store states that the plugin needs during the compilation.
/// Each compilation will spawn a new runtime with [`PluginInstance::create_runtime`]
pub trait PluginRuntime {
    /// Get a string representing the source of the plugin.
    fn get_source(&self) -> Cow<'static, str>;

    /// Called before route is compiled, to make changes to the compiler
    fn on_before_compile<'a>(&mut self, _ctx: &mut CompileContext<'a>) -> PluginResult<()> {
        Ok(())
    }
    /// Called after the route is compiled, to transform the route
    fn on_after_compile(&mut self, _doc: &mut CompDoc) -> PluginResult<()> {
        Ok(())
    }
    /// Called after the route is turned into ExecDoc
    fn on_after_execute<'a>(&mut self, _doc: &mut ExecDoc<'a>) -> PluginResult<()> {
        Ok(())
    }
}

/// An instance of a plugin read from the config file, with a source where the plugin can be loaded
/// from and properties to pass into the plugin
#[derive(Debug, Clone)]
pub struct PluginInstance {
    pub plugin: Plugin,
    pub props: Value,
}

impl PluginInstance {
    pub fn create_runtime<'a>(
        &self,
        ctx: &CompileContext<'a>,
    ) -> PluginResult<Box<dyn PluginRuntime>> {
        match &self.plugin {
            Plugin::BuiltIn(p) => p.create_runtime(ctx, &self.props),
            Plugin::Script(p) => p.create_runtime(ctx, &self.props),
        }
    }
}

/// The source of a plugin, from which a [`PluginInstance`] can be created
#[derive(Debug, Clone)]
pub enum Plugin {
    /// A built-in plugin
    BuiltIn(BuiltInPlugin),
    /// A script that is downloaded but not parsed.
    Script(ScriptPlugin),
}
