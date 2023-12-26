use std::collections::BTreeMap;

use crate::json::{IntoSafeRouteBlob, SafeRouteBlob};
use crate::prop;

pub struct LinePropMap<'a, T>
where
    T: IntoSafeRouteBlob,
{
    normal: BTreeMap<String, T>,
    desugared: BTreeMap<String, SafeRouteBlob<'a>>,
}

impl<'a, T> LinePropMap<'a, T>
where
    T: IntoSafeRouteBlob,
{
    pub fn new() -> Self {
        Self {
            normal: BTreeMap::new(),
            desugared: BTreeMap::new(),
        }
    }
    /// Insert a property and automatically desugar it
    pub fn insert(&mut self, key: String, value: T) {
        match key.as_ref() {
            prop::COORD => {
                let value = value.into_unchecked();
                self.desugared.insert(
                    prop::MOVEMENTS.to_string(),
                    SafeRouteBlob::OwnedArray(vec![value]),
                );
            }
            prop::MOVEMENTS => {
                self.desugared.insert(key, value.into_unchecked());
            }
            prop::ICON => {
                let value = value.into_unchecked();
                self.desugared
                    .insert(prop::ICON_DOC.to_string(), value.clone());
                self.desugared.insert(prop::ICON_MAP.to_string(), value);
            }
            prop::ICON_DOC | prop::ICON_MAP => {
                self.desugared.insert(key, value.into_unchecked());
            }
            _ => {
                self.normal.insert(key, value);
            }
        }
    }
    /// Insert a value directly.
    pub fn insert_value(&mut self, key: String, value: SafeRouteBlob<'a>) {
        self.normal.remove(&key);
        self.desugared.insert(key, value);
    }
    pub fn get(&self, key: &str) -> Option<SafeRouteBlob<'_>> {
        match self.normal.get(key) {
            Some(x) => Some(x.ref_into_unchecked()),
            None => self.desugared.get(key).map(|x| x.ref_into_unchecked()),
        }
    }
    pub fn remove(&self, key: &str) -> Option<SafeRouteBlob<'_>> {
        match self.normal.remove(key) {
            Some(x) => Some(x.into_unchecked()),
            None => self.desugared.remove(key).map(|x| x.into_unchecked()),
        }
    }

    pub fn extend(&mut self, other: Self) {
        self.normal.extend(other.normal);
        self.desugared.extend(other.desugared);
    }

    pub fn evaluate(mut self) -> BTreeMap<String, SafeRouteBlob<'a>> {
        for (key, value) in self.normal {
            self.desugared.insert(key, value.into_unchecked());
        }
        self.desugared
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::{json, Value};

    #[test]
    fn test_properties_coord() {
        let mut properties = LinePropMap::new();
        properties.insert("coord".to_string(), json!([1, 2]));
        assert!(properties.get("coord").is_none());
        let value: Value = properties.get("movements").unwrap().into();
        assert_eq!(value, json!([[1, 2]]));
    }

    #[test]
    fn test_properties_icon() {
        let mut properties = LinePropMap::new();
        properties.insert("icon".to_string(), json!([1, 2]));
        assert!(properties.get("icon").is_none());
        let value: Value = properties.get("icon-doc").unwrap().into();
        assert_eq!(value, json!([1, 2]));
        let value: Value = properties.get("icon-map").unwrap().into();
        assert_eq!(value, json!([1, 2]));
    }
}
