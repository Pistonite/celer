use std::collections::BTreeMap;

use celerctypes::DocNote;
use serde_json::{Map, Value};

use crate::json::Coerce;
use crate::lang;
use crate::lang::PresetInst;
use crate::CompLine;

use super::{Compiler, CompilerError, CompilerResult};

const MAX_PRESET_DEPTH: usize = 10;

/// Convenience macro for making a compiler error
macro_rules! make_error {
    ($text:expr, $error:expr) => {
        Err((
            CompLine {
                text: lang::parse_rich($text),
                ..Default::default()
            },
            vec![$error],
        ))
    };
}

/// Convenience macro for validating a json value and add error
macro_rules! validate_not_array_or_object {
    ($value:expr, $errors:ident, $property:literal) => {{
        let v = $value;
        if v.is_array() || v.is_object() {
            $errors.push(CompilerError::InvalidLinePropertyType(
                $property.to_string(),
            ));
            false
        } else {
            true
        }
    }};
    ($value:expr, $errors:ident, $property:expr) => {{
        let v = $value;
        if v.is_array() || v.is_object() {
            $errors.push(CompilerError::InvalidLinePropertyType($property));
            false
        } else {
            true
        }
    }};
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
        // Convert line into object form
        let (text, mut line_obj) = convert_line_to_object(value)?;

        // Process the presets
        let mut errors = vec![];
        let mut properties = BTreeMap::new();
        if let Some(presets) = line_obj.remove("presets") {
            self.process_presets(0, presets, &mut properties, &mut errors)
                .await;
        }

        if !properties.contains_key("text") {
            properties.insert("text".to_string(), Value::String(text.clone()));
        }

        if text.starts_with('_') {
            let preset_inst = PresetInst::try_parse(&text);
            if let Some(inst) = preset_inst {
                // At this level, we will only process the preset if it exists
                // otherwise treat the string as a regular string
                if self.presets.contains_key(&inst.name) {
                    self.process_preset(0, &inst, &mut properties, &mut errors)
                        .await;
                }
            }
        }
        properties.extend(line_obj);
        let mut output = self.create_line().await;

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

    /// Find the preset and fill output with the hydrated values from the preset
    async fn process_preset(
        &self,
        depth: usize,
        inst: &PresetInst,
        output: &mut BTreeMap<String, Value>,
        errors: &mut Vec<CompilerError>,
    ) {
        if depth > MAX_PRESET_DEPTH {
            errors.push(CompilerError::MaxPresetDepthExceeded(inst.name.to_string()));
            return;
        }
        let preset = match self.presets.get(&inst.name) {
            None => {
                errors.push(CompilerError::PresetNotFound(inst.name.to_string()));
                return;
            }
            Some(preset) => preset,
        };

        let mut properties = preset.hydrate(&inst.args).await;
        if let Some(presets) = properties.remove("presets") {
            self.process_presets(depth, presets, output, errors).await;
        }
        output.extend(properties);
    }

    /// Process the "presets" property in the line object
    ///
    /// Saves the properties from the preset to the output map
    #[async_recursion::async_recursion]
    async fn process_presets(
        &self,
        depth: usize,
        presets: Value,
        output: &mut BTreeMap<String, Value>,
        errors: &mut Vec<CompilerError>,
    ) {
        match presets {
            Value::Array(arr) => {
                for (i, preset_value) in arr.into_iter().enumerate() {
                    if validate_not_array_or_object!(&preset_value, errors, format!("presets[{i}]"))
                    {
                        let preset_string = preset_value.coerce_to_string();
                        if !preset_string.starts_with('_') {
                            errors.push(CompilerError::InvalidPresetString(preset_string))
                        } else {
                            let preset_inst = PresetInst::try_parse(&preset_string);
                            match preset_inst {
                                None => {
                                    errors.push(CompilerError::InvalidPresetString(preset_string));
                                }
                                Some(inst) => {
                                    self.process_preset(depth + 1, &inst, output, errors).await;
                                }
                            }
                        }
                    }
                }
            }
            _ => {
                errors.push(CompilerError::InvalidLinePropertyType(
                    "presets".to_string(),
                ));
            }
        }
    }

    async fn create_line(&self) -> CompLine {
        CompLine {
            line_color: self.color.clone(),
            map_coord: self.coord.clone(),
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
            "text" => {
                validate_not_array_or_object!(&value, errors, "text");
                output.text = lang::parse_rich(&value.coerce_to_string());
            }
            "comment" => {
                validate_not_array_or_object!(&value, errors, "comment");
                output.secondary_text = lang::parse_rich(&value.coerce_to_string());
            }
            "notes" => {
                let iter = match value {
                    Value::Array(arr) => arr.into_iter(),
                    Value::Object(_) => {
                        errors.push(CompilerError::InvalidLinePropertyType("notes".to_string()));
                        vec![].into_iter()
                    }
                    _ => vec![value].into_iter(),
                };

                let mut notes = vec![];
                for (i, note_value) in iter.enumerate() {
                    validate_not_array_or_object!(&note_value, errors, format!("notes[{i}]"));
                    notes.push(DocNote::Text {
                        content: lang::parse_rich(&note_value.coerce_to_string()),
                    });
                }
                output.notes = notes;
            }
            "split-name" => {
                if validate_not_array_or_object!(&value, errors, "split-name") {
                    output.split_name = Some(lang::parse_rich(&value.coerce_to_string()));
                }
            }
            "icon" => match value {
                Value::Array(_) => {
                    errors.push(CompilerError::InvalidLinePropertyType("icon".to_string()));
                }
                Value::Object(obj) => {
                    for (key, value) in obj {
                        match key.as_str() {
                            "doc" => {
                                if validate_not_array_or_object!(&value, errors, "icon.doc") {
                                    output.doc_icon = Some(value.coerce_to_string());
                                }
                            }
                            "map" => {
                                if validate_not_array_or_object!(&value, errors, "icon.map") {
                                    output.map_icon = Some(value.coerce_to_string());
                                }
                            }
                            "priority" => {
                                if let Some(i) = value.as_i64() {
                                    output.map_icon_priority = i;
                                } else {
                                    errors.push(CompilerError::InvalidLinePropertyType(
                                        "icon.priority".to_string(),
                                    ));
                                }
                            }
                            key => {
                                errors.push(CompilerError::UnusedProperty(format!("icon.{key}")));
                            }
                        }
                    }
                }
                _ => {
                    let icon = value.coerce_to_string();
                    output.doc_icon = Some(icon.clone());
                    output.map_icon = Some(icon);
                }
            },
            "counter" => {
                if validate_not_array_or_object!(&value, errors, "counter") {
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
            "color" => {
                if validate_not_array_or_object!(&value, errors, "color") {
                    let new_color = value.coerce_to_string();
                    output.line_color = new_color.clone();
                    self.color = new_color;
                }
            }
            _ => todo!(),
        }
    }
}

fn convert_line_to_object(
    value: Value,
) -> Result<(String, serde_json::Map<String, Value>), (CompLine, Vec<CompilerError>)> {
    let text = value.coerce_to_string();
    match value {
        Value::Array(_) => make_error!(&text, CompilerError::ArrayCannotBeLine),
        Value::Object(obj) => {
            let mut iter = obj.into_iter();
            let (key, obj) = match iter.next() {
                None => {
                    return make_error!(&text, CompilerError::EmptyObjectCannotBeLine);
                }
                Some(first) => first,
            };
            if iter.next().is_some() {
                return make_error!(&text, CompilerError::TooManyKeysInObjectLine);
            }
            let properties = match obj {
                Value::Object(map) => map,
                _ => {
                    return make_error!(&text, CompilerError::LinePropertiesMustBeObject);
                }
            };
            Ok((key, properties))
        }
        _ => Ok((value.coerce_to_string(), serde_json::Map::new())),
    }
}

#[cfg(test)]
mod ut {
    use celerctypes::{DocRichText, GameCoord};
    use serde_json::json;

    use crate::{comp::CompilerBuilder, lang::Preset, CompMovement};

    use super::*;

    #[tokio::test]
    async fn test_primitive() {
        let mut compiler = Compiler::default();
        let result = compiler.comp_line(json!(null)).await.unwrap();
        assert_eq!(result, CompLine::default());

        let result = compiler.comp_line(json!(true)).await.unwrap();
        assert_eq!(
            result,
            CompLine {
                text: vec![DocRichText {
                    tag: None,
                    text: "true".to_string()
                }],
                ..Default::default()
            }
        );

        let result = compiler.comp_line(json!(false)).await.unwrap();
        assert_eq!(
            result,
            CompLine {
                text: vec![DocRichText {
                    tag: None,
                    text: "false".to_string()
                }],
                ..Default::default()
            }
        );

        let result = compiler.comp_line(json!(0)).await.unwrap();
        assert_eq!(
            result,
            CompLine {
                text: vec![DocRichText {
                    tag: None,
                    text: "0".to_string()
                }],
                ..Default::default()
            }
        );

        let result = compiler.comp_line(json!(-123)).await.unwrap();
        assert_eq!(
            result,
            CompLine {
                text: vec![DocRichText {
                    tag: None,
                    text: "-123".to_string()
                }],
                ..Default::default()
            }
        );

        let result = compiler.comp_line(json!(456)).await.unwrap();
        assert_eq!(
            result,
            CompLine {
                text: vec![DocRichText {
                    tag: None,
                    text: "456".to_string()
                }],
                ..Default::default()
            }
        );

        let result = compiler.comp_line(json!("hello world")).await.unwrap();
        assert_eq!(
            result,
            CompLine {
                text: vec![DocRichText {
                    tag: None,
                    text: "hello world".to_string()
                }],
                ..Default::default()
            }
        );

        let result = compiler.comp_line(json!(".tag(foo) world")).await.unwrap();
        assert_eq!(
            result,
            CompLine {
                text: vec![
                    DocRichText {
                        tag: Some("tag".to_string()),
                        text: "foo".to_string()
                    },
                    DocRichText {
                        tag: None,
                        text: " world".to_string()
                    }
                ],
                ..Default::default()
            }
        );
    }

    #[tokio::test]
    async fn test_invalid() {
        let mut compiler = Compiler::default();
        let result = compiler.comp_line(json!([])).await.unwrap_err();
        assert_eq!(
            result,
            (
                CompLine {
                    text: vec![DocRichText {
                        tag: None,
                        text: "[object array]".to_string()
                    }],
                    ..Default::default()
                },
                vec![CompilerError::ArrayCannotBeLine]
            )
        );

        let result = compiler.comp_line(json!({})).await.unwrap_err();
        assert_eq!(
            result,
            (
                CompLine {
                    text: vec![DocRichText {
                        tag: None,
                        text: "[object object]".to_string()
                    }],
                    ..Default::default()
                },
                vec![CompilerError::EmptyObjectCannotBeLine]
            )
        );

        let result = compiler
            .comp_line(json!({
                "one": {},
                "two": {},
            }))
            .await
            .unwrap_err();
        assert_eq!(
            result,
            (
                CompLine {
                    text: vec![DocRichText {
                        tag: None,
                        text: "[object object]".to_string()
                    }],
                    ..Default::default()
                },
                vec![CompilerError::TooManyKeysInObjectLine]
            )
        );

        let result = compiler
            .comp_line(json!({
                "one": "not an object",
            }))
            .await
            .unwrap_err();
        assert_eq!(
            result,
            (
                CompLine {
                    text: vec![DocRichText {
                        tag: None,
                        text: "[object object]".to_string()
                    }],
                    ..Default::default()
                },
                vec![CompilerError::LinePropertiesMustBeObject]
            )
        );
    }

    #[tokio::test]
    async fn test_text_overrides() {
        let mut compiler = Compiler::default();
        let result = compiler
            .comp_line(json!({
                "foo": {
                "text": "hello world",
            }}))
            .await
            .unwrap();
        assert_eq!(
            result,
            CompLine {
                text: vec![DocRichText {
                    tag: None,
                    text: "hello world".to_string()
                }],
                ..Default::default()
            }
        );

        let result = compiler
            .comp_line(json!({
                "foo": {
                "text": ["hello world"],
            }}))
            .await
            .unwrap_err();
        assert_eq!(
            result,
            (
                CompLine {
                    text: vec![DocRichText {
                        tag: None,
                        text: "[object array]".to_string()
                    }],
                    ..Default::default()
                },
                vec![CompilerError::InvalidLinePropertyType("text".to_string())]
            )
        );

        let result = compiler
            .comp_line(json!({
                "foo": {
                "comment": "hello world",
            }}))
            .await
            .unwrap();
        assert_eq!(
            result,
            CompLine {
                text: vec![DocRichText {
                    tag: None,
                    text: "foo".to_string()
                }],
                secondary_text: vec![DocRichText {
                    tag: None,
                    text: "hello world".to_string()
                }],
                ..Default::default()
            }
        );

        let result = compiler
            .comp_line(json!({
                "foo": {
                "comment": ["hello world"],
            }}))
            .await
            .unwrap_err();
        assert_eq!(
            result,
            (
                CompLine {
                    text: vec![DocRichText {
                        tag: None,
                        text: "foo".to_string()
                    }],
                    secondary_text: vec![DocRichText {
                        tag: None,
                        text: "[object array]".to_string()
                    }],
                    ..Default::default()
                },
                vec![CompilerError::InvalidLinePropertyType(
                    "comment".to_string()
                )]
            )
        );

        let result = compiler
            .comp_line(json!({
                "foo": {
                "notes": "hello world",
            }}))
            .await
            .unwrap();
        assert_eq!(
            result,
            CompLine {
                text: vec![DocRichText {
                    tag: None,
                    text: "foo".to_string()
                }],
                notes: vec![DocNote::Text {
                    content: vec![DocRichText {
                        tag: None,
                        text: "hello world".to_string()
                    }],
                }],
                ..Default::default()
            }
        );

        let result = compiler
            .comp_line(json!({
                "foo": {
                "notes": ["hello world", "foo bar"],
            }}))
            .await
            .unwrap();
        assert_eq!(
            result,
            CompLine {
                text: vec![DocRichText {
                    tag: None,
                    text: "foo".to_string()
                }],
                notes: vec![
                    DocNote::Text {
                        content: vec![DocRichText {
                            tag: None,
                            text: "hello world".to_string()
                        },],
                    },
                    DocNote::Text {
                        content: vec![DocRichText {
                            tag: None,
                            text: "foo bar".to_string()
                        },],
                    },
                ],
                ..Default::default()
            }
        );

        let result = compiler
            .comp_line(json!({
                "foo": {
                "notes": {},
            }}))
            .await
            .unwrap_err();
        assert_eq!(
            result,
            (
                CompLine {
                    text: vec![DocRichText {
                        tag: None,
                        text: "foo".to_string()
                    }],
                    notes: vec![],
                    ..Default::default()
                },
                vec![CompilerError::InvalidLinePropertyType("notes".to_string())]
            )
        );

        let result = compiler
            .comp_line(json!({
                "foo": {
                "notes": ["hello", {}],
                "comment": {},
            }}))
            .await
            .unwrap_err();
        assert_eq!(
            result,
            (
                CompLine {
                    text: vec![DocRichText {
                        tag: None,
                        text: "foo".to_string()
                    }],
                    notes: vec![
                        DocNote::Text {
                            content: vec![DocRichText {
                                tag: None,
                                text: "hello".to_string()
                            }],
                        },
                        DocNote::Text {
                            content: vec![DocRichText {
                                tag: None,
                                text: "[object object]".to_string()
                            }],
                        }
                    ],
                    secondary_text: vec![DocRichText {
                        tag: None,
                        text: "[object object]".to_string()
                    }],
                    ..Default::default()
                },
                vec![
                    CompilerError::InvalidLinePropertyType("comment".to_string()),
                    CompilerError::InvalidLinePropertyType("notes[1]".to_string()),
                ]
            )
        );

        let result = compiler
            .comp_line(json!({
                "foo": {
                "split-name": "test .v(boo)",
            }}))
            .await
            .unwrap();
        assert_eq!(
            result,
            CompLine {
                text: vec![DocRichText {
                    tag: None,
                    text: "foo".to_string()
                }],
                split_name: Some(vec![
                    DocRichText {
                        tag: None,
                        text: "test ".to_string()
                    },
                    DocRichText {
                        tag: Some("v".to_string()),
                        text: "boo".to_string()
                    }
                ]),
                ..Default::default()
            }
        );

        let result = compiler
            .comp_line(json!({
                "foo": {
                "split-name": ["hello world"],
            }}))
            .await
            .unwrap_err();
        assert_eq!(
            result,
            (
                CompLine {
                    text: vec![DocRichText {
                        tag: None,
                        text: "foo".to_string()
                    }],
                    ..Default::default()
                },
                vec![CompilerError::InvalidLinePropertyType(
                    "split-name".to_string()
                )]
            )
        );

        let result = compiler
            .comp_line(json!({
                "foo": {
                "split-name": "",
            }}))
            .await
            .unwrap();
        assert_eq!(
            result,
            CompLine {
                text: vec![DocRichText {
                    tag: None,
                    text: "foo".to_string()
                }],
                split_name: Some(vec![]),
                ..Default::default()
            }
        );
    }

    #[tokio::test]
    async fn test_preset_one_level() {
        let test_preset = Preset::compile(json!({
            "text": "hello world",
            "comment": "foo bar",
        }))
        .await
        .unwrap();
        let test_preset_no_text = Preset::compile(json!({
            "comment": "foo bar",
        }))
        .await
        .unwrap();
        let mut builder = CompilerBuilder::default();
        builder
            .add_preset("_preset", test_preset)
            .add_preset("_notext", test_preset_no_text);
        let mut compiler = builder.build();
        let result = compiler.comp_line(json!("_preset")).await.unwrap();
        assert_eq!(
            result,
            CompLine {
                text: vec![DocRichText {
                    tag: None,
                    text: "hello world".to_string()
                }],
                secondary_text: vec![DocRichText {
                    tag: None,
                    text: "foo bar".to_string()
                }],
                ..Default::default()
            }
        );

        let result = compiler
            .comp_line(json!({
                "_preset": {
                    "comment": "foo bar 2",
                }
            }))
            .await
            .unwrap();
        assert_eq!(
            result,
            CompLine {
                text: vec![DocRichText {
                    tag: None,
                    text: "hello world".to_string()
                }],
                secondary_text: vec![DocRichText {
                    tag: None,
                    text: "foo bar 2".to_string()
                }],
                ..Default::default()
            }
        );

        let result = compiler
            .comp_line(json!({
                "_notext": {
                    "comment": "foo bar 2",
                }
            }))
            .await
            .unwrap();
        assert_eq!(
            result,
            CompLine {
                text: vec![DocRichText {
                    tag: None,
                    text: "_notext".to_string()
                }],
                secondary_text: vec![DocRichText {
                    tag: None,
                    text: "foo bar 2".to_string()
                }],
                ..Default::default()
            }
        );

        let result = compiler
            .comp_line(json!({
                "_notext": {
                    "text": "foo bar 2",
                }
            }))
            .await
            .unwrap();
        assert_eq!(
            result,
            CompLine {
                text: vec![DocRichText {
                    tag: None,
                    text: "foo bar 2".to_string()
                }],
                secondary_text: vec![DocRichText {
                    tag: None,
                    text: "foo bar".to_string()
                }],
                ..Default::default()
            }
        );

        let result = compiler
            .comp_line(json!({
                "_invalid": {
                    "comment": "foo bar 2",
                }
            }))
            .await
            .unwrap();
        assert_eq!(
            result,
            CompLine {
                text: vec![DocRichText {
                    tag: None,
                    text: "_invalid".to_string()
                }],
                secondary_text: vec![DocRichText {
                    tag: None,
                    text: "foo bar 2".to_string()
                }],
                ..Default::default()
            }
        );

        let result = compiler
            .comp_line(json!({
                "test": {
                    "text": "_preset",
                }
            }))
            .await
            .unwrap();
        assert_eq!(
            result,
            CompLine {
                text: vec![DocRichText {
                    tag: None,
                    text: "_preset".to_string()
                }],
                ..Default::default()
            }
        );
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
        let result = compiler
            .comp_line(json!({
                "_preset::one": {
                    "presets": ["_preset::two"],
                }
            }))
            .await
            .unwrap();
        assert_eq!(
            result,
            CompLine {
                text: vec![DocRichText {
                    tag: None,
                    text: "preset two text".to_string()
                }],
                secondary_text: vec![DocRichText {
                    tag: None,
                    text: "preset one".to_string()
                }],
                ..Default::default()
            }
        );

        let result = compiler
            .comp_line(json!({
                "test": {
                    "presets": "foo",
                }
            }))
            .await
            .unwrap_err();
        assert_eq!(
            result,
            (
                CompLine {
                    text: vec![DocRichText {
                        tag: None,
                        text: "test".to_string()
                    }],
                    ..Default::default()
                },
                vec![CompilerError::InvalidLinePropertyType(
                    "presets".to_string()
                ),]
            )
        );

        let result = compiler
            .comp_line(json!({
                "test": {
                    "presets": [{}, "foo", "_foo", "_hello::", 123],
                }
            }))
            .await
            .unwrap_err();
        assert_eq!(
            result,
            (
                CompLine {
                    text: vec![DocRichText {
                        tag: None,
                        text: "test".to_string()
                    }],
                    ..Default::default()
                },
                vec![
                    CompilerError::InvalidLinePropertyType("presets[0]".to_string()),
                    CompilerError::InvalidPresetString("foo".to_string()),
                    CompilerError::PresetNotFound("_foo".to_string()),
                    CompilerError::InvalidPresetString("_hello::".to_string()),
                    CompilerError::InvalidPresetString("123".to_string()),
                ]
            )
        );

        let result = compiler
            .comp_line(json!({
                "_preset::three<1>": {
                    "presets": ["_preset::one"],
                }
            }))
            .await
            .unwrap();
        assert_eq!(
            result,
            CompLine {
                text: vec![DocRichText {
                    tag: None,
                    text: "preset three".to_string()
                }],
                secondary_text: vec![DocRichText {
                    tag: None,
                    text: "preset two".to_string()
                }],
                ..Default::default()
            }
        );

        let result = compiler
            .comp_line(json!("_preset::four< abcde >"))
            .await
            .unwrap();
        assert_eq!(
            result,
            CompLine {
                text: vec![DocRichText {
                    tag: None,
                    text: "preset four: arg is  abcde ".to_string()
                }],
                secondary_text: vec![DocRichText {
                    tag: None,
                    text: "preset two".to_string()
                }],
                ..Default::default()
            }
        );

        let result = compiler
            .comp_line(json!("_preset::overflow"))
            .await
            .unwrap_err();
        assert_eq!(
            result,
            (
                CompLine {
                    text: vec![DocRichText {
                        tag: None,
                        text: "_preset::overflow".to_string()
                    }],
                    ..Default::default()
                },
                vec![CompilerError::MaxPresetDepthExceeded(
                    "_preset::overflow".to_string()
                )]
            )
        );
    }

    #[tokio::test]
    async fn test_icon_overrides() {
        let mut compiler = Compiler::default();
        let result = compiler
            .comp_line(json!({
                "icon is string": {
                    "icon": "my-icon",
                },
            }))
            .await
            .unwrap();
        assert_eq!(
            result,
            CompLine {
                text: vec![DocRichText {
                    tag: None,
                    text: "icon is string".to_string(),
                }],
                doc_icon: Some("my-icon".to_string()),
                map_icon: Some("my-icon".to_string()),
                ..Default::default()
            }
        );

        let result = compiler
            .comp_line(json!({
                "icon is string": {
                    "icon": ["my-icon"],
                },
            }))
            .await
            .unwrap_err();
        assert_eq!(
            result,
            (
                CompLine {
                    text: vec![DocRichText {
                        tag: None,
                        text: "icon is string".to_string(),
                    }],
                    ..Default::default()
                },
                vec![CompilerError::InvalidLinePropertyType("icon".to_string()),]
            )
        );

        let result = compiler
            .comp_line(json!({
                "icon is array": {
                    "icon": ["my-icon"],
                },
            }))
            .await
            .unwrap_err();
        assert_eq!(
            result,
            (
                CompLine {
                    text: vec![DocRichText {
                        tag: None,
                        text: "icon is array".to_string(),
                    }],
                    ..Default::default()
                },
                vec![CompilerError::InvalidLinePropertyType("icon".to_string()),]
            )
        );

        let result = compiler
            .comp_line(json!({
                "icon is empty object": {
                    "icon": {},
                },
            }))
            .await
            .unwrap();
        assert_eq!(
            result,
            CompLine {
                text: vec![DocRichText {
                    tag: None,
                    text: "icon is empty object".to_string(),
                }],
                ..Default::default()
            }
        );

        let result = compiler
            .comp_line(json!({
                "icon is object": {
                    "icon": {
                        "doc": "my-doc-icon",
                        "map": "my-map-icon",
                        "priority": 1,
                    },
                },
            }))
            .await
            .unwrap();
        assert_eq!(
            result,
            CompLine {
                text: vec![DocRichText {
                    tag: None,
                    text: "icon is object".to_string(),
                }],
                doc_icon: Some("my-doc-icon".to_string()),
                map_icon: Some("my-map-icon".to_string()),
                map_icon_priority: 1,
                ..Default::default()
            }
        );

        let result = compiler
            .comp_line(json!({
                "icon is object": {
                    "icon": {
                        "doc":{},
                        "map": ["my-map-icon"],
                        "priority": 1.2,
                        "boo": "foo",
                    },
                },
            }))
            .await
            .unwrap_err();
        assert_eq!(
            result,
            (
                CompLine {
                    text: vec![DocRichText {
                        tag: None,
                        text: "icon is object".to_string(),
                    }],
                    ..Default::default()
                },
                vec![
                    CompilerError::UnusedProperty("icon.boo".to_string()),
                    CompilerError::InvalidLinePropertyType("icon.doc".to_string()),
                    CompilerError::InvalidLinePropertyType("icon.map".to_string()),
                    CompilerError::InvalidLinePropertyType("icon.priority".to_string()),
                ]
            )
        );
    }

    #[tokio::test]
    async fn test_counter_override() {
        let mut compiler = Compiler::default();
        let result = compiler
            .comp_line(json!({
                "counter is string": {
                    "counter": "hello",
                },
            }))
            .await
            .unwrap();
        assert_eq!(
            result,
            CompLine {
                text: vec![DocRichText {
                    tag: None,
                    text: "counter is string".to_string(),
                }],
                counter_text: Some(DocRichText {
                    tag: None,
                    text: "hello".to_string(),
                }),
                ..Default::default()
            }
        );

        let result = compiler
            .comp_line(json!({
                "counter is tagged string": {
                    "counter": ".test(hello)",
                },
            }))
            .await
            .unwrap();
        assert_eq!(
            result,
            CompLine {
                text: vec![DocRichText {
                    tag: None,
                    text: "counter is tagged string".to_string(),
                }],
                counter_text: Some(DocRichText {
                    tag: Some("test".to_string()),
                    text: "hello".to_string(),
                }),
                ..Default::default()
            }
        );

        let result = compiler
            .comp_line(json!({
                "counter is empty tagged string": {
                    "counter": ".test()",
                },
            }))
            .await
            .unwrap();
        assert_eq!(
            result,
            CompLine {
                text: vec![DocRichText {
                    tag: None,
                    text: "counter is empty tagged string".to_string(),
                }],
                counter_text: Some(DocRichText {
                    tag: Some("test".to_string()),
                    text: "".to_string(),
                }),
                ..Default::default()
            }
        );

        let result = compiler
            .comp_line(json!({
                "counter is empty string": {
                    "counter": "",
                },
            }))
            .await
            .unwrap();
        assert_eq!(
            result,
            CompLine {
                text: vec![DocRichText {
                    tag: None,
                    text: "counter is empty string".to_string(),
                }],
                counter_text: None,
                ..Default::default()
            }
        );

        let result = compiler
            .comp_line(json!({
                "counter is invalid": {
                    "counter": ["hello"],
                },
            }))
            .await
            .unwrap_err();
        assert_eq!(
            result,
            (
                CompLine {
                    text: vec![DocRichText {
                        tag: None,
                        text: "counter is invalid".to_string(),
                    }],
                    ..Default::default()
                },
                vec![CompilerError::InvalidLinePropertyType(
                    "counter".to_string()
                ),]
            )
        );

        let result = compiler
            .comp_line(json!({
                "counter is more than one text block": {
                    "counter": ".v(hello) ",
                },
            }))
            .await
            .unwrap_err();
        assert_eq!(
            result,
            (
                CompLine {
                    text: vec![DocRichText {
                        tag: None,
                        text: "counter is more than one text block".to_string(),
                    }],
                    counter_text: Some(DocRichText {
                        tag: Some("v".to_string()),
                        text: "hello".to_string(),
                    }),
                    ..Default::default()
                },
                vec![CompilerError::TooManyTagsInCounter,]
            )
        );
    }

    #[tokio::test]
    async fn test_inherit_color_coord() {
        let builder = CompilerBuilder::new("color".to_string(), GameCoord(1.0, 2.0, 3.0));
        let mut compiler = builder.build();

        let result = compiler
            .comp_line(json!("no color or coord"))
            .await
            .unwrap();
        assert_eq!(
            result,
            CompLine {
                text: vec![DocRichText {
                    tag: None,
                    text: "no color or coord".to_string(),
                }],
                line_color: "color".to_string(),
                map_coord: GameCoord(1.0, 2.0, 3.0),
                ..Default::default()
            }
        );
    }

    #[tokio::test]
    async fn test_change_color() {
        let builder = CompilerBuilder::new("color".to_string(), GameCoord(1.0, 2.0, 3.0));
        let mut compiler = builder.build();

        let result = compiler
            .comp_line(json!({
                "change color": {
                    "color": "new-color",
                }
            }))
            .await
            .unwrap();
        assert_eq!(
            result,
            CompLine {
                text: vec![DocRichText {
                    tag: None,
                    text: "change color".to_string(),
                }],
                line_color: "new-color".to_string(),
                map_coord: GameCoord(1.0, 2.0, 3.0),
                ..Default::default()
            }
        );

        let result = compiler
            .comp_line(json!({
                "change color 2": {
                    "color": ["newer-color"],
                }
            }))
            .await
            .unwrap_err();
        assert_eq!(
            result,
            (
                CompLine {
                    text: vec![DocRichText {
                        tag: None,
                        text: "change color 2".to_string(),
                    }],
                    line_color: "new-color".to_string(),
                    map_coord: GameCoord(1.0, 2.0, 3.0),
                    ..Default::default()
                },
                vec![CompilerError::InvalidLinePropertyType("color".to_string())]
            )
        );
    }

    #[tokio::test]
    async fn test_change_coord() {
        let builder = CompilerBuilder::new("".to_string(), GameCoord(1.0, 2.0, 3.0));
        let mut compiler = builder.build();

        let result = compiler.comp_line(json!({
            "change coord": {
                "coord": [4.0, 5.0, 6.0],
            }
        })).await.unwrap();
        assert_eq!(result, CompLine {
            text: vec![DocRichText {
                tag: None,
                text: "change coord".to_string(),
            }],
            map_coord: GameCoord(4.0, 5.0, 6.0),
            movements: vec![
                CompMovement {
                    to: GameCoord(4.0, 5.0, 6.0),
                    warp: false
                }
            ],
            ..Default::default()
        });
    }
}
