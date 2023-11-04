use std::collections::BTreeMap;

use serde_json::{Map, Value};

use crate::json::Cast;

use super::PackerError;

/// JSON value with an Err variant
///
/// This is used to expose errors to the compiler, so it can be displayed
/// using the diagnostics API
pub enum PackerValue {
    Ok(Value),
    Err(PackerError),
    Array(Vec<PackerValue>),
    Object(BTreeMap<String, PackerValue>),
}

impl Cast for PackerValue {
    type Object = BTreeMap<String, PackerValue>;

    fn try_into_object(self) -> Result<<PackerValue as Cast>::Object, Self> {
        match self {
            Self::Ok(v) => match v.try_into_object() {
                Ok(v) => {
                    let mut new_obj = BTreeMap::new();
                    for (key, value) in v.into_iter() {
                        new_obj.insert(key, Self::Ok(value));
                    }
                    Ok(new_obj)
                }
                Err(v) => Err(Self::Ok(v)),
            },
            Self::Object(v) => Ok(v),
            _ => Err(self),
        }
    }

    fn try_into_array(self) -> Result<Vec<Self>, Self> {
        match self {
            Self::Ok(v) => match v.try_into_array() {
                Ok(v) => {
                    let mut new_arr = vec![];
                    for x in v.into_iter() {
                        new_arr.push(Self::Ok(x));
                    }
                    Ok(new_arr)
                }
                Err(v) => Err(Self::Ok(v)),
            },
            Self::Array(v) => Ok(v),
            _ => Err(self),
        }
    }
}

impl PackerValue {
    pub fn is_object(&self) -> bool {
        match self {
            Self::Object(_) => true,
            Self::Ok(v) => v.is_object(),
            _ => false,
        }
    }

    pub fn is_array(&self) -> bool {
        match self {
            Self::Array(_) => true,
            Self::Ok(v) => v.is_array(),
            _ => false,
        }
    }

    /// Flatten the errors.
    ///
    /// If the value contains any error, returns `Err` with all the errors,
    /// otherwise, returns Ok with the inner value.
    pub fn flatten(self) -> Result<Value, Vec<PackerError>> {
        let mut errors = vec![];
        let flattened = self.flatten_internal(&mut errors);

        if errors.is_empty() {
            match flattened {
                Some(x) => Ok(x),
                _ => Err(errors),
            }
        } else {
            Err(errors)
        }
    }

    fn flatten_internal(self, output_errors: &mut Vec<PackerError>) -> Option<Value> {
        match self {
            Self::Ok(x) => Some(x),
            Self::Err(x) => {
                output_errors.push(x);
                None
            }
            Self::Array(v) => {
                let mut new_arr = vec![];
                for x in v.into_iter() {
                    if let Some(x) = x.flatten_internal(output_errors) {
                        new_arr.push(x);
                    }
                }
                Some(Value::Array(new_arr))
            }
            Self::Object(o) => {
                let mut new_obj = Map::new();
                for (key, value) in o.into_iter() {
                    if let Some(x) = value.flatten_internal(output_errors) {
                        new_obj.insert(key, x);
                    }
                }
                Some(Value::Object(new_obj))
            }
        }
    }
}
