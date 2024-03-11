//! Exporter plugin for LiveSplit split files

use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::comp::{CompDoc, CompLine, CompSection};
use crate::env::{self, yield_budget, RefCounted};
use crate::expo::{ExpoBlob, ExpoDoc, ExportIcon, ExportMetadata};
use crate::export_error;
use crate::json::Coerce;
use crate::macros::async_trait;
use crate::plugin::{PluginResult, PluginRuntime};
use crate::res::ResPath;

pub struct ExportLiveSplitPlugin;

#[async_trait(auto)]
impl PluginRuntime for ExportLiveSplitPlugin {
    fn get_id(&self) -> Cow<'static, str> {
        Cow::Owned(super::BuiltInPlugin::ExportLiveSplit.id())
    }

    async fn on_prepare_export(&mut self) -> PluginResult<Option<Vec<ExportMetadata>>> {
        let metadata = ExportMetadata {
            plugin_id: self.get_id().into_owned(),
            name: "LiveSplit".to_string(),
            description: "Export to a LiveSplit split file".to_string(),
            icon: ExportIcon::Data,
            extension: Some("lss".to_string()),
            export_id: None,
            example_config: Some(include_str!("./export_livesplit.yaml").to_string()),
            learn_more: Some("/docs/plugin/export-livesplit#export-livesplit".to_string()),
        };
        Ok(Some(vec![metadata]))
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
        let icon = payload
            .get("icons")
            .map(|x| x.coerce_truthy())
            .unwrap_or(false);
        let subsplit = payload
            .get("subsplits")
            .map(|x| x.coerce_truthy())
            .unwrap_or(false);
        let webp_compat = match payload.get("webp-compat") {
            None => WebpCompat::Error,
            Some(x) => serde_json::from_value(x.clone()).unwrap_or(WebpCompat::Error),
        };

        // build relevant split types
        let mut split_types = BTreeSet::new();
        if let Some(x) = payload.get("split-types") {
            let x = match x.as_array() {
                Some(x) => x,
                _ => return export_error!("Invalid split-types"),
            };
            // payload contain split display names
            // need to convert to tags
            let names: BTreeSet<String> = x.iter().map(|x| x.coerce_to_string()).collect();
            for (tag_name, tag) in doc.config.tags.iter() {
                if let Some(split_type) = &tag.split_type {
                    if names.contains(split_type) {
                        split_types.insert(tag_name.clone());
                    }
                }
            }
        };

        if split_types.is_empty() {
            return export_error!("No splits to export. Make sure you selected at least one split type in the settings.");
        }

        // build lines to split
        let mut split_sections = Vec::with_capacity(doc.route.len());
        for section in &doc.route {
            let mut split_lines = vec![];
            for line in &section.lines {
                yield_budget(64).await;
                if should_split_on(line, &split_types) {
                    split_lines.push(line);
                }
            }
            if !split_lines.is_empty() {
                split_sections.push((section, split_lines));
            }
        }

        if split_sections.is_empty() {
            return export_error!("No splits to export. Make sure you selected at least one split type in the settings.");
        }

        // build icon cache
        let icon_cache = if icon {
            match build_icon_cache(doc, &split_sections, webp_compat).await {
                Ok(cache) => cache,
                Err(e) => return export_error!(e),
            }
        } else {
            BTreeMap::new()
        };

        let mut run = livesplit_core::Run::new();

        for (section, split_lines) in split_sections {
            let length = split_lines.len();
            for (i, line) in split_lines.iter().enumerate() {
                yield_budget(64).await;
                let mut name = match &line.split_name {
                    Some(name) => name.to_string(),
                    None => line.text.to_string(),
                };
                if subsplit {
                    if i == length - 1 {
                        name = format!("{{{}}}{name}", section.name);
                    } else {
                        name = format!("-{name}");
                    }
                }
                let segment = create_segment(line, &name, icon, &icon_cache);
                run.push_segment(segment);
            }
        }

        let mut file_content = String::new();
        if let Err(e) = livesplit_core::run::saver::livesplit::save_run(&run, &mut file_content) {
            return export_error!(format!("Failed to export to split file: {e}"));
        }

        let file_name = format!("{}.lss", doc.config.meta.title);

        Ok(Some(ExpoDoc::Success {
            file_name,
            file_content: ExpoBlob::from_utf8(file_content),
        }))
    }
}

fn should_split_on(line: &CompLine, split_types: &BTreeSet<String>) -> bool {
    let counter = match &line.counter_text {
        Some(counter) => counter,
        None => return false,
    };
    let tag = match &counter.tag {
        Some(tag) => tag,
        None => return false,
    };

    split_types.contains(tag)
}

async fn build_icon_cache(
    doc: &CompDoc<'_>,
    split_sections: &[(&CompSection, Vec<&CompLine>)],
    webp_compat: WebpCompat,
) -> Result<BTreeMap<String, Vec<u8>>, String> {
    let mut icon_seen = BTreeSet::new();
    let mut icon_futures = vec![];
    for (_, lines) in split_sections {
        for line in lines {
            yield_budget(64).await;
            let icon_id = match &line.doc_icon {
                Some(icon_id) => icon_id,
                None => continue,
            };
            if !icon_seen.insert(icon_id) {
                continue;
            }
            if let Some(icon_url) = doc.config.icons.get(icon_id) {
                icon_futures.push(load_icon(
                    icon_id.to_string(),
                    icon_url.to_string(),
                    webp_compat,
                ))
            }
        }
    }
    let results = env::join_future_vec(icon_futures).await;
    let mut cache = BTreeMap::new();
    for result in results {
        match result {
            Ok(Ok((id, bytes))) => {
                cache.insert(id, bytes.to_vec());
            }
            Ok(Err(e)) | Err(e) => return Err(e),
        }
    }
    Ok(cache)
}

fn create_segment(
    line: &CompLine,
    name: &str,
    include_icon: bool,
    icon_cache: &BTreeMap<String, Vec<u8>>,
) -> livesplit_core::Segment {
    let mut segment = livesplit_core::Segment::new(name);
    if include_icon {
        if let Some(icon_id) = &line.doc_icon {
            if let Some(icon_bytes) = icon_cache.get(icon_id) {
                segment.set_icon(icon_bytes);
            }
        }
    }

    segment
}

/// Compability mode for WebP
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum WebpCompat {
    /// Emit error. This is the default
    Error,
    /// Skip the icon
    Skip,
}

async fn load_icon(
    icon_id: String,
    icon_url: String,
    webp_compat: WebpCompat,
) -> Result<(String, RefCounted<[u8]>), String> {
    let loader = match env::global_loader::get() {
        None => {
            return Err(
                "No global loader available to load the icons for split export!".to_string(),
            )
        }
        Some(loader) => loader,
    };

    let path = ResPath::new_remote_unchecked("", &icon_url);
    let data = match loader.load_raw(&path).await {
        Ok(data) => data,
        Err(e) => return Err(format!("Failed to load icon `{icon_url}`: {e}")),
    };

    if data.starts_with(b"RIFF") {
        if let WebpCompat::Error = webp_compat {
            return Err(format!("Failed to load icon `{icon_url}`: RIFF (webp) icons are not supported by LiveSplit. Set the option `webp-compat: skip` to skip invalid webp icons."));
        }
    }

    Ok((icon_id, data))
}
