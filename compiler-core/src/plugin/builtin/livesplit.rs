//! Exporter plugin for LiveSplit split files

use std::borrow::Cow;

use serde_json::json;

use crate::expo::{ExportMetadata, ExportIcon};

use crate::plugin::{PluginRuntime, PluginResult};

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
            properties: json!(null),
            example_config: Some(include_str!("./livesplit.yaml").to_string())
        };
        Ok(Some(vec![metadata]))
    }
}