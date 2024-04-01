//! Exporter plugin for mist split files

use std::borrow::Cow;
use std::collections::BTreeSet;

use mist_core::timer::Run;

use serde_json::Value;

use super::should_split_on;

use crate::comp::CompDoc;
use crate::expo::{ExpoBlob, ExpoDoc, ExportIcon, ExportMetadata};
use crate::export_error;
use crate::json::Coerce;
use crate::macros::async_trait;
use crate::plugin::{PluginResult, Runtime};

pub struct ExportMist;

#[async_trait(auto)]
impl Runtime for ExportMist {
    fn get_id(&self) -> Cow<'static, str> {
        Cow::Owned(super::Native::ExportMist.id())
    }

    async fn on_prepare_export(&mut self) -> PluginResult<Option<Vec<ExportMetadata>>> {
        let meta = ExportMetadata {
            plugin_id: self.get_id().into_owned(),
            name: "mist".into(),
            description: "Export to a mist split file".into(),
            icon: ExportIcon::Data,
            extension: Some("msf".into()),
            export_id: None,
            example_config: Some(include_str!("./export_mist.yaml").into()),
            learn_more: Some("/docs/plugin/export-mist#export-mist".into()),
        };
        Ok(Some(vec![meta]))
    }

    async fn on_export_comp_doc<'p>(
        &mut self,
        _: &str,
        payload: &Value,
        doc: &CompDoc<'p>,
    ) -> PluginResult<Option<ExpoDoc>> {
        let payload = match payload.as_object() {
            Some(payload) => payload,
            None => return export_error!("Invalid payload"),
        };
        let mut split_types = BTreeSet::new();
        if let Some(x) = payload.get("split-types") {
            let x = match x.as_array() {
                Some(x) => x,
                _ => return export_error!("Invalid split types"),
            };
            let names: BTreeSet<String> = x.iter().map(|x| x.coerce_to_string()).collect();
            for (tag_name, tag) in doc.config.tags.iter() {
                if let Some(split_type) = &tag.split_type {
                    if names.contains(split_type) {
                        split_types.insert(tag_name.clone());
                    }
                }
            }
        }

        if split_types.is_empty() {
            return export_error!("No splits to export. Make sure you selected at least one split type in the settings.");
        }

        let mut run = Run::empty();
        let mut splits = vec![];
        for section in &doc.route {
            for line in section.lines.iter() {
                if should_split_on(line, &split_types) {
                    splits.push(line.split_name.as_ref().unwrap_or(&line.text).to_string())
                }
            }
        }

        if splits.is_empty() {
            return export_error!("No splits to export. Make sure you selected at least one split type in the settings.");
        }

        run.set_splits(&splits);
        let content = run.to_string();
        if content.is_err() {
            return export_error!("Failed to serialize split file");
        }

        Ok(Some(ExpoDoc::Success {
            file_name: format!("{}.msf", doc.config.meta.title),
            file_content: ExpoBlob::from_utf8(content.unwrap()),
        }))
    }
}
