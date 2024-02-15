//! Exporter plugin for LiveSplit split files

use std::borrow::Cow;
use std::collections::BTreeSet;

use serde_json::Value;

use crate::comp::{CompDoc, CompLine};
use crate::json::Coerce;
use crate::expo::{ExpoBlob, ExpoDoc, ExportIcon, ExportMetadata};
use crate::plugin::{PluginResult, PluginRuntime};
use crate::{export_error, util};

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

    fn on_export_comp_doc(&mut self, _: &str, payload: &Value, doc: &CompDoc) -> PluginResult<Option<ExpoDoc>> {
        let payload = match payload.as_object() {
            Some(payload) => payload,
            None => return export_error!("Invalid payload")
        };
        let icon = payload.get("icons").map(|x| x.coerce_truthy()).unwrap_or(false);
        let subsplit = payload.get("subsplits").map(|x| x.coerce_truthy()).unwrap_or(false);

        // TODO #190: encode icon
        if icon {
            return export_error!("Icon export is not supported yet.");
        }

        // build relevant split types
        let mut split_types = BTreeSet::new();
        if let Some(x) = payload.get("split-types") {
            let x = match x.as_array() {
                Some(x) => x,
                _ => return export_error!("Invalid split-types")
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

        let mut segments_xml = String::new();
        for section in &doc.route {
            let length = section.lines.len();
            for (i, line) in section.lines.iter().enumerate() {
                if should_split_on(line, &split_types) {
                    let mut name = match &line.split_name {
                        Some(name) => name.to_string(),
                        None => line.text.to_string()
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
        }

        if segments_xml.is_empty() {
            return export_error!("No splits to export. Make sure you selected at least one split type in the settings.");
        }

        let file_content = format!("\
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
        ");

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
        None => return false
    };
    let tag = match &counter.tag {
        Some(tag) => tag,
        None => return false
    };

    split_types.contains(tag)
}

fn append_segment(output: &mut String, name: &str, _line: &CompLine) {
    output.push_str("<Segment> \
        <Name>");
    output.push_str(util::xml_escape(name).as_ref());
    output.push_str("</Name><Icon>");
    // TODO #190: encode icon
    output.push_str("</Icon><SplitTimes>\
                <SplitTime name=\"Personal Best\"/>\
            </SplitTimes>\
        <BestSegmentTime />\
        <SegmentHistory />\
    </Segment>");
}
