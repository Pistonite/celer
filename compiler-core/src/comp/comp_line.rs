use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::json::Coerce;
use crate::lang;
use crate::lang::PresetInst;
use crate::prop;
use crate::types::{DocDiagnostic, DocNote, DocRichText, DocRichTextBlock, GameCoord};

use super::{
    validate_not_array_or_object, CompError, CompMarker, CompMovement, Compiler, CompilerResult,
};

#[derive(PartialEq, Default, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CompLine {
    /// Primary text content of the line
    pub text: DocRichText,
    /// Main line color
    pub line_color: String,
    /// Main movements of this line
    pub movements: Vec<CompMovement>,
    /// Diagnostic messages
    pub diagnostics: Vec<DocDiagnostic>,
    /// Icon id to show on the document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc_icon: Option<String>,
    /// Icon id to show on the map
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_icon: Option<String>,
    /// Coordinate of the map icon
    pub map_coord: GameCoord,
    /// Map icon priority. 0=primary, 1=secondary, >2=other
    pub map_icon_priority: i64,
    /// Map markers
    pub markers: Vec<CompMarker>,
    /// Secondary text to show below the primary text
    pub secondary_text: DocRichText,
    /// Counter text to display
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counter_text: Option<DocRichTextBlock>,
    /// The notes
    pub notes: Vec<DocNote>,
    /// The split name, if different from text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split_name: Option<DocRichText>,
    /// If the line is a banner
    pub is_banner: bool,
    /// The rest of the properties as json blobs
    ///
    /// These are ignored by ExecDoc, but the plugins can use them
    pub properties: BTreeMap<String, Value>,
}

impl<'a> Compiler<'a> {
    /// Compile a line
    ///
    /// 1. Text line is turned into {line: {}}
    /// 2. precedence of the presets (later overides previous)
    ///    - uses
    ///    - self text (if the preset doesn't define text)
    ///    - self preset
    ///    - self properties
    ///
    /// Errors are returned as an Err variant with the line and the errors.
    /// Diagnostics are not added to the line.
    pub fn comp_line(&mut self, value: Value) -> CompilerResult<CompLine> {
        let mut errors = vec![];

        // Convert line into object form
        let (text, mut line_obj) = match super::desugar_line(value) {
            Ok(line) => line,
            Err((text, error)) => {
                errors.push(error);
                let output = CompLine {
                    text: lang::parse_rich(&text),
                    ..Default::default()
                };
                return Err((output, errors));
            }
        };

        let mut properties = BTreeMap::new();

        // Process the presets
        if let Some(presets) = line_obj.remove(prop::PRESETS) {
            self.process_presets(0, presets, &mut properties, &mut errors);
        }

        if !properties.contains_key(prop::TEXT) {
            properties.insert(prop::TEXT.to_string(), Value::String(text.clone()));
        }

        if text.starts_with('_') {
            let preset_inst = PresetInst::try_parse(&text);
            if let Some(inst) = preset_inst {
                // At this level, we will only process the preset if it exists
                // otherwise treat the string as a regular string
                if self.meta.presets.contains_key(&inst.name) {
                    self.apply_preset(0, &inst, &mut properties, &mut errors);
                }
            }
        }
        properties.extend(line_obj);
        super::desugar_properties(&mut properties);
        if let Some(movements) = properties.remove(prop::MOVEMENTS) {
            properties.insert(
                prop::MOVEMENTS.to_string(),
                self.expand_presets_in_movements(0, movements, &mut errors),
            );
        }

        let mut output = self.create_line();

        // Process each property
        for (key, value) in properties.into_iter() {
            self.process_property(key.as_str(), value, &mut output, &mut errors)
        }

        if errors.is_empty() {
            Ok(output)
        } else {
            Err((output, errors))
        }
    }

    pub fn create_line(&self) -> CompLine {
        CompLine {
            line_color: self.color.clone(),
            map_coord: self.coord.clone(),
            map_icon_priority: self.meta.default_icon_priority,
            ..Default::default()
        }
    }

