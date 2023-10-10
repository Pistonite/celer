use celerctypes::{DocDiagnostic, RouteMetadata};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::CompilerContext;
use crate::comp::CompDoc;
use crate::json::Coerce;
use crate::lang::parse_poor;
use crate::macros::{maybe_send, async_trait};
use crate::pack::PackerResult;
use crate::prop;

mod link;
mod metrics;
mod operation;

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
#[maybe_send(async_trait)]
pub trait PluginRuntime {
    async fn on_pre_compile(&mut self, _: &mut RouteMetadata) -> PackerResult<()> {
        Ok(())
    }
    async fn on_compile(&mut self, _: &mut CompDoc) -> PlugResult<()> {
        Ok(())
    }
    async fn on_post_compile(&mut self, _: &mut RouteMetadata) -> PlugResult<()> {
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct PluginInstance {
    pub plugin: Plugin,
    pub props: Value,
}

impl PluginInstance {
    pub fn create_runtime(&self, context: &CompilerContext) -> Box<dyn PluginRuntime> {
        match &self.plugin {
            Plugin::BuiltIn(built_in) => match built_in {
                BuiltInPlugin::Link => Box::new(link::LinkPlugin),
                BuiltInPlugin::Metrics => 
                    Box::new(metrics::MetricsPlugin::from_props(&self.props, context.get_start_time())),
            },
            // TODO #24 implement JS plugin engine
            Plugin::Script(_) => Box::new(ScriptPluginRuntime),
        }
    }
}

struct ScriptPluginRuntime;
#[maybe_send(async_trait)]
impl PluginRuntime for ScriptPluginRuntime {
    async fn on_compile(&mut self, _: &mut CompDoc) -> PlugResult<()> {
        // TODO #24 implement JS plugin engine
        Err(PlugError::NotImpl(
            "Script plugins are not implemented yet".to_string(),
        ))
    }
}

#[derive(Debug, Clone)]
pub enum Plugin {
    BuiltIn(BuiltInPlugin),
    Script(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BuiltInPlugin {
    /// Collect compiler metrics and report them through the stats API
    Metrics,
    /// Transform link tags to clickable links. See [`link`]
    Link,
}

// pub async fn run_plugins(mut comp_doc: CompDoc, plugins: &[PluginRuntime]) -> CompDoc {
//     let mut errors = Vec::new();
//     for plugin in plugins {
//         if let Err(e) = plugin.run(&mut comp_doc).await {
//             errors.push(e);
//         }
//     }
//     if !errors.is_empty() {
//         for error in errors {
//             error.add_to_diagnostics(&mut comp_doc.diagnostics);
//         }
//     }
//     comp_doc
// }
