use std::collections::BTreeMap;

use serde_json::Value;

use crate::json::{Cast, Coerce};
use crate::lang::PresetInst;
use crate::macros::{async_recursion, maybe_send};
use crate::prop;

use super::{validate_not_array_or_object, Compiler, CompilerError};

impl Compiler {
    /// Apply the preset to the output.
    ///
    /// Presets are applied recursively, including presets in the movements
    #[maybe_send(async_recursion)]
    pub async fn apply_preset(
        &self,
        depth: usize,
        inst: &PresetInst,
        output: &mut BTreeMap<String, Value>,
        errors: &mut Vec<CompilerError>,
    ) {
        if depth > self.max_preset_depth {
            errors.push(CompilerError::MaxPresetDepthExceeded(inst.name.to_string()));
            return;
        }
        let preset = match self.meta.presets.get(&inst.name) {
            None => {
                errors.push(CompilerError::PresetNotFound(inst.name.to_string()));
                return;
            }
            Some(preset) => preset,
        };
        let mut properties = preset.hydrate(&inst.args).await;
        if let Some(presets) = properties.remove(prop::PRESETS) {
            self.process_presets(depth, presets, output, errors).await;
        }

        super::desugar_properties(&mut properties).await;

        if let Some(movements) = properties.remove(prop::MOVEMENTS) {
            properties.insert(
                prop::MOVEMENTS.to_string(),
                self.expand_presets_in_movements(depth, movements, errors)
                    .await,
            );
        }

        output.extend(properties);
    }

    /// Process the "presets" property in the line object
    ///
    /// Saves the properties from the preset to the output map
    pub async fn process_presets(
        &self,
        depth: usize,
        presets: Value,
        output: &mut BTreeMap<String, Value>,
        errors: &mut Vec<CompilerError>,
    ) {
        let preset_arr = match presets {
            Value::Array(arr) => arr,
            _ => vec![presets],
        };
        for (i, preset_value) in preset_arr.into_iter().enumerate() {
            if !validate_not_array_or_object!(
                &preset_value,
                errors,
                format!("{p}[{i}]", p = prop::PRESETS)
            ) {
                continue;
            }

            let preset_string = preset_value.coerce_to_string();
            if !preset_string.starts_with('_') {
                errors.push(CompilerError::InvalidPresetString(preset_string));
                continue;
            }

            let preset_inst = PresetInst::try_parse(&preset_string);
            match preset_inst {
                None => {
                    errors.push(CompilerError::InvalidPresetString(preset_string));
                }
                Some(inst) => {
                    self.apply_preset(depth + 1, &inst, output, errors).await;
                }
            }
        }
    }

    /// Expand presets in the movements array
    pub async fn expand_presets_in_movements(
        &self,
        depth: usize,
        movements: Value,
        errors: &mut Vec<CompilerError>,
    ) -> Value {
        let array = match movements {
            Value::Array(array) => array,
            _ => return movements,
        };

        let mut new_array = vec![];
        for v in array.into_iter() {
            let preset_str = match v.as_str().filter(|s| s.starts_with('_')) {
                Some(x) => x,
                None => {
                    new_array.push(v);
                    continue;
                }
            };
            let preset_inst = match PresetInst::try_parse(preset_str) {
                None => {
                    errors.push(CompilerError::InvalidPresetString(preset_str.to_string()));
                    continue;
                }
                Some(inst) => inst,
            };
            let mut map = BTreeMap::new();
            self.apply_preset(depth + 1, &preset_inst, &mut map, errors)
                .await;

            match map
                .remove(prop::MOVEMENTS)
                .and_then(|m| m.try_into_array().ok())
            {
                Some(movements) => {
                    new_array.extend(movements);
                }
                _ => {
                    errors.push(CompilerError::InvalidMovementPreset(preset_str.to_string()));
                }
            }
        }
        Value::Array(new_array)
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::comp::CompilerBuilder;
    use crate::lang::Preset;

    use super::*;

    #[tokio::test]
    async fn test_one_level() {
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
                "_preset2",
                Preset::compile(json!({
                    "text": "_preset",
                    "comment": "foo bar",
                }))
                .await
                .unwrap(),
            );
        let compiler = builder.build();
        let mut output = BTreeMap::new();
        let mut errors = Vec::new();

        compiler
            .apply_preset(
                0,
                &PresetInst {
                    name: "_preset".to_string(),
                    args: vec![],
                },
                &mut output,
                &mut errors,
            )
            .await;

        assert_eq!(
            output,
            [
                ("text".to_string(), json!("hello world")),
                ("comment".to_string(), json!("foo bar")),
            ]
            .into_iter()
            .collect()
        );
        assert_eq!(errors, vec![]);

        output.clear();

        compiler
            .apply_preset(
                0,
                &PresetInst {
                    name: "_preset2".to_string(),
                    args: vec![],
                },
                &mut output,
                &mut errors,
            )
            .await;

        assert_eq!(
            output,
            [
                ("text".to_string(), json!("_preset")),
                ("comment".to_string(), json!("foo bar")),
            ]
            .into_iter()
            .collect()
        );
    }

