use celerctypes::DocDiagnostic;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::comp::{CompDoc, CompSection};
use crate::lang::parse_poor;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginRuntime {
    pub plugin: Plugin,
    pub props: Value,
}

impl PluginRuntime {
    pub async fn run(&self, comp_doc: &mut CompDoc) -> PlugResult<()> {
        match &self.plugin {
            Plugin::BuiltIn(built_in) => built_in.run(comp_doc).await,
            Plugin::Script(_) => Err(PlugError::NotImpl("Script plugins are not implemented yet".to_string())),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Plugin {
    BuiltIn(BuiltInPlugin),
    Script(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged, rename_all = "kebab-case")]
pub enum BuiltInPlugin {
    /// Transform link tags to clickable links. See [`link`]
    Link
}

impl BuiltInPlugin {
    pub async fn run(&self, comp_doc: &mut CompDoc) -> PlugResult<()> {
        match self {
            BuiltInPlugin::Link => {
                link::run_link_plugin(comp_doc).await;
                Ok(())
            }
        }
    }
}

pub async fn run_plugins(mut comp_doc: CompDoc, plugins: &[PluginRuntime]) -> CompDoc {
    let mut errors = Vec::new();
    for plugin in plugins {
        if let Err(e) = plugin.run(&mut comp_doc).await {
            errors.push(e);
        }
    }
    if !errors.is_empty() {
        for error in errors {
            error.add_to_diagnostics(&mut comp_doc.diagnostics);
        }
    }
    comp_doc

}

