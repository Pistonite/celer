use celerctypes::{DocDiagnostic, RouteMetadata};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::comp::CompDoc;
use crate::lang::parse_poor;
use crate::pack::PackerResult;

mod link;
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
#[cfg_attr(not(feature = "wasm"), async_trait::async_trait)]
#[cfg_attr(feature = "wasm", async_trait::async_trait(?Send))]
pub trait PluginRuntime {
    async fn on_pre_compile(&mut self, project: &mut RouteMetadata) -> PackerResult<()> {
        Ok(())
    }
    async fn on_compile(&mut self, comp_doc: &mut CompDoc) -> PlugResult<()> {
        Ok(())
    }
    async fn on_post_compile(&mut self, project: &mut RouteMetadata) -> PlugResult<()> {
        Ok(())
    }
}

// macro_rules! noop_plugin_phase_impl {
//     (on_pre_compile) => {
//         async fn on_pre_compile(&mut self, _: &mut celerctypes::RouteMetadata) -> $crate::pack::PackerResult<()> {
//             Ok(())
//         }
//     };
//     (on_compile) => {
//         async fn on_compile(&mut self, _: &mut CompDoc) -> $crate::plug::PlugResult<()> {
//             Ok(())
//         }
//     };
//     (on_post_compile) => {
//         async fn on_post_compile(&mut self, _: &mut celerctypes::RouteMetadata) -> $crate::plug::PlugResult<()> {
//             Ok(())
//         }
//     }
// }
// pub (crate) use noop_plugin_phase_impl;

pub struct PluginInstance {
    pub plugin: Plugin,
    pub props: Value,
}

impl PluginInstance {
    pub fn create_runtime(&self) -> Box<dyn PluginRuntime> {
        match &self.plugin {
            Plugin::BuiltIn(built_in) => match built_in {
                BuiltInPlugin::Metrics => todo!(),
                BuiltInPlugin::Link => Box::new(link::LinkPlugin),
            },
            Plugin::Script(_) => unimplemented!(),
        }
    }
}

struct ScriptPluginRuntime;
#[cfg_attr(not(feature = "wasm"), async_trait::async_trait)]
#[cfg_attr(feature = "wasm", async_trait::async_trait(?Send))]
impl PluginRuntime for ScriptPluginRuntime {
    async fn on_compile(&mut self, _: &mut CompDoc) -> PlugResult<()> {
        // TODO #24 implement JS plugin engine
        Err(PlugError::NotImpl(
            "Script plugins are not implemented yet".to_string(),
        ))
    }
}

// impl PluginRuntime {
//     pub async fn on_pre_compile(&self, project: &mut RouteMetadata) -> PackerResult<()> {
//         match &self.plugin {
//             Plugin::BuiltIn(built_in) => built_in.on_pre_compile(project, &self.value).await,
//             // TODO #24 implement this
//             Plugin::Script(_) => Err(PlugError::NotImpl(
//                 "Script plugins are not implemented yet".to_string(),
//             )),
//         }
//     }
//     pub async fn on_compile(&self, comp_doc: &mut CompDoc) -> PlugResult<()> {
//         match &self.plugin {
//             Plugin::BuiltIn(built_in) => built_in.run(comp_doc).await,
//             // TODO #24 implement this
//             Plugin::Script(_) => Err(PlugError::NotImpl(
//                 "Script plugins are not implemented yet".to_string(),
//             )),
//         }
//     }
//
//     pub async fn on_post_compile(&self, project: &mut RouteMetadata) -> PlugResult<()> {
//         match &self.plugin {
//             Plugin::BuiltIn(built_in) => built_in.run(comp_doc).await,
//             // TODO #24 implement this
//             Plugin::Script(_) => Err(PlugError::NotImpl(
//                 "Script plugins are not implemented yet".to_string(),
//             )),
//         }
//     }
// }

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