    #[tokio::test]
    async fn test_one_level_invalid() {
        let mut builder = CompilerBuilder::default();
        builder.add_preset(
            "_preset",
            Preset::compile(json!({
                "text": "hello world",
                "comment": "foo bar",
            }))
            .await
            .unwrap(),
        );
        let compiler = builder.build();
        let mut output = BTreeMap::new();
        let mut errors = Vec::new();

        compiler
            .apply_preset(
                0,
                &PresetInst {
                    name: "_preset2".to_string(),
                    args: vec![],
                },
                &mut output,
                &mut errors,
            )
            .await;

        assert_eq!(output, BTreeMap::new());
        assert_eq!(
            errors,
            vec![CompilerError::PresetNotFound("_preset2".to_string())]
        );
    }

    #[tokio::test]
    async fn test_complex() {
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
                    "presets": "_preset::two"
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
            );
        let compiler = builder.build();

        let mut output = BTreeMap::new();
        let mut errors = Vec::new();

        compiler
            .apply_preset(
                0,
                &PresetInst {
                    name: "_preset::three".to_string(),
                    args: vec![],
                },
                &mut output,
                &mut errors,
            )
            .await;

        assert_eq!(
            output,
            [
                ("text".to_string(), json!("preset three")),
                ("comment".to_string(), json!("preset two")),
            ]
            .into_iter()
            .collect()
        );
        assert_eq!(errors, vec![]);

        output.clear();
        compiler
            .apply_preset(
                0,
                &PresetInst {
                    name: "_preset::three".to_string(),
                    args: vec!["1".to_string()],
                },
                &mut output,
                &mut errors,
            )
            .await;
        assert_eq!(
            output,
            [
                ("text".to_string(), json!("preset three")),
                ("comment".to_string(), json!("preset two")),
            ]
            .into_iter()
            .collect()
        );
        assert_eq!(errors, vec![]);

