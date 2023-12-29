use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::comp::CompDoc;
use crate::lang::{parse_poor, DocDiagnostic, DocPoorText};
use crate::pack::CompileContext;
use crate::prep::CompilerMetadata;
use crate::types::{ExecDoc};

mod builtin;
mod js;
mod operation;

pub use builtin::BuiltInPlugin;
pub use js::ScriptPlugin;

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum PluginError {
    #[error("An exception occured while executing script: {0}")]
    ScriptException(String),
}

// impl PluginError {
//     pub fn add_to_diagnostics(&self, output: &mut Vec<DocDiagnostic>) {
//         output.push(DocDiagnostic {
//             msg: parse_poor(&self.to_string()),
//             msg_type: "error".to_string(),
//             // TODO #24 get plugin name dynamically
//             source: "celerc/plugin".to_string(),
//         });
//     }
// }

pub type PluginResult<T> = Result<T, PluginError>;

/// The plugin runtime trait
///
/// A runtime of a plugin can store states that the plugin needs during the compilation.
/// Each compilation will spawn a new runtime with [`PluginInstance::create_runtime`]
pub trait PluginRuntime {
    /// Get a string representing the source of the plugin.
    fn get_source(&self) -> &str;
    fn add_diagnostics(&self, msg: DocPoorText, msg_type: String, output: &mut Vec<DocDiagnostic>) {
        todo!()
        // output.push(DocDiagnostic {
        //     msg,
        //     msg_type,
        //     source: self.get_source().to_string(),
        // });
    }

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
