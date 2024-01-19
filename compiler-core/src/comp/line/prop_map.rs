use std::collections::BTreeMap;

use crate::json::{IntoSafeRouteBlob, SafeRouteBlob};
use crate::lang::HydrateTarget;
use crate::prop;

#[repr(transparent)]
#[derive(Debug, Default)]
pub struct LinePropMap<'a> {
    inner: BTreeMap<String, SafeRouteBlob<'a>>,
}

impl<'a> HydrateTarget<'a> for LinePropMap<'a> {
    /// Insert a property and automatically desugar it
    fn insert<T>(&mut self, key: String, value: T)
    where
        T: IntoSafeRouteBlob + 'a,
    {
        let value = value.into_unchecked();
        match key.as_ref() {
            prop::COORD => {
                let value = SafeRouteBlob::OwnedArray(vec![value]);
                self.insert(prop::MOVEMENTS.to_string(), value);
            }
            prop::ICON => {
                self.insert(prop::ICON_DOC.to_string(), value.clone());
                self.insert(prop::ICON_MAP.to_string(), value);
            }
            _ => {
                self.inner.insert(key, value);
            }
        }
    }
}

impl<'a> LinePropMap<'a> {
    pub fn new() -> Self {
        Self {
            inner: BTreeMap::new(),
        }
    }

    /// Convert self to a BTreeMap with properties evaluated to SafeRouteBlob
    pub fn evaluate(self) -> BTreeMap<String, SafeRouteBlob<'a>> {
        self.inner
    }

    // delegate methods
    // not using deref here since insert could be ambiguous/confusing
    #[inline]
    pub fn remove(&mut self, key: &str) -> Option<SafeRouteBlob<'a>> {
        self.inner.remove(key)
    }
    #[inline]
    pub fn get(&self, key: &str) -> Option<&SafeRouteBlob<'a>> {
        self.inner.get(key)
    }
    #[inline]
    pub fn extend<T>(&mut self, other: T)
    where
        T: IntoIterator<Item = (String, SafeRouteBlob<'a>)>,
    {
        self.inner.extend(other);
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
        let value: Value = properties
            .get("movements")
            .unwrap()
            .ref_into_unchecked()
            .into();
        assert_eq!(value, json!([[1, 2]]));
    }

    #[test]
    fn test_properties_icon() {
        let mut properties = LinePropMap::new();
        properties.insert("icon".to_string(), json!([1, 2]));
        assert!(properties.get("icon").is_none());
        let value: Value = properties
            .get("icon-doc")
            .unwrap()
            .ref_into_unchecked()
            .into();
        assert_eq!(value, json!([1, 2]));
        let value: Value = properties
            .get("icon-map")
            .unwrap()
            .ref_into_unchecked()
            .into();
        assert_eq!(value, json!([1, 2]));
    }
}
