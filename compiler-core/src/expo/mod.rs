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

use crate::exec::ExecContext;
use crate::macros::derive_wasm;

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

    /// Extra properties to pass to the exporter when running
    ///
    /// This can be used to distinguish multiple exports from the same exporter.
    /// This is not part of the config and cannot be changed by the user
    pub properties: Value,

    /// (Optional) Example YAML configuration for the exporter to show to the user
    pub example_config: Option<String>,
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

/// The exported document type
#[derive(Debug, Clone)]
#[derive_wasm]
pub struct ExpoDoc {
    /// The file name
    pub file_name: String,
    /// The content of the file
    pub bytes: Vec<u8>,
}

impl<'p> ExecContext<'p> {
    pub fn prepare_exports(mut self) -> ExpoContext<'p> {
        let mut result = Vec::new();
        for plugin in &mut self.plugin_runtimes {
            if let Ok(Some(meta)) = plugin.on_prepare_export() {
                result.extend(meta);
            }
        }
        ExpoContext {
            exec_ctx: self,
            export_metadata: result,
        }
    }
}