    /// Process a property and save it to the output line
    fn process_property(
        &mut self,
        key: &str,
        value: Value,
        output: &mut CompLine,
        errors: &mut Vec<CompError>,
    ) {
        match key {
            prop::TEXT => {
                validate_not_array_or_object!(&value, errors, prop::TEXT.to_string());
                output.text = lang::parse_rich(&value.coerce_to_string());
            }
            prop::COMMENT => {
                validate_not_array_or_object!(&value, errors, prop::COMMENT.to_string());
                output.secondary_text = lang::parse_rich(&value.coerce_to_string());
            }
            prop::NOTES => {
                let iter = match value {
                    Value::Array(arr) => arr.into_iter(),
                    Value::Object(_) => {
                        errors.push(CompError::InvalidLinePropertyType(prop::NOTES.to_string()));
                        vec![].into_iter()
                    }
                    _ => vec![value].into_iter(),
                };

                let mut notes = vec![];
                for (i, note_value) in iter.enumerate() {
                    validate_not_array_or_object!(
                        &note_value,
                        errors,
                        format!("{p}[{i}]", p = prop::NOTES)
                    );
                    notes.push(DocNote::Text {
                        content: lang::parse_rich(&note_value.coerce_to_string()),
                    });
                }
                output.notes = notes;
            }
            prop::SPLIT_NAME => {
                if validate_not_array_or_object!(&value, errors, prop::SPLIT_NAME.to_string()) {
                    output.split_name = Some(lang::parse_rich(&value.coerce_to_string()));
                }
            }
            prop::ICON_DOC => {
                if validate_not_array_or_object!(&value, errors, prop::ICON_DOC.to_string()) {
                    if value.coerce_truthy() {
                        output.doc_icon = Some(value.coerce_to_string());
                    } else {
                        output.doc_icon = None;
                    }
                }
            }
            prop::ICON_MAP => {
                if validate_not_array_or_object!(&value, errors, prop::ICON_MAP.to_string()) {
                    if value.coerce_truthy() {
                        output.map_icon = Some(value.coerce_to_string());
                    } else {
                        output.map_icon = None;
                    }
                }
            }
            prop::ICON_PRIORITY => {
                if validate_not_array_or_object!(&value, errors, prop::ICON_PRIORITY.to_string()) {
                    if let Some(i) = value.try_coerce_to_i64() {
                        output.map_icon_priority = i;
                    } else {
                        errors.push(CompError::InvalidLinePropertyType(
                            prop::ICON_PRIORITY.to_string(),
                        ));
                    }
                }
            }
            prop::COUNTER => {
                if validate_not_array_or_object!(&value, errors, prop::COUNTER.to_string()) {
                    let text = value.coerce_to_string();
                    if !text.is_empty() {
                        let mut blocks = lang::parse_rich(&text).into_iter();
                        if let Some(first) = blocks.next() {
                            output.counter_text = Some(first);
                        }
                        if blocks.next().is_some() {
                            errors.push(CompError::TooManyTagsInCounter);
                        }
                    }
                }
            }
            prop::COLOR => {
                if validate_not_array_or_object!(&value, errors, prop::COLOR.to_string()) {
                    let new_color = value.coerce_to_string();
                    output.line_color = new_color.clone();
                    self.color = new_color;
                }
            }
            prop::MOVEMENTS => {
                match value {
                    Value::Array(array) => {
                        // need to track the coordinate of the final position with a stack
                        let mut ref_stack = vec![];
                        for (i, v) in array.into_iter().enumerate() {
                            if let Some(m) = self.comp_movement(
                                &format!("{p}[{i}]", p = prop::MOVEMENTS),
                                v,
                                errors,
                            ) {
                                match &m {
                                    CompMovement::Push => {
                                        if let Some(i) = ref_stack.last() {
                                            ref_stack.push(*i);
                                        }
                                    }
                                    CompMovement::Pop => {
                                        ref_stack.pop();
                                    }
                                    _ => match ref_stack.last_mut() {
                                        Some(i) => *i = output.movements.len(),
                                        None => ref_stack.push(output.movements.len()),
                                    },
                                }
                                output.movements.push(m);
                            }
                        }
                        if let Some(i) = ref_stack.last() {
                            if let CompMovement::To { to, .. } = &output.movements[*i] {
                                output.map_coord = to.clone();
                                self.coord = to.clone();
                            } else {
                                unreachable!();
                            }
                        }
                    }
                    _ => errors.push(CompError::InvalidLinePropertyType(
                        prop::MOVEMENTS.to_string(),
                    )),
                }
            }
            prop::MARKERS => match value {
                Value::Array(array) => {
                    for (i, v) in array.into_iter().enumerate() {
                        if let Some(m) =
                            self.comp_marker(&format!("{p}[{i}]", p = prop::MARKERS), v, errors)
                        {
                            output.markers.push(m);
                        }
                    }
                }
                _ => errors.push(CompError::InvalidLinePropertyType(
                    prop::MARKERS.to_string(),
                )),
            },
            prop::BANNER => match value.try_coerce_to_bool() {
                Some(value) => output.is_banner = value,
                None => {
                    errors.push(CompError::InvalidLinePropertyType(prop::BANNER.to_string()));
                }
            },
            _ => {
                output.properties.insert(key.to_string(), value);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use map_macro::btree_map;
    use serde_json::json;

    use crate::comp::test_utils;
    use crate::comp::{CompMarker, CompMovement, CompilerBuilder};
    use crate::lang::Preset;
    use crate::types::{Axis, DocRichText, GameCoord, MapCoordMap, MapMetadata, RouteMetadata};

    use super::*;

    fn test_comp_ok(compiler: &mut Compiler<'static>, input: Value, expected: CompLine) {
        let result = compiler.comp_line(input);
        assert_eq!(result, Ok(expected));
    }

    fn test_comp_err(
        compiler: &mut Compiler<'static>,
        input: Value,
        expected: CompLine,
        errors: Vec<CompError>,
    ) {
        let result = compiler.comp_line(input);
        assert_eq!(result, Err((expected, errors)));
    }

    #[test]
    fn test_primitive() {
        let mut compiler = Compiler::default();
        test_comp_ok(&mut compiler, json!(null), CompLine::default());

        test_comp_ok(
            &mut compiler,
            json!(true),
            CompLine {
                text: lang::parse_rich("true"),
                ..Default::default()
            },
        );

        test_comp_ok(
            &mut compiler,
            json!(false),
            CompLine {
                text: lang::parse_rich("false"),
                ..Default::default()
            },
        );

        test_comp_ok(
            &mut compiler,
            json!(0),
            CompLine {
                text: lang::parse_rich("0"),
                ..Default::default()
            },
        );

        test_comp_ok(
            &mut compiler,
            json!(-123),
            CompLine {
                text: lang::parse_rich("-123"),
                ..Default::default()
            },
        );

        test_comp_ok(
            &mut compiler,
            json!(456),
            CompLine {
                text: lang::parse_rich("456"),
                ..Default::default()
            },
        );

        test_comp_ok(
            &mut compiler,
            json!("hello world"),
            CompLine {
                text: lang::parse_rich("hello world"),
                ..Default::default()
            },
        );

        test_comp_ok(
            &mut compiler,
            json!(".tag(foo) world"),
            CompLine {
                text: lang::parse_rich(".tag(foo) world"),
                ..Default::default()
            },
        );
    }

    #[test]
    fn test_invalid() {
        let mut compiler = Compiler::default();

        test_comp_err(
            &mut compiler,
            json!([]),
            CompLine {
                text: DocRichText::text("[object array]"),
                ..Default::default()
            },
            vec![CompError::ArrayCannotBeLine],
        );

        test_comp_err(
            &mut compiler,
            json!({}),
            CompLine {
                text: DocRichText::text("[object object]"),
                ..Default::default()
            },
            vec![CompError::EmptyObjectCannotBeLine],
        );

        test_comp_err(
            &mut compiler,
            json!({
                "one": {},
                "two": {},
            }),
            CompLine {
                text: DocRichText::text("[object object]"),
                ..Default::default()
            },
            vec![CompError::TooManyKeysInObjectLine],
        );

        test_comp_err(
            &mut compiler,
            json!({
                "one": "not an object",
            }),
            CompLine {
                text: DocRichText::text("[object object]"),
                ..Default::default()
            },
            vec![CompError::LinePropertiesMustBeObject],
        );
    }

    #[test]
    fn test_text_overrides() {
        let mut compiler = Compiler::default();

        test_comp_ok(
            &mut compiler,
            json!({
                "foo": {
                    "text": "hello world",
                }
            }),
            CompLine {
                text: DocRichText::text("hello world"),
                ..Default::default()
            },
        );

        test_comp_err(
            &mut compiler,
            json!({
                "foo": {
                    "text": ["hello world"],
                }
            }),
            CompLine {
                text: DocRichText::text("[object array]"),
                ..Default::default()
            },
            vec![CompError::InvalidLinePropertyType("text".to_string())],
        );

        test_comp_ok(
            &mut compiler,
            json!({
                "foo": {
                    "comment": "hello world",
                }
            }),
            CompLine {
                text: DocRichText::text("foo"),
                secondary_text: DocRichText::text("hello world"),
                ..Default::default()
            },
        );

        test_comp_err(
            &mut compiler,
            json!({
                "foo": {
                    "comment": ["hello world"],
                }
            }),
            CompLine {
                text: DocRichText::text("foo"),
                secondary_text: DocRichText::text("[object array]"),
                ..Default::default()
            },
            vec![CompError::InvalidLinePropertyType("comment".to_string())],
        );

        test_comp_ok(
            &mut compiler,
            json!({
                "foo": {
                    "notes": "hello world",
                }
            }),
            CompLine {
                text: DocRichText::text("foo"),
                notes: vec![DocNote::Text {
                    content: DocRichText::text("hello world"),
                }],
                ..Default::default()
            },
        );

        test_comp_ok(
            &mut compiler,
            json!({
                "foo": {
                    "notes": ["hello world", "foo bar"],
                }
            }),
            CompLine {
                text: DocRichText::text("foo"),
                notes: vec![
                    DocNote::Text {
                        content: DocRichText::text("hello world"),
                    },
                    DocNote::Text {
                        content: DocRichText::text("foo bar"),
                    },
                ],
                ..Default::default()
            },
        );

        test_comp_err(
            &mut compiler,
            json!({
                "foo": {
                    "notes": {},
                }
            }),
            CompLine {
                text: DocRichText::text("foo"),
                ..Default::default()
            },
            vec![CompError::InvalidLinePropertyType("notes".to_string())],
        );

        test_comp_err(
            &mut compiler,
            json!({
                "foo": {
                    "notes": ["hello", {}],
                    "comment": {},
                }
            }),
            CompLine {
                text: DocRichText::text("foo"),
                notes: vec![
                    DocNote::Text {
                        content: DocRichText::text("hello"),
                    },
                    DocNote::Text {
                        content: DocRichText::text("[object object]"),
                    },
                ],
                secondary_text: DocRichText::text("[object object]"),
                ..Default::default()
            },
            vec![
                CompError::InvalidLinePropertyType("comment".to_string()),
                CompError::InvalidLinePropertyType("notes[1]".to_string()),
            ],
        );

        test_comp_ok(
            &mut compiler,
            json!({
                "foo": {
                    "split-name": "test .v(boo)",
                }
            }),
            CompLine {
                text: DocRichText::text("foo"),
                split_name: Some(DocRichText(vec![
                    DocRichTextBlock::text("test "),
                    DocRichTextBlock::with_tag("v", "boo"),
                ])),
                ..Default::default()
            },
        );

        test_comp_err(
            &mut compiler,
            json!({
                    "foo": {
                    "split-name": ["hello world"],
                }
            }),
            CompLine {
                text: DocRichText::text("foo"),
                ..Default::default()
            },
            vec![CompError::InvalidLinePropertyType("split-name".to_string())],
        );

        test_comp_ok(
            &mut compiler,
            json!({
                "foo": {
                    "split-name": "",
                }
            }),
            CompLine {
                text: DocRichText::text("foo"),
                split_name: Some(DocRichText(vec![])),
                ..Default::default()
            },
        );
    }

    #[test]
    fn test_preset_one_level() {
        let mut builder = CompilerBuilder::default();
        builder
            .add_preset(
                "_preset",
                Preset::compile(json!({
                    "text": "hello world",
                    "comment": "foo bar",
                }))
                .unwrap(),
            )
            .add_preset(
                "_notext",
                Preset::compile(json!({
                    "comment": "foo bar",
                }))
                .unwrap(),
            );
        let mut compiler = builder.build();

        test_comp_ok(
            &mut compiler,
            json!("_preset"),
            CompLine {
                text: DocRichText::text("hello world"),
                secondary_text: DocRichText::text("foo bar"),
                ..Default::default()
            },
        );

        test_comp_ok(
            &mut compiler,
            json!({
                "_preset": {
                    "comment": "foo bar 2",
                }
            }),
            CompLine {
                text: DocRichText::text("hello world"),
                secondary_text: DocRichText::text("foo bar 2"),
                ..Default::default()
            },
        );

        test_comp_ok(
            &mut compiler,
            json!({
                "_notext": {
                    "comment": "foo bar 2",
                }
            }),
            CompLine {
                text: DocRichText::text("_notext"),
                secondary_text: DocRichText::text("foo bar 2"),
                ..Default::default()
            },
        );

        test_comp_ok(
            &mut compiler,
            json!({
                "_notext": {
                    "text": "foo bar 2",
                }
            }),
            CompLine {
                text: DocRichText::text("foo bar 2"),
                secondary_text: DocRichText::text("foo bar"),
                ..Default::default()
            },
        );

        test_comp_ok(
            &mut compiler,
            json!({
                "_invalid": {
                    "comment": "foo bar 2",
                }
            }),
            CompLine {
                text: DocRichText::text("_invalid"),
                secondary_text: DocRichText::text("foo bar 2"),
                ..Default::default()
            },
        );

        test_comp_ok(
            &mut compiler,
            json!({
                "test": {
                    "text": "_preset",
                }
            }),
            CompLine {
                text: DocRichText::text("_preset"),
                ..Default::default()
            },
        );
    }

    #[test]
    fn test_preset_nested() {
        let mut builder = CompilerBuilder::default();
        builder
            .add_preset(
                "_preset::one",
                Preset::compile(json!({
                    "comment": "preset one",
                }))
                .unwrap(),
            )
            .add_preset(
                "_preset::two",
                Preset::compile(json!({
                    "comment": "preset two",
                    "text": "preset two text",
                }))
                .unwrap(),
            )
            .add_preset(
                "_preset::three",
                Preset::compile(json!({
                    "text": "preset three",
                    "presets": ["_preset::two"]
                }))
                .unwrap(),
            )
            .add_preset(
                "_preset::four",
                Preset::compile(json!({
                    "text": "preset four: arg is $(0)",
                    "presets": ["_preset::one", "_preset::three"]
                }))
                .unwrap(),
            )
            .add_preset(
                "_preset::overflow",
                Preset::compile(json!({
                    "presets": ["_preset::overflow"]
                }))
                .unwrap(),
            );
        let mut compiler = builder.build();

        test_comp_ok(
            &mut compiler,
            json!({
                "_preset::one": {
                    "presets": ["_preset::two"],
                }
            }),
            CompLine {
                text: DocRichText::text("preset two text"),
                secondary_text: DocRichText::text("preset one"),
                ..Default::default()
            },
        );

        test_comp_err(
            &mut compiler,
            json!({
                "test": {
                    "presets": "foo",
                }
            }),
            CompLine {
                text: DocRichText::text("test"),
                ..Default::default()
            },
            vec![CompError::InvalidPresetString("foo".to_string())],
        );

        test_comp_err(
            &mut compiler,
            json!({
                "test": {
                    "presets": [{}, "foo", "_foo", "_hello::", 123],
                }
            }),
            CompLine {
                text: DocRichText::text("test"),
                ..Default::default()
            },
            vec![
                CompError::InvalidLinePropertyType("presets[0]".to_string()),
                CompError::InvalidPresetString("foo".to_string()),
                CompError::PresetNotFound("_foo".to_string()),
                CompError::InvalidPresetString("_hello::".to_string()),
                CompError::InvalidPresetString("123".to_string()),
            ],
        );

        test_comp_ok(
            &mut compiler,
            json!({
                "_preset::three<1>": {
                    "presets": ["_preset::one"],
                }
            }),
            CompLine {
                text: DocRichText::text("preset three"),
                secondary_text: DocRichText::text("preset two"),
                ..Default::default()
            },
        );

        test_comp_ok(
            &mut compiler,
            json!("_preset::four< abcde >"),
            CompLine {
                text: DocRichText::text("preset four: arg is  abcde "),
                secondary_text: DocRichText::text("preset two"),
                ..Default::default()
            },
        );

        test_comp_err(
            &mut compiler,
            json!("_preset::overflow"),
            CompLine {
                text: DocRichText::text("_preset::overflow"),
                ..Default::default()
            },
            vec![CompError::MaxPresetDepthExceeded(
                "_preset::overflow".to_string(),
            )],
        );
    }

    #[test]
    fn test_icon_overrides() {
        let mut compiler = Compiler::default();

        test_comp_ok(
            &mut compiler,
            json!({
                "icon is string": {
                    "icon": "my-icon",
                },
            }),
            CompLine {
                text: DocRichText::text("icon is string"),
                doc_icon: Some("my-icon".to_string()),
                map_icon: Some("my-icon".to_string()),
                ..Default::default()
            },
        );

        test_comp_err(
            &mut compiler,
            json!({
                "icon is string": {
                    "icon": ["my-icon"],
                },
            }),
            CompLine {
                text: DocRichText::text("icon is string"),
                ..Default::default()
            },
            vec![
                CompError::InvalidLinePropertyType("icon-doc".to_string()),
                CompError::InvalidLinePropertyType("icon-map".to_string()),
            ],
        );

        test_comp_err(
            &mut compiler,
            json!({
                "icon is array": {
                    "icon": ["my-icon"],
                },
            }),
            CompLine {
                text: DocRichText::text("icon is array"),
                ..Default::default()
            },
            vec![
                CompError::InvalidLinePropertyType("icon-doc".to_string()),
                CompError::InvalidLinePropertyType("icon-map".to_string()),
            ],
        );

        test_comp_err(
            &mut compiler,
            json!({
                "icon is empty object": {
                    "icon": {}
                },
            }),
            CompLine {
                text: DocRichText::text("icon is empty object"),
                ..Default::default()
            },
            vec![
                CompError::InvalidLinePropertyType("icon-doc".to_string()),
                CompError::InvalidLinePropertyType("icon-map".to_string()),
            ],
        );

        test_comp_ok(
            &mut compiler,
            json!({
                "icon all 3": {
                    "icon-doc": "my-doc-icon",
                    "icon-map": "my-map-icon",
                    "icon-priority": "1",
                },
            }),
            CompLine {
                text: DocRichText::text("icon all 3"),
                doc_icon: Some("my-doc-icon".to_string()),
                map_icon: Some("my-map-icon".to_string()),
                map_icon_priority: 1,
                ..Default::default()
            },
        );

        test_comp_err(
            &mut compiler,
            json!({
                "icon is object": {
                    "icon-doc":{},
                    "icon-map": ["my-map-icon"],
                    "icon-priority": 1.2,
                    "icon-boo": "foo",
                },
            }),
            CompLine {
                text: DocRichText::text("icon is object"),
                properties: btree_map! {
                    "icon-boo".to_string() => json!("foo"),
                },
                ..Default::default()
            },
            vec![
                CompError::InvalidLinePropertyType("icon-doc".to_string()),
                CompError::InvalidLinePropertyType("icon-map".to_string()),
                CompError::InvalidLinePropertyType("icon-priority".to_string()),
            ],
        );
    }

    #[test]
    fn test_default_icon_priority() {
        let mut builder = CompilerBuilder::default();
        builder.set_default_icon_priority(10);
        let mut compiler = builder.build();

        test_comp_ok(
            &mut compiler,
            json!({
                "icon is partial": {
                    "icon-map": "my-map-icon",
                },
            }),
            CompLine {
                text: DocRichText::text("icon is partial"),
                doc_icon: None,
                map_icon: Some("my-map-icon".to_string()),
                map_icon_priority: 10,
                ..Default::default()
            },
        );
    }

    #[test]
    fn test_icon_hide() {
        let mut builder = CompilerBuilder::default();
        builder.add_preset(
            "_Example",
            Preset::compile(json!({
                "icon-doc": "my-doc-icon",
                "icon-map": "my-map-icon",
            }))
            .unwrap(),
        );
        let mut compiler = builder.build();

        test_comp_ok(
            &mut compiler,
            json!({
                "_Example": {
                    "icon-map": null,
                },
            }),
            CompLine {
                text: DocRichText::text("_Example"),
                map_icon: None,
                doc_icon: Some("my-doc-icon".to_string()),
                ..Default::default()
            },
        );

        test_comp_ok(
            &mut compiler,
            json!({
                "_Example": {
                    "icon-doc": false,
                },
            }),
            CompLine {
                text: DocRichText::text("_Example"),
                doc_icon: None,
                map_icon: Some("my-map-icon".to_string()),
                ..Default::default()
            },
        );
    }

    #[test]
    fn test_counter_override() {
        let mut compiler = Compiler::default();

        test_comp_ok(
            &mut compiler,
            json!({
                "counter is string": {
                    "counter": "hello",
                },
            }),
            CompLine {
                text: DocRichText::text("counter is string"),
                counter_text: Some(DocRichTextBlock::text("hello")),
                ..Default::default()
            },
        );

        test_comp_ok(
            &mut compiler,
            json!({
                "counter is tagged string": {
                    "counter": ".test(hello)",
                },
            }),
            CompLine {
                text: DocRichText::text("counter is tagged string"),
                counter_text: Some(DocRichTextBlock::with_tag("test", "hello")),
                ..Default::default()
            },
        );

        test_comp_ok(
            &mut compiler,
            json!({
                "counter is empty tagged string": {
                    "counter": ".test()",
                },
            }),
            CompLine {
                text: DocRichText::text("counter is empty tagged string"),
                counter_text: Some(DocRichTextBlock::with_tag("test", "")),
                ..Default::default()
            },
        );

        test_comp_ok(
            &mut compiler,
            json!({
                "counter is empty string": {
                    "counter": "",
                },
            }),
            CompLine {
                text: DocRichText::text("counter is empty string"),
                counter_text: None,
                ..Default::default()
            },
        );

        test_comp_err(
            &mut compiler,
            json!({
                "counter is invalid": {
                    "counter": ["hello"],
                },
            }),
            CompLine {
                text: DocRichText::text("counter is invalid"),
                ..Default::default()
            },
            vec![CompError::InvalidLinePropertyType("counter".to_string())],
        );

        test_comp_err(
            &mut compiler,
            json!({
                "counter is more than one text block": {
                    "counter": ".v(hello) foo",
                },
            }),
            CompLine {
                text: DocRichText::text("counter is more than one text block"),
                counter_text: Some(DocRichTextBlock::with_tag("v", "hello")),
                ..Default::default()
            },
            vec![CompError::TooManyTagsInCounter],
        );
    }

    #[test]
    fn test_inherit_color_coord() {
        let builder = CompilerBuilder::new(
            Default::default(),
            "color".to_string(),
            GameCoord(1.0, 2.0, 3.0),
        );
        let mut compiler = builder.build();

        test_comp_ok(
            &mut compiler,
            json!("no color or coord"),
            CompLine {
                text: DocRichText::text("no color or coord"),
                line_color: "color".to_string(),
                map_coord: GameCoord(1.0, 2.0, 3.0),
                ..Default::default()
            },
        );
    }

    #[test]
    fn test_change_color() {
        let builder = CompilerBuilder::new(
            Default::default(),
            "color".to_string(),
            GameCoord(1.0, 2.0, 3.0),
        );
        let mut compiler = builder.build();

        test_comp_ok(
            &mut compiler,
            json!({
                "change color": {
                    "color": "new-color",
                }
            }),
            CompLine {
                text: DocRichText::text("change color"),
                line_color: "new-color".to_string(),
                map_coord: GameCoord(1.0, 2.0, 3.0),
                ..Default::default()
            },
        );

        test_comp_err(
            &mut compiler,
            json!({
                "change color 2": {
                    "color": ["newer-color"],
                }
            }),
            CompLine {
                text: DocRichText::text("change color 2"),
                line_color: "new-color".to_string(),
                map_coord: GameCoord(1.0, 2.0, 3.0),
                ..Default::default()
            },
            vec![CompError::InvalidLinePropertyType("color".to_string())],
        );
    }

    #[test]
    fn test_change_coord() {
        let builder = CompilerBuilder::new(
            RouteMetadata {
                map: MapMetadata {
                    coord_map: MapCoordMap {
                        mapping_3d: (Axis::X, Axis::Y, Axis::Z),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
            "".to_string(),
            GameCoord(1.0, 2.0, 3.0),
        );
        let mut compiler = builder.clone().build();

        test_comp_ok(
            &mut compiler,
            json!({
                "change coord": {
                    "coord": [4.0, 5.0, 6.0],
                }
            }),
            CompLine {
                text: DocRichText::text("change coord"),
                map_coord: GameCoord(4.0, 5.0, 6.0),
                movements: vec![CompMovement::to(GameCoord(4.0, 5.0, 6.0))],
                ..Default::default()
            },
        );
        assert_eq!(compiler.coord, GameCoord(4.0, 5.0, 6.0));

        let mut compiler = builder.clone().build();
        test_comp_ok(
            &mut compiler,
            json!({
                "push pop": {
                    "movements": [
                        "push",
                        [4.0, 5.0, 6.0],
                        "pop",
                    ]
                }
            }),
            CompLine {
                text: DocRichText::text("push pop"),
                map_coord: GameCoord(1.0, 2.0, 3.0),
                movements: vec![
                    CompMovement::Push,
                    CompMovement::to(GameCoord(4.0, 5.0, 6.0)),
                    CompMovement::Pop,
                ],
                ..Default::default()
            },
        );

        let mut compiler = builder.build();
        test_comp_err(
            &mut compiler,
            json!({
                "invalid": {
                    "movements": {}
                }
            }),
            CompLine {
                text: DocRichText::text("invalid"),
                map_coord: GameCoord(1.0, 2.0, 3.0),
                ..Default::default()
            },
            vec![CompError::InvalidLinePropertyType("movements".to_string())],
        );
    }

    #[test]
    fn test_movements_preset() {
        let mut builder = CompilerBuilder::new(
            RouteMetadata {
                map: MapMetadata {
                    coord_map: MapCoordMap {
                        mapping_3d: (Axis::X, Axis::Y, Axis::Z),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
            "".to_string(),
            GameCoord(1.0, 2.0, 3.0),
        );
        builder.add_preset(
            "_preset::one",
            Preset::compile(json!({
                "movements": [
                    [7, "8", 9],
                    [7, "8", 9],
                ]
            }))
            .unwrap(),
        );
        let mut compiler = builder.build();

        test_comp_ok(
            &mut compiler,
            json!({
                "preset": {
                    "movements": [
                        [3, 4, 5],
                        "_preset::one",
                    ]
                }
            }),
            CompLine {
                text: DocRichText::text("preset"),
                map_coord: GameCoord(7.0, 8.0, 9.0),
                movements: vec![
                    CompMovement::to(GameCoord(3.0, 4.0, 5.0)),
                    CompMovement::to(GameCoord(7.0, 8.0, 9.0)),
                    CompMovement::to(GameCoord(7.0, 8.0, 9.0)),
                ],
                ..Default::default()
            },
        );
    }

    #[test]
    fn test_markers() {
        let mut compiler = test_utils::create_test_compiler_with_coord_transform();

        test_comp_ok(
            &mut compiler,
            json!({
                "test markers": {
                    "markers": [
                        {"at": [1, 2, 4], "color": "marker 1"},
                        [1, "2", 3]
                    ]
                }
            }),
            CompLine {
                text: DocRichText::text("test markers"),
                markers: vec![
                    CompMarker {
                        at: GameCoord(1.0, 2.0, 4.0),
                        color: Some("marker 1".to_string()),
                    },
                    CompMarker::at(GameCoord(1.0, 2.0, 3.0)),
                ],
                ..Default::default()
            },
        );

        test_comp_err(
            &mut compiler,
            json!({
                "test markers invalid type": {
                    "markers": {}
                }
            }),
            CompLine {
                text: DocRichText::text("test markers invalid type"),
                ..Default::default()
            },
            vec![CompError::InvalidLinePropertyType("markers".to_string())],
        );

        test_comp_err(
            &mut compiler,
            json!({
                "test markers invalid marker type": {
                    "markers": [
                        "hello"
                    ]
                }
            }),
            CompLine {
                text: DocRichText::text("test markers invalid marker type"),
                ..Default::default()
            },
            vec![CompError::InvalidMarkerType],
        );
    }

    #[test]
    fn test_unused_properties() {
        let mut compiler = Compiler::default();

        test_comp_ok(
            &mut compiler,
            json!({
                "test": {
                    "unused": "property"
                }
            }),
            CompLine {
                text: DocRichText::text("test"),
                properties: [("unused".to_string(), json!("property"))]
                    .into_iter()
                    .collect(),
                ..Default::default()
            },
        );
    }

    #[test]
    fn test_banner() {
        let mut compiler = Compiler::default();

        test_comp_ok(
            &mut compiler,
            json!({
                "test": {
                    "banner": "true"
                }
            }),
            CompLine {
                text: DocRichText::text("test"),
                is_banner: true,
                ..Default::default()
            },
        );

        test_comp_ok(
            &mut compiler,
            json!({
                "test": {
                    "banner": true
                }
            }),
            CompLine {
                text: DocRichText::text("test"),
                is_banner: true,
                ..Default::default()
            },
        );

        test_comp_ok(
            &mut compiler,
            json!({
                "test": {
                    "banner": false
                }
            }),
            CompLine {
                text: DocRichText::text("test"),
                is_banner: false,
                ..Default::default()
            },
        );
    }
}
