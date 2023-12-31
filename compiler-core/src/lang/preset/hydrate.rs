use std::collections::BTreeMap;

use serde_json::{json, Value};

use crate::macros::test_suite;

use super::{Preset, PresetBlob};

impl Preset {
    /// Hydrate a preset with the given arguments
    ///
    /// Return a new json blob with all template strings hydrated with the arguments
    pub fn hydrate<S>(&self, args: &[S]) -> BTreeMap<String, Value>
    where
        S: AsRef<str> + Sync,
    {
        let mut out = BTreeMap::new();
        for (key, value) in self.0.iter() {
            out.insert(key.hydrate(args), value.hydrate(args));
        }
        out
    }
}

impl PresetBlob {
    /// Hydrate a preset blob with the given arguments
    pub fn hydrate<S>(&self, args: &[S]) -> Value
    where
        S: AsRef<str> + Sync,
    {
        match self {
            Self::NonTemplate(value) => value.clone(),
            Self::Template(tempstr) => {
                let str = tempstr.hydrate(args);
                Value::String(str)
            }
            Self::Array(arr) => {
                let mut out = vec![];
                for x in arr {
                    out.push(x.hydrate(args));
                }
                Value::Array(out)
            }
            Self::Object(props) => {
                let mut out = json!({});
                for (key_template, val) in props {
                    let key = key_template.hydrate(args);
                    out[key] = val.hydrate(args);
                }
                out
            }
        }
    }
}

#[test_suite]
mod test {
    use map_macro::btree_map;

    use super::*;

    const ARGS: &[&str] = &["hello", "world", "temp"];

    #[test]
    fn test_emptyobj() {
        let preset = Preset::compile(json!({})).unwrap();

        assert_eq!(preset.hydrate(ARGS), btree_map! {});
    }

    #[test]
    fn test_one_level() {
        let preset = Preset::compile(json!({
            "a": "foo$(1)",
            "b": "world",
            "c": "te$(0)mp"
        }))
        .unwrap();

        assert_eq!(
            preset.hydrate(ARGS),
            btree_map! {
                "a".to_string() => json!("fooworld"),
                "b".to_string() => json!("world"),
                "c".to_string() => json!("tehellomp"),
            }
        );
    }

    #[test]
    fn test_nested() {
        let preset = Preset::compile(json!({
            "a": "foo$(1)",
            "b": ["world$(2)", {
                "c": "te$(0)mp"
            }],
            "c": "temp$(0)$(1)",
            "$(1)a$(2)": "$(0)"
        }))
        .unwrap();

        assert_eq!(
            preset.hydrate(ARGS),
            btree_map! {
                "a".to_string() => json!("fooworld"),
                "b".to_string() => json!(["worldtemp", {
                    "c": "tehellomp"
                }]),
                "c".to_string() => json!("temphelloworld"),
                "worldatemp".to_string() => json!("hello"),
            }
        );
    }
}
