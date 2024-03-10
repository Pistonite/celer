//! # Export (expo) phase
//!
//! This phase collects export metadata from the plugins. The export metadata
//! is used to display export options to the user.
//!
//! # Input
//! The input is a [`ExecContext`].
//!
//! # Output
//! The output is a [`ExpoContext`].
use serde_json::Value;

use crate::comp::CompDoc;
use crate::exec::ExecContext;
use crate::macros::derive_wasm;

mod blob;
pub use blob::*;

/// Output of the export phase
#[derive_wasm]
pub struct ExpoContext<'p> {
    #[serde(flatten)]
    pub exec_ctx: ExecContext<'p>,
    /// The export metadata
    pub export_metadata: Vec<ExportMetadata>,
}

/// Data to define a plugin's export capability
#[derive(Debug, Clone)]
#[derive_wasm]
pub struct ExportMetadata {
    /// The id of the export plugin to run. This is the same string in the `use` of the plugin
    pub plugin_id: String,
    /// Name of the export. For example "LiveSplit". This is shown in the menu
    pub name: String,
    /// Long description. This shows as a tooltip
    pub description: String,

    /// (Optional) Icon to show next to the name
    #[serde(default)]
    pub icon: ExportIcon,

    /// (Optional) File extension of the export. For example "lss"
    pub extension: Option<String>,

    /// (Optional) Extra id to distinguish multiple exports from the same exporter
    pub export_id: Option<String>,

    /// (Optional) Example YAML configuration for the exporter to show to the user
    pub example_config: Option<String>,

    /// (Optional) Learn more link to guide the user on how to configure the export.
    ///
    /// ONLY VISIBLE if you also provide an example config!
    pub learn_more: Option<String>,
}

/// Icon for the export.
///
/// This is only for visual. It does not restrict what the export can/cannot contain.
#[derive(Debug, Clone, Default)]
#[derive_wasm]
pub enum ExportIcon {
    Archive,
    Binary,
    Cat,
    Code,
    Data,
    #[default]
    File,
    Image,
    Text,
    Video,
}

/// Request to export a document sent from the client
#[derive(Debug, Clone)]
#[derive_wasm]
pub struct ExportRequest {
    /// Id of the exporter plugin to run
    #[serde(rename = "pluginId")]
    pub plugin_id: String,
    /// Extra id to distinguish multiple exports from the same exporter
    #[serde(rename = "exportId")]
    pub export_id: String,
    /// Configuration payload provided by the user
    pub payload: Value,
}

/// The exported document type
#[derive(Debug, Clone)]
#[derive_wasm]
pub enum ExpoDoc {
    /// Success output. Contains file name and the bytes
    #[serde(rename_all = "camelCase")]
    Success {
        file_name: String,
        file_content: ExpoBlob,
    },
    /// Error output with a message
    Error(String),
}

#[macro_export]
macro_rules! export_error {
    ($msg:literal) => {
        Ok(Some(ExpoDoc::Error($msg.to_string())))
    };
    ($msg:expr) => {
        Ok(Some(ExpoDoc::Error($msg)))
    };
}

impl<'p> ExecContext<'p> {
    pub async fn prepare_exports(mut self) -> ExpoContext<'p> {
        let mut result = Vec::new();
        for plugin in &mut self.plugin_runtimes {
            if let Ok(Some(meta)) = plugin.on_prepare_export().await {
                result.extend(meta);
            }
        }
        ExpoContext {
            exec_ctx: self,
            export_metadata: result,
        }
    }
}

impl<'p> CompDoc<'p> {
    /// Run the export request on this document after the comp phase
    ///
    /// Returning `Some` means the export was successful. Returning `None` means the export is
    /// pending and needed to run in the exec phase
    pub async fn run_exporter(&mut self, req: &ExportRequest) -> Option<ExpoDoc> {
        let mut plugins = std::mem::take(&mut self.plugin_runtimes);

        for plugin in &mut plugins {
            if req.plugin_id == plugin.get_id() {
                let result = match plugin.on_export_comp_doc(&req.export_id, &req.payload, self).await {
                    Ok(None) => None,
                    Ok(Some(expo_doc)) => Some(expo_doc),
                    Err(e) => Some(ExpoDoc::Error(e.to_string())),
                };
                self.plugin_runtimes = plugins;
                return result;
            }
        }

        self.plugin_runtimes = plugins;
        Some(ExpoDoc::Error(format!(
            "Plugin {} not found",
            req.plugin_id
        )))
    }
}

impl<'p> ExecContext<'p> {
    /// Run the export request on this document after the exec phase
    pub async fn run_exporter(self, req: ExportRequest) -> ExpoDoc {
        let mut plugins = self.plugin_runtimes;

        for plugin in &mut plugins {
            if req.plugin_id == plugin.get_id() {
                let result =
                    match plugin.on_export_exec_doc(&req.export_id, req.payload, &self.exec_doc).await {
                        Ok(expo_doc) => expo_doc,
                        Err(e) => ExpoDoc::Error(e.to_string()),
                    };
                return result;
            }
        }

        ExpoDoc::Error(format!("Plugin {} not found", req.plugin_id))
    }
}
