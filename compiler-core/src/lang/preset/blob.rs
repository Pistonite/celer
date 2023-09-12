use std::collections::BTreeMap;

use serde_json::Value;

use crate::lang::TempStr;

use super::{Preset, PresetBlob};

impl Preset {
    /// Compile a preset
    ///
    /// Returns None if the json blob is not an object
    pub async fn compile(value: Value) -> Option<Self> {
        let obj = match value {
            Value::Object(obj) => obj,
            _ => return None,
        };
        let mut properties = BTreeMap::new();
        for (key, value) in obj.into_iter() {
            let sub = PresetBlob::compile(value).await;
            properties.insert(key, sub);
        }
        Some(Self(properties))
    }
}

impl PresetBlob {
    /// Compile template strings in a json blob
    pub async fn compile(mut value: Value) -> Self {
        match Self::compile_internal(&mut value).await {
            Some(blob) => blob,
            None => Self::NonTemplate(value),
        }
    }
    /// Recursive compile helper
    ///
    /// If the blob has any template strings in it, returns a Some variant with
    /// the template strings compiled and the input value taken out.
    /// Otherwise returns a None variant.
    #[cfg_attr(not(feature = "wasm"), async_recursion::async_recursion)]
    #[cfg_attr(feature = "wasm", async_recursion::async_recursion(?Send))]
    async fn compile_internal(value: &mut Value) -> Option<Self> {
        match value {
            Value::String(s) => {
                let tempstr = TempStr::from(s);
                if tempstr.is_literal() {
                    None
                } else {
                    Some(Self::Template(tempstr))
                }
            }
            Value::Array(arr) => {
                let mut result = vec![];
                let mut has_template = false;
                for value in arr.iter_mut() {
                    let sub = Self::compile_internal(value).await;
                    if sub.is_some() {
                        has_template = true;
                    }
                    result.push((sub, value));
                }
                if has_template {
                    Some(Self::Array(
                        result
                            .into_iter()
                            .map(|(sub, value)| {
                                if let Some(sub) = sub {
                                    sub
                                } else {
                                    Self::NonTemplate(value.take())
                                }
                            })
                            .collect(),
                    ))
                } else {
                    None
                }
            }
            Value::Object(obj) => {
                let mut result = vec![];
                let mut has_template = false;
                for (key, value) in obj.iter_mut() {
                    let sub = Self::compile_internal(value).await;
                    if sub.is_some() {
                        has_template = true;
                    }
                    result.push((key, sub, value));
                }
                if has_template {
                    Some(Self::Object(
                        result
                            .into_iter()
                            .map(|(key, sub, value)| {
                                if let Some(sub) = sub {
                                    (key.clone(), sub)
                                } else {
                                    (key.clone(), Self::NonTemplate(value.take()))
                                }
                            })
                            .collect(),
                    ))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::lang::TempStr;

    use super::*;

    #[tokio::test]
    async fn test_error() {
        assert_eq!(Preset::compile(json!([])).await, None);
        assert_eq!(Preset::compile(json!("something")).await, None);
        assert_eq!(Preset::compile(json!(true)).await, None);
        assert_eq!(Preset::compile(json!(null)).await, None);
        assert_eq!(Preset::compile(json!(0)).await, None);
    }

    #[tokio::test]
    async fn test_empty() {
        assert_eq!(
            Preset::compile(json!({})).await,
            Some(Preset([].into_iter().collect()))
        );
    }

    #[tokio::test]
    async fn test_one_level() {
        assert_eq!(
            Preset::compile(json!({
                "a": "foo",
                "b": "foo$(1)",
            }))
            .await,
            Some(Preset(
                [
                    ("a".to_string(), PresetBlob::NonTemplate(json!("foo"))),
                    (
                        "b".to_string(),
                        PresetBlob::Template(TempStr::from("foo$(1)"))
                    ),
                ]
                .into_iter()
                .collect()
            ))
        );
    }

    #[tokio::test]
    async fn test_many_level() {
        assert_eq!(
            Preset::compile(json!({
                "a": "foo",
                "b": "foo$(1)",
                "c": {
                    "d": "foo",
                    "e": "foo$(1)",
                },
                "d": {
                    "a": "foo",
                    "b": "foo",
                },
                "e": ["foo", "foo"],
                "f": ["foo", "foo", {
                    "a": "foo",
                    "b": "foo$(1)",
                    "c": ["bar"]
                }],
                "g": ["foo", "foo", {
                    "a": "foo",
                    "b": "foo",
                    "c": ["bar"]
                }],
            }))
            .await,
            Some(Preset(
                [
                    ("a".to_string(), PresetBlob::NonTemplate(json!("foo"))),
                    (
                        "b".to_string(),
                        PresetBlob::Template(TempStr::from("foo$(1)"))
                    ),
                    (
                        "c".to_string(),
                        PresetBlob::Object(
                            [
                                ("d".to_string(), PresetBlob::NonTemplate(json!("foo"))),
                                (
                                    "e".to_string(),
                                    PresetBlob::Template(TempStr::from("foo$(1)"))
                                )
                            ]
                            .into_iter()
                            .collect()
                        )
                    ),
                    (
                        "d".to_string(),
                        PresetBlob::NonTemplate(json!({
                            "a": "foo",
                            "b": "foo",
                        }))
                    ),
                    (
                        "e".to_string(),
                        PresetBlob::NonTemplate(json!(["foo", "foo"]))
                    ),
                    (
                        "f".to_string(),
                        PresetBlob::Array(vec![
                            PresetBlob::NonTemplate(json!("foo")),
                            PresetBlob::NonTemplate(json!("foo")),
                            PresetBlob::Object(
                                [
                                    ("a".to_string(), PresetBlob::NonTemplate(json!("foo"))),
                                    (
                                        "b".to_string(),
                                        PresetBlob::Template(TempStr::from("foo$(1)"))
                                    ),
                                    ("c".to_string(), PresetBlob::NonTemplate(json!(["bar"])))
                                ]
                                .into_iter()
                                .collect()
                            ),
                        ])
                    ),
                    (
                        "g".to_string(),
                        PresetBlob::NonTemplate(json!(["foo", "foo", {
                            "a": "foo",
                            "b": "foo",
                            "c": ["bar"]
                        }]))
                    ),
                ]
                .into_iter()
                .collect()
            ))
        );
    }
}
