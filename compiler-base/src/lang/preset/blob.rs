use serde_json::Value;

use crate::lang::TempStr;

use super::{Preset, PresetBlob};

impl Preset {
    /// Compile a preset
    ///
    /// Returns None if the json blob is not an object
    pub fn compile(value: Value) -> Option<Self> {
        let obj = match value {
            Value::Object(obj) => obj,
            _ => return None,
        };
        let mut properties = Vec::with_capacity(obj.len());
        for (key, value) in obj.into_iter() {
            let sub = PresetBlob::compile(value);
            properties.push((TempStr::from(key), sub));
        }
        Some(Self(properties))
    }
}

impl PresetBlob {
    /// Compile template strings in a json blob
    pub fn compile(mut value: Value) -> Self {
        match Self::compile_internal(&mut value) {
            Some(blob) => blob,
            None => Self::NonTemplate(value),
        }
    }
    /// Recursive compile helper
    ///
    /// If the blob has any template strings in it, returns a Some variant with
    /// the template strings compiled and the input value `.take()`-en out.
    /// Otherwise returns a None variant.
    fn compile_internal(value: &mut Value) -> Option<Self> {
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
                    let sub = Self::compile_internal(value);
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
                    let key = TempStr::from(key);
                    let sub = Self::compile_internal(value);
                    if sub.is_some() || !key.is_literal() {
                        has_template = true;
                    }
                    result.push((key, sub, value));
                }
                if has_template {
                    let props = result
                        .into_iter()
                        .map(|(key, sub, value)| {
                            if let Some(sub) = sub {
                                (key, sub)
                            } else {
                                (key, Self::NonTemplate(value.take()))
                            }
                        })
                        .collect();
                    Some(Self::Object(props))
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

    #[test]
    fn test_error() {
        assert_eq!(Preset::compile(json!([])), None);
        assert_eq!(Preset::compile(json!("something")), None);
        assert_eq!(Preset::compile(json!(true)), None);
        assert_eq!(Preset::compile(json!(null)), None);
        assert_eq!(Preset::compile(json!(0)), None);
    }

    #[test]
    fn test_empty() {
        assert_eq!(
            Preset::compile(json!({})),
            Some(Preset([].into_iter().collect()))
        );
    }

    #[test]
    fn test_one_level() {
        let value = json!({
            "a": "foo",
            "b": "foo$(1)",
        });
        let expected = Some(Preset(vec![
            (TempStr::from("a"), PresetBlob::NonTemplate(json!("foo"))),
            (
                TempStr::from("b"),
                PresetBlob::Template(TempStr::from("foo$(1)")),
            ),
        ]));
        assert_eq!(Preset::compile(value), expected);
    }

    #[test]
    fn test_many_levels() {
        let value = json!({
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
            "$(7)": ["foo", "foo", {
                "a": "foo",
                "b": "foo",
                "c": ["bar"]
            }],
            "h": {
                "b": "foo",
                "$(12)": ["bar"]
            },
        });
        let expected = Some(Preset(vec![
            (
                TempStr::from("$(7)"),
                PresetBlob::NonTemplate(json!(["foo", "foo", {
                    "a": "foo",
                    "b": "foo",
                    "c": ["bar"]
                }])),
            ),
            (TempStr::from("a"), PresetBlob::NonTemplate(json!("foo"))),
            (
                TempStr::from("b"),
                PresetBlob::Template(TempStr::from("foo$(1)")),
            ),
            (
                TempStr::from("c"),
                PresetBlob::Object(vec![
                    (TempStr::from("d"), PresetBlob::NonTemplate(json!("foo"))),
                    (
                        TempStr::from("e"),
                        PresetBlob::Template(TempStr::from("foo$(1)")),
                    ),
                ]),
            ),
            (
                TempStr::from("d"),
                PresetBlob::NonTemplate(json!({
                    "a": "foo",
                    "b": "foo",
                })),
            ),
            (
                TempStr::from("e"),
                PresetBlob::NonTemplate(json!(["foo", "foo"])),
            ),
            (
                TempStr::from("f"),
                PresetBlob::Array(vec![
                    PresetBlob::NonTemplate(json!("foo")),
                    PresetBlob::NonTemplate(json!("foo")),
                    PresetBlob::Object(vec![
                        (TempStr::from("a"), PresetBlob::NonTemplate(json!("foo"))),
                        (
                            TempStr::from("b"),
                            PresetBlob::Template(TempStr::from("foo$(1)")),
                        ),
                        (TempStr::from("c"), PresetBlob::NonTemplate(json!(["bar"]))),
                    ]),
                ]),
            ),
            (
                TempStr::from("h"),
                PresetBlob::Object(vec![
                    (
                        TempStr::from("$(12)"),
                        PresetBlob::NonTemplate(json!(["bar"])),
                    ),
                    (TempStr::from("b"), PresetBlob::NonTemplate(json!("foo"))),
                ]),
            ),
        ]));
        assert_eq!(Preset::compile(value), expected);
    }
}
