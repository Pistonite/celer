use std::collections::BTreeMap;

use serde_json::Value;

use crate::json::{IntoSafeRouteBlob, SafeRouteBlob};

use super::{Preset, PresetBlob};

// pub struct PresetHydrate<'a, 'b> {
//     preset: &'a PresetBlob,
//     args: &'b [String],
// }
//
// impl<'a, 'b> IntoSafeRouteBlob for PresetHydrate<'a, 'b> {
//     fn into_unchecked(self) -> SafeRouteBlob<'static>
//     {
//         match self.preset {
//             PresetBlob::NonTemplate(value) => value.ref_into_unchecked(),
//             PresetBlob::Template(tempstr) => {
//                 let str = tempstr.hydrate(self.args);
//                 Value::String(str).into_unchecked()
//             }
//             PresetBlob::Array(arr) => {
//                 let mut out = vec![];
//                 for x in arr {
//                     out.push(x.hydrate(self.args).into_unchecked());
//                 }
//                 SafeRouteBlob::OwnedArray(out)
//             }
//             PresetBlob::Object(props) => {
//                 let mut out = BTreeMap::new();
//                 for (key_template, val) in props {
//                     let key = key_template.hydrate(self.args);
//                     let val = val.hydrate(self.args);
//                     out.insert(key, val.into_unchecked());
//                 }
//                 SafeRouteBlob::OwnedObject(out)
//             }
//         }
//     }
//     fn ref_into_unchecked(&self) -> SafeRouteBlob<'_> {
//         Self {
//             preset: self.preset,
//             args: self.args,
//         }
//         .into_unchecked()
//     }
// }

impl Preset {
    /// Hydrate a preset with the given arguments
    ///
    /// Return a new json blob with all template strings hydrated with the arguments
    pub fn hydrate<'c>(&'c self, args: &[String], map: &mut BTreeMap<String, SafeRouteBlob<'c>>)
    {
        for (key, value) in self.0.iter() {
            map.insert(key.hydrate(args), value.hydrate(args));
        }
    }
}

impl PresetBlob {
    /// Hydrate a preset blob with the given arguments
    pub fn hydrate(&self, args: &[String]) -> SafeRouteBlob<'_> {
        match self {
            PresetBlob::NonTemplate(value) => value.ref_into_unchecked(),
            PresetBlob::Template(tempstr) => {
                let str = tempstr.hydrate(args);
                Value::String(str).into_unchecked()
            }
            PresetBlob::Array(arr) => {
                let mut out = vec![];
                for x in arr {
                    out.push(x.hydrate(args).into_unchecked());
                }
                SafeRouteBlob::OwnedArray(out)
            }
            PresetBlob::Object(props) => {
                let mut out = BTreeMap::new();
                for (key_template, val) in props {
                    let key = key_template.hydrate(args);
                    let val = val.hydrate(args);
                    out.insert(key, val.into_unchecked());
                }
                SafeRouteBlob::OwnedObject(out)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use map_macro::btree_map;
    use serde_json::json;

    use super::*;

    const ARGS: &[&str] = &["hello", "world", "temp"];

    impl Preset {
        fn test_hydrate(&self, args: &[&str]) -> BTreeMap<String, Value> {
            let args = args.iter().map(|x| x.to_string()).collect::<Vec<_>>();
            let mut map = BTreeMap::new();
            self.hydrate(&args, &mut map);
            map.into_iter().map(|(k, v)| (k, v.into_unchecked().into())).collect()
        }
    }

    #[test]
    fn test_emptyobj() {
        let preset = Preset::compile(json!({})).unwrap();

        assert_eq!(preset.test_hydrate(ARGS), btree_map! {});
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
            preset.test_hydrate(ARGS),
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
            preset.test_hydrate(ARGS),
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
