//! Exporter plugin for LiveSplit split files

use std::borrow::Cow;

use crate::expo::{ExportIcon, ExportMetadata};

use crate::plugin::{PluginResult, PluginRuntime};

pub struct ExportLiveSplitPlugin;

impl PluginRuntime for ExportLiveSplitPlugin {
    fn get_id(&self) -> Cow<'static, str> {
        Cow::Owned(super::BuiltInPlugin::ExportLiveSplit.id())
    }

    fn on_prepare_export(&mut self) -> PluginResult<Option<Vec<ExportMetadata>>> {
        let metadata = ExportMetadata {
            plugin_id: self.get_id().into_owned(),
            name: "LiveSplit".to_string(),
            description: "Export to a LiveSplit split file".to_string(),
            icon: ExportIcon::Data,
            extension: Some("lss".to_string()),
            export_id: None,
            example_config: Some(include_str!("./livesplit.yaml").to_string()),
            learn_more: Some("/docs/plugin/export-livesplit".to_string()),
        };
        Ok(Some(vec![metadata]))
    }
}
