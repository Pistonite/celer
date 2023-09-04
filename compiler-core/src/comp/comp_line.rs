use std::collections::{BTreeMap, HashMap};

use celerctypes::{DocDiagnostic, DocNote, DocRichText, GameCoord};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::json::Coerce;
use crate::lang;
use crate::lang::PresetInst;
use crate::util::async_for;

use super::prop;
use super::{
    validate_not_array_or_object, CompMarker, CompMovement, Compiler, CompilerError, CompilerResult,
};

#[derive(PartialEq, Default, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CompLine {
    /// Primary text content of the line
    pub text: Vec<DocRichText>,
    /// Main line color
    pub line_color: String,
    /// Main movements of this line
    pub movements: Vec<CompMovement>,
    /// Diagnostic messages
    pub diagnostics: Vec<DocDiagnostic>,
    /// Icon id to show on the document
    pub doc_icon: Option<String>,
    /// Icon id to show on the map
    pub map_icon: Option<String>,
    /// Coordinate of the map icon
    pub map_coord: GameCoord,
    /// Map icon priority. 0=primary, 1=secondary, >2=other
    pub map_icon_priority: i64,
    /// Map markers
    pub markers: Vec<CompMarker>,
    /// Secondary text to show below the primary text
    pub secondary_text: Vec<DocRichText>,
    /// Counter text to display
    pub counter_text: Option<DocRichText>,
    /// The notes
    pub notes: Vec<DocNote>,
    /// The split name, if different from text
    pub split_name: Option<Vec<DocRichText>>,
    /// The rest of the properties as json blobs
    ///
    /// These are ignored by ExecDoc, but the transformers can use them
    #[serde(skip)]
    pub properties: HashMap<String, Value>,
}

impl Compiler {
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
    pub async fn comp_line(&mut self, value: Value) -> CompilerResult<CompLine> {
        let mut errors = vec![];

        // Convert line into object form
        let (text, mut line_obj) = match super::desugar_line(value).await {
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
            self.process_presets(0, presets, &mut properties, &mut errors)
                .await;
        }

        if !properties.contains_key(prop::TEXT) {
            properties.insert(prop::TEXT.to_string(), Value::String(text.clone()));
        }

        if text.starts_with('_') {
            let preset_inst = PresetInst::try_parse(&text);
            if let Some(inst) = preset_inst {
                // At this level, we will only process the preset if it exists
                // otherwise treat the string as a regular string
                if self.presets.contains_key(&inst.name) {
                    self.apply_preset(0, &inst, &mut properties, &mut errors)
                        .await;
                }
            }
        }
        properties.extend(line_obj);
        super::desugar_properties(&mut properties).await;
        if let Some(movements) = properties.remove(prop::MOVEMENTS) {
            properties.insert(
                prop::MOVEMENTS.to_string(),
                self.expand_presets_in_movements(0, movements, &mut errors)
                    .await,
            );
        }

        let mut output = self.create_line();

        // Process each property
        for (key, value) in properties.into_iter() {
            self.process_property(key.as_str(), value, &mut output, &mut errors)
                .await;
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
            map_icon_priority: self.default_icon_priority,
            ..Default::default()
        }
    }

