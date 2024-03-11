//! Exporter plugin for LiveSplit split files

use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::comp::{CompDoc, CompLine};
use crate::expo::{ExpoBlob, ExpoDoc, ExportIcon, ExportMetadata};
use crate::json::Coerce;
use crate::plugin::{PluginResult, PluginRuntime};
use crate::macros::async_trait;
use crate::{export_error, util};
use crate::env::{self, RefCounted};
use crate::res::{Loader, ResPath};

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

        let mut segments = vec![];

        for section in &doc.route {
            let mut split_lines = vec![];
            for line in &section.lines {
                if should_split_on(line, &split_types) {
                    split_lines.push(line);
                }
            }
            let length = split_lines.len();
            for (i, line) in split_lines.iter().enumerate() {
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
                append_segment(&mut segments_xml, &name, line);
            }
        }

        if segments_xml.is_empty() {
            return export_error!("No splits to export. Make sure you selected at least one split type in the settings.");
        }

        let file_content = format!(
            "\
<?xml version=\"1.0\" encoding=\"UTF-8\"?>\
<Run version=\"1.7.0\">\
    <GameIcon/>\
    <GameName/>\
    <CategoryName/>\
    <LayoutPath/>\
    <Metadata>\
        <Run id=\"\"/>\
        <Platform usesEmulator=\"False\"/>\
        <Region/>\
        <Variables/>\
    </Metadata>\
    <Offset>00:00:00</Offset>\
    <AttemptCount>0</AttemptCount>\
    <AttemptHistory/>\
    <Segments>\
        {segments_xml}\
    </Segments>\
    <AutoSplitterSettings/>\
</Run>\
        "
        );

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

fn append_segment(output: &mut String, name: &str, _line: &CompLine) {
    output.push_str(
        "<Segment> \
        <Name>",
    );
    output.push_str(util::xml_escape(name).as_ref());
    output.push_str("</Name><Icon>");
    // TODO #190: encode icon
    output.push_str(
        "</Icon><SplitTimes>\
                <SplitTime name=\"Personal Best\"/>\
            </SplitTimes>\
        <BestSegmentTime />\
        <SegmentHistory />\
    </Segment>",
    );
}

async fn create_segment(
    doc: &CompDoc<'_>, 
    line: &CompLine, 
    name: &str, 
    include_icon: bool,
    webp_compat: WebpCompat,
    icon_cache: &mut BTreeMap<String, Vec<u8>>,
) -> Result<livesplit_core::Segment, String> {
    let mut segment = livesplit_core::Segment::new(name);
    if include_icon {
        if let Some(icon_id) = &line.doc_icon {
            if let Some(icon_bytes) = icon_cache.get(icon_id) {
                segment.set_icon(icon_bytes);
            } else if let Some(icon_url) = doc.config.icons.get(icon_id) {
                let icon_bytes = load_icon(icon_url, webp_compat).await?;
                let icon_bytes: &[u8] = &*icon_bytes;
                segment.set_icon(icon_bytes);
                icon_cache.insert(icon_id.to_string(), icon_bytes.to_vec());
            }
        }
    }

    Ok(segment)
}

/// Compability mode for WebP
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum WebpCompat {
    /// Emit error. This is the default
    Error,
    /// Skip the icon
    Skip
}

async fn load_icon(icon_url: &str, webp_compat: WebpCompat) -> Result<RefCounted<[u8]>, String> {
    let loader = match env::global_loader::get() {
        None => return Err("No global loader available to load the icons for split export!".to_string()),
        Some(loader) => loader,
    };

    let path = ResPath::new_remote_unchecked("", icon_url);
    let data = match loader.load_raw(&path).await {
        Ok(data) => data,
        Err(e) => return Err(format!("Failed to load icon: {e}")),
    };

    if data.starts_with(b"RIFF") {
        if let WebpCompat::Error = webp_compat {
            return Err("RIFF (webp) icons are not supported by LiveSplit. Set the option `webp-compat: skip` to skip invalid webp icons.".to_string())
        }
    }

    Ok(data)
}
