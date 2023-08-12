use std::collections::BTreeMap;

use serde_json::{json, Value};

use super::{Preset, PresetBlob};

impl Preset {
    /// Hydrate a preset with the given arguments
    ///
    /// Return a new json blob with all template strings hydrated with the arguments
    pub async fn hydrate<S>(&self, args: &[S]) -> BTreeMap<String, Value>
    where
        S: AsRef<str> + Sync,
    {
        let mut out = BTreeMap::new();
        for (key, value) in self.0.iter() {
            out.insert(key.clone(), value.hydrate(args).await);
        }
        out
    }
}

impl PresetBlob {
    /// Hydrate a preset blob with the given arguments
    #[async_recursion::async_recursion]
    pub async fn hydrate<S>(&self, args: &[S]) -> Value
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
                    out.push(x.hydrate(args).await);
                }
                Value::Array(out)
            }
            Self::Object(obj) => {
                let mut out = json!({});
                for (key, val) in obj {
                    out[key] = val.hydrate(args).await;
                }
                out
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const ARGS: &[&str] = &["hello", "world", "temp"];

    #[tokio::test]
    async fn test_emptyobj() {
        let preset = Preset::compile(json!({})).await.unwrap();

        assert_eq!(preset.hydrate(ARGS).await, [].into_iter().collect());
    }

    #[tokio::test]
    async fn test_one_level() {
        let preset = Preset::compile(json!({
            "a": "foo$(1)",
            "b": "world",
            "c": "te$(0)mp"
        }))
        .await
        .unwrap();

        assert_eq!(
            preset.hydrate(ARGS).await,
            [
                ("a".to_string(), json!("fooworld")),
                ("b".to_string(), json!("world")),
                ("c".to_string(), json!("tehellomp")),
            ]
            .into_iter()
            .collect()
        );
    }

    #[tokio::test]
    async fn test_nested() {
        let preset = Preset::compile(json!({
            "a": "foo$(1)",
            "b": ["world$(2)", {
                "c": "te$(0)mp"
            }],
            "c": "temp$(0)$(1)"
        }))
        .await
        .unwrap();

        assert_eq!(
            preset.hydrate(ARGS).await,
            [
                ("a".to_string(), json!("fooworld")),
                (
                    "b".to_string(),
                    json!(["worldtemp", {
                        "c": "tehellomp"
                    }])
                ),
                ("c".to_string(), json!("temphelloworld")),
            ]
            .into_iter()
            .collect()
        );
    }
}