        output.clear();
        compiler
            .apply_preset(
                0,
                &PresetInst {
                    name: "_preset::four".to_string(),
                    args: vec![" abcde ".to_string()],
                },
                &mut output,
                &mut errors,
            )
            .await;
        assert_eq!(
            output,
            [
                ("text".to_string(), json!("preset four: arg is  abcde ")),
                ("comment".to_string(), json!("preset two")),
            ]
            .into_iter()
            .collect()
        );
        assert_eq!(errors, vec![]);
    }

    #[tokio::test]
    async fn test_complex_invalid() {
        let mut builder = CompilerBuilder::default();
        builder
            .add_preset(
                "_preset::one",
                Preset::compile(json!({
                    "presets": "preset one",
                }))
                .await
                .unwrap(),
            )
            .add_preset(
                "_preset::two",
                Preset::compile(json!({
                    "presets": [{}, "foo", "_foo", "_hello::", 123],
                }))
                .await
                .unwrap(),
            )
            .add_preset(
                "_preset::three",
                Preset::compile(json!({
                    "presets": ["_preset::three"]
                }))
                .await
                .unwrap(),
            );
        let compiler = builder.build();

        let mut output = BTreeMap::new();
        let mut errors = Vec::new();

        compiler
            .apply_preset(
                0,
                &PresetInst {
                    name: "_preset::one".to_string(),
                    args: vec![],
                },
                &mut output,
                &mut errors,
            )
            .await;
        assert_eq!(output, BTreeMap::new());
        assert_eq!(
            errors,
            vec![CompilerError::InvalidPresetString("preset one".to_string())]
        );

        errors.clear();

        compiler
            .apply_preset(
                0,
                &PresetInst {
                    name: "_preset::two".to_string(),
                    args: vec![],
                },
                &mut output,
                &mut errors,
            )
            .await;
        assert_eq!(output, BTreeMap::new());
        assert_eq!(
            errors,
            vec![
                CompilerError::InvalidLinePropertyType("presets[0]".to_string()),
                CompilerError::InvalidPresetString("foo".to_string()),
                CompilerError::PresetNotFound("_foo".to_string()),
                CompilerError::InvalidPresetString("_hello::".to_string()),
                CompilerError::InvalidPresetString("123".to_string()),
            ]
        );

        errors.clear();

        compiler
            .apply_preset(
                0,
                &PresetInst {
                    name: "_preset::three".to_string(),
                    args: vec![],
                },
                &mut output,
                &mut errors,
            )
            .await;
        assert_eq!(output, BTreeMap::new());
        assert_eq!(
            errors,
            vec![CompilerError::MaxPresetDepthExceeded(
                "_preset::three".to_string()
            ),]
        );
    }

    #[tokio::test]
    async fn test_movement() {
        let mut builder = CompilerBuilder::default();
        builder
            .add_preset(
                "_preset::one",
                Preset::compile(json!({
                    "movements": [
                        "push",
                    "_preset::two",
                    "pop",
                ]
                }))
                .await
                .unwrap(),
            )
            .add_preset(
                "_preset::two",
                Preset::compile(json!({
                    "coord": [1, 2, 3]
                }))
                .await
                .unwrap(),
            )
            .add_preset(
                "_invalid::one",
                Preset::compile(json!({
                    "movements": 1
                }))
                .await
                .unwrap(),
            );
        let compiler = builder.build();

        let mut output = BTreeMap::new();
        let mut errors = Vec::new();

        compiler
            .apply_preset(
                0,
                &PresetInst {
                    name: "_preset::one".to_string(),
                    args: vec![],
                },
                &mut output,
                &mut errors,
            )
            .await;

        assert_eq!(
            output,
            [("movements".to_string(), json!(["push", [1, 2, 3], "pop",])),]
                .into_iter()
                .collect()
        );
        assert_eq!(errors, vec![]);

        output.clear();

        compiler
            .apply_preset(
                0,
                &PresetInst {
                    name: "_preset::two".to_string(),
                    args: vec![],
                },
                &mut output,
                &mut errors,
            )
            .await;

        assert_eq!(
            output,
            [("movements".to_string(), json!([[1, 2, 3]])),]
                .into_iter()
                .collect()
        );
    }

    #[tokio::test]
    async fn test_movements_invalid() {
        let mut builder = CompilerBuilder::default();
        builder
            .add_preset(
                "_invalid::one",
                Preset::compile(json!({
                    "movements": 1
                }))
                .await
                .unwrap(),
            )
            .add_preset(
                "_invalid::nomovements",
                Preset::compile(json!({
                    "text": 1
                }))
                .await
                .unwrap(),
            )
            .add_preset(
                "_invalid::two",
                Preset::compile(json!({
                    "movements": [
                        "_invalid",
                        "_invalid::",
                        "_invalid::nomovements",
                        "_invalid::one",
                        "push",
                        [0,0,0]
                    ]
                }))
                .await
                .unwrap(),
            )
            .add_preset(
                "_invalid::overflow",
                Preset::compile(json!({
                    "movements": [
                        "push",
                        "_invalid::overflow"
                    ]
                }))
                .await
                .unwrap(),
            );
        let compiler = builder.build();

        let mut output = BTreeMap::new();
        let mut errors = Vec::new();

        compiler
            .apply_preset(
                0,
                &PresetInst {
                    name: "_invalid::one".to_string(),
                    args: vec![],
                },
                &mut output,
                &mut errors,
            )
            .await;

        assert_eq!(
            output,
            [("movements".to_string(), json!(1)),].into_iter().collect()
        );
        assert_eq!(errors, vec![]);

        output.clear();

        compiler
            .apply_preset(
                0,
                &PresetInst {
                    name: "_invalid::two".to_string(),
                    args: vec![],
                },
                &mut output,
                &mut errors,
            )
            .await;

        assert_eq!(
            output,
            [("movements".to_string(), json!(["push", [0, 0, 0]]))]
                .into_iter()
                .collect()
        );

        assert_eq!(
            errors,
            vec![
                CompilerError::PresetNotFound("_invalid".to_string()),
                CompilerError::InvalidMovementPreset("_invalid".to_string()),
                CompilerError::InvalidPresetString("_invalid::".to_string()),
                CompilerError::InvalidMovementPreset("_invalid::nomovements".to_string()),
                CompilerError::InvalidMovementPreset("_invalid::one".to_string()),
            ]
        );

        output.clear();
        errors.clear();

        compiler
            .apply_preset(
                0,
                &PresetInst {
                    name: "_invalid::overflow".to_string(),
                    args: vec![],
                },
                &mut output,
                &mut errors,
            )
            .await;

        // we don't care what the output is here

        assert_eq!(
            errors,
            vec![
                CompilerError::MaxPresetDepthExceeded("_invalid::overflow".to_string()),
                CompilerError::InvalidMovementPreset("_invalid::overflow".to_string()),
            ]
        );
    }
}
