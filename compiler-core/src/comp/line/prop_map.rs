use std::collections::BTreeMap;
use std::ops::{DerefMut, Deref};

use crate::json::{IntoSafeRouteBlob, SafeRouteBlob};
use crate::prop;

#[repr(transparent)]
pub struct LinePropMap<'a> {
    inner: BTreeMap<String, SafeRouteBlob<'a>>,
}

impl<'a> Deref for LinePropMap<'a> {
    type Target = BTreeMap<String, SafeRouteBlob<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a> DerefMut for LinePropMap<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<'a> LinePropMap<'a> {
    pub fn new() -> Self {
        Self {
            inner: BTreeMap::new(),
        }
    }

    /// Insert a property and automatically desugar it
    pub fn insert<T>(&mut self, key: String, value: T) 
    where T: IntoSafeRouteBlob + 'a
    {
        let value = value.into_unchecked();
        match key.as_ref() {
            prop::COORD => {
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
    // /// Insert a value directly.
    // pub fn insert_value(&mut self, key: String, value: SafeRouteBlob<'a>) {
    //     self.normal.remove(&key);
    //     self.desugared.insert(key, value);
    // }
    // pub fn get(&self, key: &str) -> Option<SafeRouteBlob<'_>> {
    //     match self.normal.get(key) {
    //         Some(x) => Some(x.ref_into_unchecked()),
    //         None => self.desugared.get(key).map(|x| x.ref_into_unchecked()),
    //     }
    // }
    // pub fn remove(&mut self, key: &str) -> Option<SafeRouteBlob<'_>> {
    //     match self.normal.remove(key) {
    //         Some(x) => Some(x.into_unchecked()),
    //         None => self.desugared.remove(key).map(|x| x.into_unchecked()),
    //     }
    // }
    //
    // pub fn extend(&mut self, other: Self) {
    //     self.inner.extend(other.inner);
    // }
    //
    pub fn evaluate(self) -> BTreeMap<String, SafeRouteBlob<'a>> {
        // for (key, value) in self.normal {
        //     self.desugared.insert(key, value.into_unchecked());
        // }
        // self.desugared
        self.inner
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
        let value: Value = properties.get("movements").unwrap().ref_into_unchecked().into();
        assert_eq!(value, json!([[1, 2]]));
    }

    #[test]
    fn test_properties_icon() {
        let mut properties = LinePropMap::new();
        properties.insert("icon".to_string(), json!([1, 2]));
        assert!(properties.get("icon").is_none());
        let value: Value = properties.get("icon-doc").unwrap().ref_into_unchecked().into();
        assert_eq!(value, json!([1, 2]));
        let value: Value = properties.get("icon-map").unwrap().ref_into_unchecked().into();
        assert_eq!(value, json!([1, 2]));
    }
}
