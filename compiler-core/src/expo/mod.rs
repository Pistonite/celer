use crate::exec::ExecContext;

/// Data to define a plugin's export capability
pub struct ExportMetadata {
    /// The id of the plugin. This is the same string in the `use` of the plugin
    pub plugin_id: String,
    /// The target of the exporter
    pub target: ExportTarget,
    /// Name of the export. For example "LiveSplit"
    pub name: String,
    /// File extension of the export. For example "lss"
    pub extension: Option<String>,
    /// Long description. This shows as a tooltip
    pub description: String,
    // todo: icon
}

pub enum ExportTarget {
    /// The exporter only runs for the CompDoc
    CompDoc,
    /// The exporter only runs for the ExecDoc
    ExecDoc,
    /// The exporter should run for both phases,
    /// and produce the output in the ExecDoc phase
    Both,
}

/// The exported document type
pub struct ExpoDoc {
    /// The file name
    pub file_name: String,
    /// The content of the file
    pub bytes: Vec<u8>,
}

impl<'p> ExecContext<'p> {
    pub fn prepare_exports(&mut self) -> Vec<ExportMetadata> {
        let mut result = Vec::new();
        for plugin in &mut self.plugin_runtimes {
            // TODO #33: error handling
            if let Ok(Some(meta)) = plugin.on_prepare_export() {
                result.push(meta);
            }
        }
        result
    }
}