    /// Process a property and save it to the output line
    async fn process_property(
        &mut self,
        key: &str,
        value: Value,
        output: &mut CompLine,
        errors: &mut Vec<CompilerError>,
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
                        errors.push(CompilerError::InvalidLinePropertyType(
                            prop::NOTES.to_string(),
                        ));
                        vec![].into_iter()
                    }
                    _ => vec![value].into_iter(),
                };

                let mut notes = vec![];
                async_for!((i, note_value) in iter.enumerate(), {
                    validate_not_array_or_object!(
                        &note_value,
                        errors,
                        format!("{p}[{i}]", p = prop::NOTES)
                    );
                    notes.push(DocNote::Text {
                        content: lang::parse_rich(&note_value.coerce_to_string()),
                    });
                });
                output.notes = notes;
            }
            prop::SPLIT_NAME => {
                if validate_not_array_or_object!(&value, errors, prop::SPLIT_NAME.to_string()) {
                    output.split_name = Some(lang::parse_rich(&value.coerce_to_string()));
                }
            }
            prop::ICON => match value {
                Value::Array(_) => {
                    errors.push(CompilerError::InvalidLinePropertyType(
                        prop::ICON.to_string(),
                    ));
                }
                Value::Object(obj) => {
                    async_for!((key, value) in obj, {
                        match key.as_str() {
                            prop::DOC => {
                                if validate_not_array_or_object!(
                                    &value,
                                    errors,
                                    format!("{}.{}", prop::ICON, prop::DOC)
                                ) {
                                    output.doc_icon = Some(value.coerce_to_string());
                                }
                            }
                            prop::MAP => {
                                if validate_not_array_or_object!(
                                    &value,
                                    errors,
                                    format!("{}.{}", prop::ICON, prop::MAP)
                                ) {
                                    output.map_icon = Some(value.coerce_to_string());
                                }
                            }
                            prop::PRIORITY => {
                                if let Some(i) = value.as_i64() {
                                    output.map_icon_priority = i;
                                } else {
                                    errors.push(CompilerError::InvalidLinePropertyType(format!(
                                        "{}.{}",
                                        prop::ICON, prop::PRIORITY
                                    )));
                                }
                            }
                            key => {
                                errors.push(CompilerError::UnusedProperty(format!(
                                    "{}.{key}",
                                    prop::ICON
                                )));
                            }
                        }
                    });
                }
                _ => {
                    let icon = value.coerce_to_string();
                    output.doc_icon = Some(icon.clone());
                    output.map_icon = Some(icon);
                }
            },
            prop::COUNTER => {
                if validate_not_array_or_object!(&value, errors, prop::COUNTER.to_string()) {
                    let text = value.coerce_to_string();
                    if !text.is_empty() {
                        let mut blocks = lang::parse_rich(&text).into_iter();
                        if let Some(first) = blocks.next() {
                            output.counter_text = Some(first);
                        }
                        if blocks.next().is_some() {
                            errors.push(CompilerError::TooManyTagsInCounter);
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
                        async_for!((i, v) in array.into_iter().enumerate(), {
                            if let Some(m) = self
                                .comp_movement(&format!("{p}[{i}]", p = prop::MOVEMENTS), v, errors)
                                .await
                            {
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
                        });
                        if let Some(i) = ref_stack.last() {
                            if let CompMovement::To { to, .. } = &output.movements[*i] {
                                output.map_coord = to.clone();
                            } else {
                                unreachable!();
                            }
                        }
                    }
                    _ => errors.push(CompilerError::InvalidLinePropertyType(
                        prop::MOVEMENTS.to_string(),
                    )),
                }
            }
            prop::MARKERS => match value {
                Value::Array(array) => {
                    async_for!((i, v) in array.into_iter().enumerate(), {
                        if let Some(m) = self
                            .comp_marker(&format!("{p}[{i}]", p = prop::MARKERS), v, errors)
                            .await
                        {
                            output.markers.push(m);
                        }
                    });
                }
                _ => errors.push(CompilerError::InvalidLinePropertyType(
                    prop::MARKERS.to_string(),
                )),
            },
            _ => {
                output.properties.insert(key.to_string(), value);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use celerctypes::{Axis, DocRichText, GameCoord, MapCoordMap, MapMetadata, RouteMetadata};
    use serde_json::json;

    use crate::comp::test_utils;
    use crate::comp::{CompMarker, CompMovement, CompilerBuilder};
    use crate::lang::Preset;

    use super::*;

    async fn test_comp_ok(compiler: &mut Compiler, input: Value, expected: CompLine) {
        let result = compiler.comp_line(input).await;
        assert_eq!(result, Ok(expected));
    }

    async fn test_comp_err(
        compiler: &mut Compiler,
        input: Value,
        expected: CompLine,
        errors: Vec<CompilerError>,
    ) {
        let result = compiler.comp_line(input).await;
        assert_eq!(result, Err((expected, errors)));
    }

    #[tokio::test]
    async fn test_primitive() {
        let mut compiler = Compiler::default();
        test_comp_ok(&mut compiler, json!(null), CompLine::default()).await;

        test_comp_ok(
            &mut compiler,
            json!(true),
            CompLine {
                text: vec![DocRichText::text("true")],
                ..Default::default()
            },
        )
        .await;

        test_comp_ok(
            &mut compiler,
            json!(false),
            CompLine {
                text: vec![DocRichText::text("false")],
                ..Default::default()
            },
        )
        .await;

        test_comp_ok(
            &mut compiler,
            json!(0),
            CompLine {
                text: vec![DocRichText::text("0")],
                ..Default::default()
            },
        )
        .await;

        test_comp_ok(
            &mut compiler,
            json!(-123),
            CompLine {
                text: vec![DocRichText::text("-123")],
                ..Default::default()
            },
        )
        .await;

        test_comp_ok(
            &mut compiler,
            json!(456),
            CompLine {
                text: vec![DocRichText::text("456")],
                ..Default::default()
            },
        )
        .await;

        test_comp_ok(
            &mut compiler,
            json!("hello world"),
            CompLine {
                text: vec![DocRichText::text("hello world")],
                ..Default::default()
            },
        )
        .await;

        test_comp_ok(
            &mut compiler,
            json!(".tag(foo) world"),
            CompLine {
                text: vec![
                    DocRichText::with_tag("tag", "foo"),
                    DocRichText::text(" world"),
                ],
                ..Default::default()
            },
        )
        .await;
    }

    #[tokio::test]
    async fn test_invalid() {
        let mut compiler = Compiler::default();

        test_comp_err(
            &mut compiler,
            json!([]),
            CompLine {
                text: vec![DocRichText::text("[object array]")],
                ..Default::default()
            },
            vec![CompilerError::ArrayCannotBeLine],
        )
        .await;

        test_comp_err(
            &mut compiler,
            json!({}),
            CompLine {
                text: vec![DocRichText::text("[object object]")],
                ..Default::default()
            },
            vec![CompilerError::EmptyObjectCannotBeLine],
        )
        .await;

        test_comp_err(
            &mut compiler,
            json!({
                "one": {},
                "two": {},
            }),
            CompLine {
                text: vec![DocRichText::text("[object object]")],
                ..Default::default()
            },
            vec![CompilerError::TooManyKeysInObjectLine],
        )
        .await;

        test_comp_err(
            &mut compiler,
            json!({
                "one": "not an object",
            }),
            CompLine {
                text: vec![DocRichText::text("[object object]")],
                ..Default::default()
            },
            vec![CompilerError::LinePropertiesMustBeObject],
        )
        .await;
    }

    #[tokio::test]
    async fn test_text_overrides() {
        let mut compiler = Compiler::default();

        test_comp_ok(
            &mut compiler,
            json!({
                "foo": {
                    "text": "hello world",
                }
            }),
            CompLine {
                text: vec![DocRichText::text("hello world")],
                ..Default::default()
            },
        )
        .await;

        test_comp_err(
            &mut compiler,
            json!({
                "foo": {
                    "text": ["hello world"],
                }
            }),
            CompLine {
                text: vec![DocRichText::text("[object array]")],
                ..Default::default()
            },
            vec![CompilerError::InvalidLinePropertyType("text".to_string())],
        )
        .await;

        test_comp_ok(
            &mut compiler,
            json!({
                "foo": {
                    "comment": "hello world",
                }
            }),
            CompLine {
                text: vec![DocRichText::text("foo")],
                secondary_text: vec![DocRichText::text("hello world")],
                ..Default::default()
            },
        )
        .await;

        test_comp_err(
            &mut compiler,
            json!({
                "foo": {
                    "comment": ["hello world"],
                }
            }),
            CompLine {
                text: vec![DocRichText::text("foo")],
                secondary_text: vec![DocRichText::text("[object array]")],
                ..Default::default()
            },
            vec![CompilerError::InvalidLinePropertyType(
                "comment".to_string(),
            )],
        )
        .await;

        test_comp_ok(
            &mut compiler,
            json!({
                "foo": {
                    "notes": "hello world",
                }
            }),
            CompLine {
                text: vec![DocRichText::text("foo")],
                notes: vec![DocNote::Text {
                    content: vec![DocRichText::text("hello world")],
                }],
                ..Default::default()
            },
        )
        .await;

        test_comp_ok(
            &mut compiler,
            json!({
                "foo": {
                    "notes": ["hello world", "foo bar"],
                }
            }),
            CompLine {
                text: vec![DocRichText::text("foo")],
                notes: vec![
                    DocNote::Text {
                        content: vec![DocRichText::text("hello world")],
                    },
                    DocNote::Text {
                        content: vec![DocRichText::text("foo bar")],
                    },
                ],
                ..Default::default()
            },
        )
        .await;

        test_comp_err(
            &mut compiler,
            json!({
                "foo": {
                    "notes": {},
                }
            }),
            CompLine {
                text: vec![DocRichText::text("foo")],
                ..Default::default()
            },
            vec![CompilerError::InvalidLinePropertyType("notes".to_string())],
        )
        .await;

        test_comp_err(
            &mut compiler,
            json!({
                "foo": {
                    "notes": ["hello", {}],
                    "comment": {},
                }
            }),
            CompLine {
                text: vec![DocRichText::text("foo")],
                notes: vec![
                    DocNote::Text {
                        content: vec![DocRichText::text("hello")],
                    },
                    DocNote::Text {
                        content: vec![DocRichText::text("[object object]")],
                    },
                ],
                secondary_text: vec![DocRichText::text("[object object]")],
                ..Default::default()
            },
            vec![
                CompilerError::InvalidLinePropertyType("comment".to_string()),
                CompilerError::InvalidLinePropertyType("notes[1]".to_string()),
            ],
        )
        .await;

        test_comp_ok(
            &mut compiler,
            json!({
                "foo": {
                    "split-name": "test .v(boo)",
                }
            }),
            CompLine {
                text: vec![DocRichText::text("foo")],
                split_name: Some(vec![
                    DocRichText::text("test "),
                    DocRichText::with_tag("v", "boo"),
                ]),
                ..Default::default()
            },
        )
        .await;

        test_comp_err(
            &mut compiler,
            json!({
                    "foo": {
                    "split-name": ["hello world"],
                }
            }),
            CompLine {
                text: vec![DocRichText::text("foo")],
                ..Default::default()
            },
            vec![CompilerError::InvalidLinePropertyType(
                "split-name".to_string(),
            )],
        )
        .await;

        test_comp_ok(
            &mut compiler,
            json!({
                "foo": {
                    "split-name": "",
                }
            }),
            CompLine {
                text: vec![DocRichText::text("foo")],
                split_name: Some(vec![]),
                ..Default::default()
            },
        )
        .await;
    }

    #[tokio::test]
    async fn test_preset_one_level() {
        let mut builder = CompilerBuilder::default();
        builder
            .add_preset(
                "_preset",
                Preset::compile(json!({
                    "text": "hello world",
                    "comment": "foo bar",
                }))
                .await
                .unwrap(),
            )
            .add_preset(
                "_notext",
                Preset::compile(json!({
                    "comment": "foo bar",
                }))
                .await
                .unwrap(),
            );
        let mut compiler = builder.build();

        test_comp_ok(
            &mut compiler,
            json!("_preset"),
            CompLine {
                text: vec![DocRichText::text("hello world")],
                secondary_text: vec![DocRichText::text("foo bar")],
                ..Default::default()
            },
        )
        .await;

        test_comp_ok(
            &mut compiler,
            json!({
                "_preset": {
                    "comment": "foo bar 2",
                }
            }),
            CompLine {
                text: vec![DocRichText::text("hello world")],
                secondary_text: vec![DocRichText::text("foo bar 2")],
                ..Default::default()
            },
        )
        .await;

        test_comp_ok(
            &mut compiler,
            json!({
                "_notext": {
                    "comment": "foo bar 2",
                }
            }),
            CompLine {
                text: vec![DocRichText::text("_notext")],
                secondary_text: vec![DocRichText::text("foo bar 2")],
                ..Default::default()
            },
        )
        .await;

        test_comp_ok(
            &mut compiler,
            json!({
                "_notext": {
                    "text": "foo bar 2",
                }
            }),
            CompLine {
                text: vec![DocRichText::text("foo bar 2")],
                secondary_text: vec![DocRichText::text("foo bar")],
                ..Default::default()
            },
        )
        .await;

        test_comp_ok(
            &mut compiler,
            json!({
                "_invalid": {
                    "comment": "foo bar 2",
                }
            }),
            CompLine {
                text: vec![DocRichText::text("_invalid")],
                secondary_text: vec![DocRichText::text("foo bar 2")],
                ..Default::default()
            },
        )
        .await;

        test_comp_ok(
            &mut compiler,
            json!({
                "test": {
                    "text": "_preset",
                }
            }),
            CompLine {
                text: vec![DocRichText::text("_preset")],
                ..Default::default()
            },
        )
        .await;
    }

    #[tokio::test]
    async fn test_preset_nested() {
        let mut builder = CompilerBuilder::default();
        builder
            .add_preset(
                "_preset::one",
                Preset::compile(json!({
                    "comment": "preset one",
                }))
                .await
                .unwrap(),
            )
            .add_preset(
                "_preset::two",
                Preset::compile(json!({
                    "comment": "preset two",
                    "text": "preset two text",
                }))
                .await
                .unwrap(),
            )
            .add_preset(
                "_preset::three",
                Preset::compile(json!({
                    "text": "preset three",
                    "presets": ["_preset::two"]
                }))
                .await
                .unwrap(),
            )
            .add_preset(
                "_preset::four",
                Preset::compile(json!({
                    "text": "preset four: arg is $(0)",
                    "presets": ["_preset::one", "_preset::three"]
                }))
                .await
                .unwrap(),
            )
            .add_preset(
                "_preset::overflow",
                Preset::compile(json!({
                    "presets": ["_preset::overflow"]
                }))
                .await
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
                text: vec![DocRichText::text("preset two text")],
                secondary_text: vec![DocRichText::text("preset one")],
                ..Default::default()
            },
        )
        .await;

        test_comp_err(
            &mut compiler,
            json!({
                "test": {
                    "presets": "foo",
                }
            }),
            CompLine {
                text: vec![DocRichText::text("test")],
                ..Default::default()
            },
            vec![CompilerError::InvalidPresetString("foo".to_string())],
        )
        .await;

        test_comp_err(
            &mut compiler,
            json!({
                "test": {
                    "presets": [{}, "foo", "_foo", "_hello::", 123],
                }
            }),
            CompLine {
                text: vec![DocRichText::text("test")],
                ..Default::default()
            },
            vec![
                CompilerError::InvalidLinePropertyType("presets[0]".to_string()),
                CompilerError::InvalidPresetString("foo".to_string()),
                CompilerError::PresetNotFound("_foo".to_string()),
                CompilerError::InvalidPresetString("_hello::".to_string()),
                CompilerError::InvalidPresetString("123".to_string()),
            ],
        )
        .await;

        test_comp_ok(
            &mut compiler,
            json!({
                "_preset::three<1>": {
                    "presets": ["_preset::one"],
                }
            }),
            CompLine {
                text: vec![DocRichText::text("preset three")],
                secondary_text: vec![DocRichText::text("preset two")],
                ..Default::default()
            },
        )
        .await;

        test_comp_ok(
            &mut compiler,
            json!("_preset::four< abcde >"),
            CompLine {
                text: vec![DocRichText::text("preset four: arg is  abcde ")],
                secondary_text: vec![DocRichText::text("preset two")],
                ..Default::default()
            },
        )
        .await;

        test_comp_err(
            &mut compiler,
            json!("_preset::overflow"),
            CompLine {
                text: vec![DocRichText::text("_preset::overflow")],
                ..Default::default()
            },
            vec![CompilerError::MaxPresetDepthExceeded(
                "_preset::overflow".to_string(),
            )],
        )
        .await;
    }

    #[tokio::test]
    async fn test_icon_overrides() {
        let mut compiler = Compiler::default();

        test_comp_ok(
            &mut compiler,
            json!({
                "icon is string": {
                    "icon": "my-icon",
                },
            }),
            CompLine {
                text: vec![DocRichText::text("icon is string")],
                doc_icon: Some("my-icon".to_string()),
                map_icon: Some("my-icon".to_string()),
                ..Default::default()
            },
        )
        .await;

        test_comp_err(
            &mut compiler,
            json!({
                "icon is string": {
                    "icon": ["my-icon"],
                },
            }),
            CompLine {
                text: vec![DocRichText::text("icon is string")],
                ..Default::default()
            },
            vec![CompilerError::InvalidLinePropertyType("icon".to_string())],
        )
        .await;

        test_comp_err(
            &mut compiler,
            json!({
                "icon is array": {
                    "icon": ["my-icon"],
                },
            }),
            CompLine {
                text: vec![DocRichText::text("icon is array")],
                ..Default::default()
            },
            vec![CompilerError::InvalidLinePropertyType("icon".to_string())],
        )
        .await;

        test_comp_ok(
            &mut compiler,
            json!({
                "icon is empty object": {
                    "icon": {},
                },
            }),
            CompLine {
                text: vec![DocRichText::text("icon is empty object")],
                ..Default::default()
            },
        )
        .await;

        test_comp_ok(
            &mut compiler,
            json!({
                "icon is object": {
                    "icon": {
                        "doc": "my-doc-icon",
                        "map": "my-map-icon",
                        "priority": 1,
                    },
                },
            }),
            CompLine {
                text: vec![DocRichText::text("icon is object")],
                doc_icon: Some("my-doc-icon".to_string()),
                map_icon: Some("my-map-icon".to_string()),
                map_icon_priority: 1,
                ..Default::default()
            },
        )
        .await;

        test_comp_err(
            &mut compiler,
            json!({
                "icon is object": {
                    "icon": {
                        "doc":{},
                        "map": ["my-map-icon"],
                        "priority": 1.2,
                        "boo": "foo",
                    },
                },
            }),
            CompLine {
                text: vec![DocRichText::text("icon is object")],
                ..Default::default()
            },
            vec![
                CompilerError::UnusedProperty("icon.boo".to_string()),
                CompilerError::InvalidLinePropertyType("icon.doc".to_string()),
                CompilerError::InvalidLinePropertyType("icon.map".to_string()),
                CompilerError::InvalidLinePropertyType("icon.priority".to_string()),
            ],
        )
        .await;
    }

    #[tokio::test]
    async fn test_default_icon_priority() {
        let mut builder = CompilerBuilder::default();
        builder.set_default_icon_priority(10);
        let mut compiler = builder.build();

        test_comp_ok(
            &mut compiler,
            json!({
                "icon is partial": {
                    "icon": {
                        "map": "my-map-icon",
                    },
                },
            }),
            CompLine {
                text: vec![DocRichText::text("icon is partial")],
                doc_icon: None,
                map_icon: Some("my-map-icon".to_string()),
                map_icon_priority: 10,
                ..Default::default()
            },
        )
        .await;
    }

    #[tokio::test]
    async fn test_counter_override() {
        let mut compiler = Compiler::default();

        test_comp_ok(
            &mut compiler,
            json!({
                "counter is string": {
                    "counter": "hello",
                },
            }),
            CompLine {
                text: vec![DocRichText::text("counter is string")],
                counter_text: Some(DocRichText::text("hello")),
                ..Default::default()
            },
        )
        .await;

        test_comp_ok(
            &mut compiler,
            json!({
                "counter is tagged string": {
                    "counter": ".test(hello)",
                },
            }),
            CompLine {
                text: vec![DocRichText::text("counter is tagged string")],
                counter_text: Some(DocRichText::with_tag("test", "hello")),
                ..Default::default()
            },
        )
        .await;

        test_comp_ok(
            &mut compiler,
            json!({
                "counter is empty tagged string": {
                    "counter": ".test()",
                },
            }),
            CompLine {
                text: vec![DocRichText::text("counter is empty tagged string")],
                counter_text: Some(DocRichText::with_tag("test", "")),
                ..Default::default()
            },
        )
        .await;

        test_comp_ok(
            &mut compiler,
            json!({
                "counter is empty string": {
                    "counter": "",
                },
            }),
            CompLine {
                text: vec![DocRichText::text("counter is empty string")],
                counter_text: None,
                ..Default::default()
            },
        )
        .await;

        test_comp_err(
            &mut compiler,
            json!({
                "counter is invalid": {
                    "counter": ["hello"],
                },
            }),
            CompLine {
                text: vec![DocRichText::text("counter is invalid")],
                ..Default::default()
            },
            vec![CompilerError::InvalidLinePropertyType(
                "counter".to_string(),
            )],
        )
        .await;

        test_comp_err(
            &mut compiler,
            json!({
                "counter is more than one text block": {
                    "counter": ".v(hello) ",
                },
            }),
            CompLine {
                text: vec![DocRichText::text("counter is more than one text block")],
                counter_text: Some(DocRichText::with_tag("v", "hello")),
                ..Default::default()
            },
            vec![CompilerError::TooManyTagsInCounter],
        )
        .await;
    }

    #[tokio::test]
    async fn test_inherit_color_coord() {
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
                text: vec![DocRichText::text("no color or coord")],
                line_color: "color".to_string(),
                map_coord: GameCoord(1.0, 2.0, 3.0),
                ..Default::default()
            },
        )
        .await;
    }

    #[tokio::test]
    async fn test_change_color() {
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
                text: vec![DocRichText::text("change color")],
                line_color: "new-color".to_string(),
                map_coord: GameCoord(1.0, 2.0, 3.0),
                ..Default::default()
            },
        )
        .await;

        test_comp_err(
            &mut compiler,
            json!({
                "change color 2": {
                    "color": ["newer-color"],
                }
            }),
            CompLine {
                text: vec![DocRichText::text("change color 2")],
                line_color: "new-color".to_string(),
                map_coord: GameCoord(1.0, 2.0, 3.0),
                ..Default::default()
            },
            vec![CompilerError::InvalidLinePropertyType("color".to_string())],
        )
        .await;
    }

    #[tokio::test]
    async fn test_change_coord() {
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
        let mut compiler = builder.build();

        test_comp_ok(
            &mut compiler,
            json!({
                "change coord": {
                    "coord": [4.0, 5.0, 6.0],
                }
            }),
            CompLine {
                text: vec![DocRichText::text("change coord")],
                map_coord: GameCoord(4.0, 5.0, 6.0),
                movements: vec![CompMovement::to(GameCoord(4.0, 5.0, 6.0))],
                ..Default::default()
            },
        )
        .await;

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
                text: vec![DocRichText::text("push pop")],
                map_coord: GameCoord(1.0, 2.0, 3.0),
                movements: vec![
                    CompMovement::Push,
                    CompMovement::to(GameCoord(4.0, 5.0, 6.0)),
                    CompMovement::Pop,
                ],
                ..Default::default()
            },
        )
        .await;

        test_comp_err(
            &mut compiler,
            json!({
                "invalid": {
                    "movements": {}
                }
            }),
            CompLine {
                text: vec![DocRichText::text("invalid")],
                map_coord: GameCoord(1.0, 2.0, 3.0),
                ..Default::default()
            },
            vec![CompilerError::InvalidLinePropertyType(
                "movements".to_string(),
            )],
        )
        .await;
    }

    #[tokio::test]
    async fn test_movements_preset() {
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
            .await
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
                text: vec![DocRichText::text("preset")],
                map_coord: GameCoord(7.0, 8.0, 9.0),
                movements: vec![
                    CompMovement::to(GameCoord(3.0, 4.0, 5.0)),
                    CompMovement::to(GameCoord(7.0, 8.0, 9.0)),
                    CompMovement::to(GameCoord(7.0, 8.0, 9.0)),
                ],
                ..Default::default()
            },
        )
        .await;
    }

    #[tokio::test]
    async fn test_markers() {
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
                text: vec![DocRichText::text("test markers")],
                markers: vec![
                    CompMarker {
                        at: GameCoord(1.0, 2.0, 4.0),
                        color: Some("marker 1".to_string()),
                    },
                    CompMarker::at(GameCoord(1.0, 2.0, 3.0)),
                ],
                ..Default::default()
            },
        )
        .await;

        test_comp_err(
            &mut compiler,
            json!({
                "test markers invalid type": {
                    "markers": {}
                }
            }),
            CompLine {
                text: vec![DocRichText::text("test markers invalid type")],
                ..Default::default()
            },
            vec![CompilerError::InvalidLinePropertyType(
                "markers".to_string(),
            )],
        )
        .await;

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
                text: vec![DocRichText::text("test markers invalid marker type")],
                ..Default::default()
            },
            vec![CompilerError::InvalidMarkerType],
        )
        .await;
    }

    #[tokio::test]
    async fn test_unused_properties() {
        let mut compiler = Compiler::default();

        test_comp_ok(
            &mut compiler,
            json!({
                "test": {
                    "unused": "property"
                }
            }),
            CompLine {
                text: vec![DocRichText::text("test")],
                properties: [("unused".to_string(), json!("property"))]
                    .into_iter()
                    .collect(),
                ..Default::default()
            },
        )
        .await;
    }
}
