//! Split format plugin
//!
//! Automatically set the `split-name` property based on the split type

use std::borrow::Cow;
use std::collections::BTreeMap;

use serde_json::Value;

use crate::json::Coerce;
use crate::lang::{self, DocRichText};
use crate::comp::{CompDoc, CompLine};
use crate::plugin::{operation, PluginResult, PluginRuntime};

pub struct SplitFormatPlugin {
    formats: BTreeMap<String, DocRichText>
}

impl SplitFormatPlugin {
    pub fn from_props(props: &Value) -> Self {
        let mut formats = BTreeMap::new();
        if let Some(props) = props.as_object() {
            for (k, v) in props {
                formats.insert(k.clone(), lang::parse_rich(&v.coerce_to_string()));
            }
        }
        Self { formats }
    }
}

impl PluginRuntime for SplitFormatPlugin {
    fn get_id(&self) -> Cow<'static, str> {
        Cow::Owned(super::BuiltInPlugin::SplitFormat.id())
    }
    fn on_after_compile(&mut self, comp_doc: &mut CompDoc) -> PluginResult<()> {
        let mut tag_to_format = BTreeMap::new();
        for (tag_name, tag) in comp_doc.config.tags.iter() {
            if let Some(split_type) = &tag.split_type {
                if let Some(format) = self.formats.get(split_type) {
                    tag_to_format.insert(tag_name.clone(), format);
                }
            }
        }
        operation::for_each_line!(line in comp_doc {
            let mut format = None;
            if let Some(counter) = &line.counter_text {
                if let Some(tag) = &counter.tag {
                    format = tag_to_format.get(tag);
                }
                // this is to get .var(type) to work
                if format.is_none() {
                    format = tag_to_format.get(&counter.text);
                }
            }
            if let Some(format) = format {
                let mut format = (*format).clone();
                transform_format(&mut format, &line);
                line.split_name = Some(format);
            }
        });

        Ok(())
    }
}

/// Transforms the prop tag inside the format
fn transform_format(format: &mut DocRichText, line: &CompLine) {
    for block in &mut format.0 {
        let tag = match &block.tag {
            Some(tag) => tag,
            None => continue,
        };
        if tag != "prop" {
            continue;
        }
        match block.text.as_ref() {
            "text" => {
                block.text = line.text.to_string();
            }
            "comment" => {
                block.text = line.secondary_text.to_string();
            }
            "counter" => {
                block.text = line.counter_text.as_ref().map(|x| x.text.to_string()).unwrap_or_default();
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_no_transform() {
        let line = CompLine::default();
        let mut format = lang::parse_rich("Test no transform");
        let expected = format.clone();
        transform_format(&mut format, &line);
        assert_eq!(format, expected);
    }

    #[test]
    fn test_transform_text() {
        let line = CompLine {
            text: lang::parse_rich("Test Text"),
            ..Default::default()
        };
        let mut format = lang::parse_rich("Test .prop(text)");
        let expected = lang::parse_rich("Test .prop(Test Text)");
        transform_format(&mut format, &line);
        assert_eq!(format, expected);
    }

    #[test]
    fn test_transform_comment() {
        let line = CompLine {
            secondary_text: lang::parse_rich("Test .test(Text)"),
            ..Default::default()
        };
        let mut format = lang::parse_rich("Test .prop(comment)");
        let expected = lang::parse_rich("Test .prop(Test Text)");
        transform_format(&mut format, &line);
        assert_eq!(format, expected);
    }

    #[test]
    fn test_transform_counter() {
        let line = CompLine {
            counter_text: lang::parse_rich(".test(Test Text)").0.into_iter().next(),
            ..Default::default()
        };
        let mut format = lang::parse_rich("Test .prop(counter)");
        let expected = lang::parse_rich("Test .prop(Test Text)");
        transform_format(&mut format, &line);
        assert_eq!(format, expected);
    }
}
