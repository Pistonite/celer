//! Route JSON blob utilities
//!
//! There are two main types of route blobs: [`RouteBlob`] and [`SafeRouteBlob`].
//! `RouteBlob` is used to hold data and error from building the route, and
//! can be turned into a `SafeRouteBlob` after checking for errors. The `SafeRouteBlob`
//! implements [`Coerce`] and [`Cast`] to allow for easy conversion to other types.
//!
//! When casting `SafeRouteBlob`, it returns borrowed type as needed to avoid copying
//! as much as possible.

use std::borrow::Cow;
use std::collections::BTreeMap;

use serde_json::{Map, Value};

use crate::res::ResError;

use super::{Cast, Coerce};

/// A route JSON blob representing the state after resolving `use`s,
/// which could contain errors.
///
/// This is used to expose errors to the compiler, so it can be displayed
/// using the diagnostics API
#[derive(Debug, Clone, PartialEq)]
pub enum RouteBlob {
    /// Primitive value (may be array or object, but does not contain error)
    Prim(Value),
    /// Error
    Err(RouteBlobError),
    /// Array of route blobs
    Array(Vec<RouteBlob>),
    /// Object of route blobs
    Object(BTreeMap<String, RouteBlob>),
}

impl From<Value> for RouteBlob {
    fn from(v: Value) -> Self {
        Self::Prim(v)
    }
}

impl From<ResError> for RouteBlob {
    fn from(e: ResError) -> Self {
        Self::Err(RouteBlobError::ResError(e.to_string()))
    }
}

#[derive(Debug, PartialEq, Clone, thiserror::Error)]
pub enum RouteBlobError {
    #[error("Failed to load resource: {0}")]
    ResError(String),

    #[error("Max depth of {0} levels of `use` is reached. Please make sure there are no circular dependencies.")]
    MaxUseDepthExceeded(usize),

    #[error("Max reference depth of {0} levels is reached. There might be a formatting error in your project files.")]
    MaxRefDepthExceeded(usize),
}

/// A route blob that is guaranteed to not contain errors
#[derive(Debug, PartialEq)]
pub enum SafeRouteBlob<'a> {
    BorrowedValue(&'a Value),
    BorrowedBlob(&'a RouteBlob),
    OwnedBlob(RouteBlob),
    BorrowedArray(&'a Vec<SafeRouteBlob<'a>>),
    OwnedArray(Vec<SafeRouteBlob<'a>>),
    BorrowedObject(&'a BTreeMap<String, SafeRouteBlob<'a>>),
    OwnedObject(BTreeMap<String, SafeRouteBlob<'a>>),
}

impl Clone for SafeRouteBlob<'_> {
    fn clone(&self) -> Self {
        match self {
            Self::BorrowedValue(v) => Self::BorrowedValue(v),
            Self::BorrowedBlob(b) => Self::BorrowedBlob(b),
            Self::OwnedBlob(b) => Self::OwnedBlob(b.clone()),
            Self::BorrowedArray(arr) => Self::BorrowedArray(arr),
            Self::OwnedArray(arr) => Self::OwnedArray(arr.clone()),
            Self::BorrowedObject(obj) => Self::BorrowedObject(obj),
            Self::OwnedObject(obj) => Self::OwnedObject(obj.clone()),
        }
    }
}

/// Convert the route blob to raw JSON, copying anything that is a borrowed value
///
/// This should only be used in tests
impl<'a> From<SafeRouteBlob<'a>> for Value {
    fn from(x: SafeRouteBlob<'a>) -> Self {
        match x {
            SafeRouteBlob::BorrowedValue(v) => v.clone(),
            SafeRouteBlob::BorrowedBlob(b) => route_blob_to_value(b.clone()),
            SafeRouteBlob::OwnedBlob(b) => route_blob_to_value(b),
            SafeRouteBlob::BorrowedArray(arr) => {
                Value::Array(arr.iter().map(|x| Value::from(x.clone())).collect())
            }
            SafeRouteBlob::OwnedArray(arr) => {
                Value::Array(arr.into_iter().map(|x| Value::from(x)).collect())
            }
            SafeRouteBlob::BorrowedObject(obj) => {
                Value::Object(obj.iter().map(|(k, v)| (k.clone(), Value::from(v.clone()))).collect())
            }
            SafeRouteBlob::OwnedObject(obj) => {
                Value::Object(obj.into_iter().map(|(k, v)| (k, Value::from(v))).collect())
            }
        }
    }
}

fn route_blob_to_value(x: RouteBlob) -> Value {
    match x {
        RouteBlob::Prim(v) => v,
        RouteBlob::Err(_) => Value::Null,
        RouteBlob::Array(arr) => Value::Array(arr.into_iter().map(route_blob_to_value).collect()),
        RouteBlob::Object(obj) => Value::Object(
            obj.into_iter()
                .map(|(k, v)| (k, route_blob_to_value(v)))
                .collect(),
        ),
    }
}

/// Internal trait to convert a value into a safe route blob
///
/// The method are called `unchecked` because they don't check for errors
pub trait IntoSafeRouteBlob {
    fn into_unchecked<'a>(self) -> SafeRouteBlob<'a>
    where
        Self: 'a;
    fn ref_into_unchecked(&self) -> SafeRouteBlob<'_>;
}

impl IntoSafeRouteBlob for Value {
    fn into_unchecked<'a>(self) -> SafeRouteBlob<'a> {
        SafeRouteBlob::OwnedBlob(RouteBlob::Prim(self))
    }

    fn ref_into_unchecked(&self) -> SafeRouteBlob<'_> {
        SafeRouteBlob::BorrowedValue(self)
    }
}

impl IntoSafeRouteBlob for RouteBlob {
    fn into_unchecked<'a>(self) -> SafeRouteBlob<'a> {
        SafeRouteBlob::OwnedBlob(self)
    }

    fn ref_into_unchecked(&self) -> SafeRouteBlob<'_> {
        SafeRouteBlob::BorrowedBlob(self)
    }
}

impl<'a> IntoSafeRouteBlob for SafeRouteBlob<'a> {
    fn into_unchecked<'b>(self) -> SafeRouteBlob<'b>
    where
        Self: 'b,
    {
        self
    }

    fn ref_into_unchecked(&self) -> SafeRouteBlob<'_> {
        match self {
            Self::BorrowedValue(v) => SafeRouteBlob::BorrowedValue(v),
            Self::BorrowedBlob(b) => SafeRouteBlob::BorrowedBlob(b),
            Self::OwnedBlob(b) => SafeRouteBlob::BorrowedBlob(b),
            Self::BorrowedArray(arr) => SafeRouteBlob::BorrowedArray(arr),
            Self::OwnedArray(arr) => SafeRouteBlob::BorrowedArray(arr),
            Self::BorrowedObject(obj) => SafeRouteBlob::BorrowedObject(obj),
            Self::OwnedObject(obj) => SafeRouteBlob::BorrowedObject(obj),
        }
    }
}

impl RouteBlob {
    /// Check if self contains error. Returns `Ok` if not, or the first error found
    pub fn checked(&self) -> Result<SafeRouteBlob<'_>, RouteBlobError> {
        match self.error() {
            Some(e) => Err(e),
            None => Ok(self.ref_into_unchecked()),
        }
    }

    /// Like `checked`, but consumes self to avoid copying later
    pub fn into_checked(self) -> Result<SafeRouteBlob<'static>, RouteBlobError> {
        match self.error() {
            Some(e) => Err(e),
            None => Ok(self.into_unchecked()),
        }
    }

    fn error(&self) -> Option<RouteBlobError> {
        match self {
            Self::Err(e) => Some(e.clone()),
            Self::Prim(_) => None,
            Self::Array(arr) => {
                for blob in arr {
                    if let Some(e) = blob.error() {
                        return Some(e);
                    }
                }
                None
            }
            Self::Object(obj) => {
                for blob in obj.values() {
                    if let Some(e) = blob.error() {
                        return Some(e);
                    }
                }
                None
            }
        }
    }
}

impl<'a> Cast for SafeRouteBlob<'a> {
    type Array = SafeRouteArray<'a>;
    type Object = SafeRouteObject<'a>;
    type AsArray<'b> = SafeRouteArray<'b> where 'a: 'b;
    type AsObject<'b> = SafeRouteObject<'b> where 'a: 'b;

    fn try_into_array(self) -> Result<Self::Array, Self> {
        match self {
            Self::BorrowedValue(Value::Array(arr)) => Ok(SafeRouteArray::BorrowedValue(arr)),
            Self::BorrowedBlob(RouteBlob::Array(arr)) => Ok(SafeRouteArray::BorrowedBlob(arr)),
            Self::BorrowedBlob(RouteBlob::Prim(Value::Array(arr))) => {
                Ok(SafeRouteArray::BorrowedValue(arr))
            }
            Self::OwnedBlob(RouteBlob::Array(arr)) => Ok(SafeRouteArray::OwnedBlob(arr)),
            Self::OwnedBlob(RouteBlob::Prim(Value::Array(arr))) => {
                Ok(SafeRouteArray::OwnedValue(arr))
            }
            Self::BorrowedArray(arr) => Ok(SafeRouteArray::BorrowedSafe(arr)),
            Self::OwnedArray(arr) => Ok(SafeRouteArray::OwnedSafe(arr)),
            _ => Err(self),
        }
    }

    fn try_into_object(self) -> Result<Self::Object, Self> {
        match self {
            Self::BorrowedValue(Value::Object(obj)) => Ok(SafeRouteObject::BorrowedValue(obj)),
            Self::BorrowedBlob(RouteBlob::Object(obj)) => Ok(SafeRouteObject::BorrowedBlob(obj)),
            Self::BorrowedBlob(RouteBlob::Prim(Value::Object(obj))) => {
                Ok(SafeRouteObject::BorrowedValue(obj))
            }
            Self::OwnedBlob(RouteBlob::Object(obj)) => Ok(SafeRouteObject::OwnedBlob(obj)),
            Self::OwnedBlob(RouteBlob::Prim(Value::Object(obj))) => {
                Ok(SafeRouteObject::OwnedValue(obj))
            }
            Self::BorrowedObject(obj) => Ok(SafeRouteObject::BorrowedSafe(obj)),
            Self::OwnedObject(obj) => Ok(SafeRouteObject::OwnedSafe(obj)),
            _ => Err(self),
        }
    }

    fn as_array(&self) -> Option<Self::AsArray<'_>> {
        match self {
            Self::BorrowedValue(Value::Array(arr)) => Some(SafeRouteArray::BorrowedValue(arr)),
            Self::BorrowedBlob(RouteBlob::Array(arr)) => Some(SafeRouteArray::BorrowedBlob(arr)),
            Self::BorrowedBlob(RouteBlob::Prim(Value::Array(arr))) => {
                Some(SafeRouteArray::BorrowedValue(arr))
            }
            Self::OwnedBlob(RouteBlob::Array(arr)) => Some(SafeRouteArray::BorrowedBlob(arr)),
            Self::OwnedBlob(RouteBlob::Prim(Value::Array(arr))) => {
                Some(SafeRouteArray::BorrowedValue(arr))
            }
            Self::BorrowedArray(arr) => Some(SafeRouteArray::BorrowedSafe(arr)),
            Self::OwnedArray(arr) => Some(SafeRouteArray::BorrowedSafe(arr)),
            _ => None,
        }
    }

    fn as_object(&self) -> Option<Self::AsObject<'_>> {
        match self {
            Self::BorrowedValue(Value::Object(obj)) => Some(SafeRouteObject::BorrowedValue(obj)),
            Self::BorrowedBlob(RouteBlob::Object(obj)) => Some(SafeRouteObject::BorrowedBlob(obj)),
            Self::BorrowedBlob(RouteBlob::Prim(Value::Object(obj))) => {
                Some(SafeRouteObject::BorrowedValue(obj))
            }
            Self::OwnedBlob(RouteBlob::Object(obj)) => Some(SafeRouteObject::BorrowedBlob(obj)),
            Self::OwnedBlob(RouteBlob::Prim(Value::Object(obj))) => {
                Some(SafeRouteObject::BorrowedValue(obj))
            }
            Self::BorrowedObject(obj) => Some(SafeRouteObject::BorrowedSafe(obj)),
            Self::OwnedObject(obj) => Some(SafeRouteObject::BorrowedSafe(obj)),
            _ => None,
        }
    }

    fn as_str(&self) -> Option<&str> {
        match self {
            Self::BorrowedValue(Value::String(s)) => Some(s),
            Self::BorrowedBlob(RouteBlob::Prim(Value::String(s))) => Some(s),
            Self::OwnedBlob(RouteBlob::Prim(Value::String(s))) => Some(s),
            _ => None,
        }
    }
}

impl<'a> SafeRouteBlob<'a> {
    fn as_prim(&self) -> Option<&Value> {
        match self {
            Self::BorrowedValue(v) => Some(v),
            Self::BorrowedBlob(RouteBlob::Prim(v)) => Some(v),
            Self::OwnedBlob(RouteBlob::Prim(v)) => Some(v),
            _ => None,
        }
    }
}

impl<'a> Coerce for SafeRouteBlob<'a> {
    fn is_null(&self) -> bool {
        match self {
            Self::BorrowedValue(v) => v.is_null(),
            Self::BorrowedBlob(RouteBlob::Prim(v)) => v.is_null(),
            Self::OwnedBlob(RouteBlob::Prim(v)) => v.is_null(),
            _ => false,
        }
    }

    fn coerce_to_string(&self) -> String {
        match self {
            Self::BorrowedValue(v) => v.coerce_to_string(),
            Self::BorrowedBlob(RouteBlob::Prim(v)) => v.coerce_to_string(),
            Self::OwnedBlob(RouteBlob::Prim(v)) => v.coerce_to_string(),
            Self::BorrowedBlob(RouteBlob::Err(_)) | Self::OwnedBlob(RouteBlob::Err(_)) => {
                "[object error]".to_string()
            }
            Self::BorrowedArray(_)
            | Self::OwnedArray(_)
            | Self::BorrowedBlob(RouteBlob::Array(_))
            | Self::OwnedBlob(RouteBlob::Array(_)) => "[object array]".to_string(),
            Self::BorrowedObject(_)
            | Self::OwnedObject(_)
            | Self::BorrowedBlob(RouteBlob::Object(_))
            | Self::OwnedBlob(RouteBlob::Object(_)) => "[object object]".to_string(),
        }
    }

    fn coerce_into_string(self) -> String {
        match self {
            Self::OwnedBlob(RouteBlob::Prim(x)) => x.coerce_into_string(),
            _ => self.coerce_to_string(),
        }
    }

    fn coerce_truthy(&self) -> bool {
        match self {
            Self::BorrowedValue(v) => v.coerce_truthy(),
            Self::BorrowedBlob(RouteBlob::Prim(v)) => v.coerce_truthy(),
            Self::OwnedBlob(RouteBlob::Prim(v)) => v.coerce_truthy(),
            Self::BorrowedBlob(RouteBlob::Err(_)) => false,
            Self::OwnedBlob(RouteBlob::Err(_)) => false,
            _ => true,
        }
    }

    fn try_coerce_to_f64(&self) -> Option<f64> {
        self.as_prim()?.try_coerce_to_f64()
    }

    fn try_coerce_to_u64(&self) -> Option<u64> {
        self.as_prim()?.try_coerce_to_u64()
    }

    fn try_coerce_to_u32(&self) -> Option<u32> {
        self.as_prim()?.try_coerce_to_u32()
    }

    fn try_coerce_to_i64(&self) -> Option<i64> {
        self.as_prim()?.try_coerce_to_i64()
    }

    fn try_coerce_to_bool(&self) -> Option<bool> {
        self.as_prim()?.try_coerce_to_bool()
    }
}

/// View of a [`SafeRouteBlob`] as an array
pub enum SafeRouteArray<'a> {
    BorrowedValue(&'a [Value]),
    OwnedValue(Vec<Value>),
    BorrowedBlob(&'a [RouteBlob]),
    OwnedBlob(Vec<RouteBlob>),
    BorrowedSafe(&'a [SafeRouteBlob<'a>]),
    OwnedSafe(Vec<SafeRouteBlob<'a>>),
}

impl<'a> SafeRouteArray<'a> {
    /// Get the length of the array
    #[inline]
    pub fn len(&self) -> usize {
        match self {
            Self::BorrowedValue(arr) => arr.len(),
            Self::OwnedValue(arr) => arr.len(),
            Self::BorrowedBlob(arr) => arr.len(),
            Self::OwnedBlob(arr) => arr.len(),
            Self::BorrowedSafe(arr) => arr.len(),
            Self::OwnedSafe(arr) => arr.len(),
        }
    }
}

impl<'a> IntoIterator for SafeRouteArray<'a> {
    type Item = SafeRouteBlob<'a>;
    type IntoIter = SafeRouteArrayIntoIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Self::BorrowedValue(arr) => SafeRouteArrayIntoIter::BorrowedValue(arr.iter()),
            Self::OwnedValue(arr) => SafeRouteArrayIntoIter::OwnedValue(arr.into_iter()),
            Self::BorrowedBlob(arr) => SafeRouteArrayIntoIter::BorrowedBlob(arr.iter()),
            Self::OwnedBlob(arr) => SafeRouteArrayIntoIter::OwnedBlob(arr.into_iter()),
            Self::BorrowedSafe(arr) => SafeRouteArrayIntoIter::BorrowedSafe(arr.iter()),
            Self::OwnedSafe(arr) => SafeRouteArrayIntoIter::OwnedSafe(arr.into_iter()),
        }
    }
}

pub enum SafeRouteArrayIntoIter<'a> {
    BorrowedValue(std::slice::Iter<'a, Value>),
    OwnedValue(std::vec::IntoIter<Value>),
    BorrowedBlob(std::slice::Iter<'a, RouteBlob>),
    OwnedBlob(std::vec::IntoIter<RouteBlob>),
    BorrowedSafe(std::slice::Iter<'a, SafeRouteBlob<'a>>),
    OwnedSafe(std::vec::IntoIter<SafeRouteBlob<'a>>),
}

impl<'a> Iterator for SafeRouteArrayIntoIter<'a> {
    type Item = SafeRouteBlob<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::BorrowedValue(iter) => iter.next().map(|x| x.ref_into_unchecked()),
            Self::OwnedValue(iter) => iter.next().map(|x| x.into_unchecked()),
            Self::BorrowedBlob(iter) => iter.next().map(|x| x.ref_into_unchecked()),
            Self::OwnedBlob(iter) => iter.next().map(|x| x.into_unchecked()),
            Self::BorrowedSafe(iter) => iter.next().map(|x| x.ref_into_unchecked()),
            Self::OwnedSafe(iter) => iter.next(),
        }
    }
}

/// View of a [`SafeRouteBlob`] as an object
pub enum SafeRouteObject<'a> {
    BorrowedValue(&'a Map<String, Value>),
    OwnedValue(Map<String, Value>),
    BorrowedBlob(&'a BTreeMap<String, RouteBlob>),
    OwnedBlob(BTreeMap<String, RouteBlob>),
    BorrowedSafe(&'a BTreeMap<String, SafeRouteBlob<'a>>),
    OwnedSafe(BTreeMap<String, SafeRouteBlob<'a>>),
}

impl<'a> SafeRouteObject<'a> {
    /// Create an empty object
    pub fn new() -> Self {
        Self::OwnedSafe(BTreeMap::new())
    }

    /// Get the length of the object
    #[inline]
    pub fn len(&self) -> usize {
        match self {
            Self::BorrowedValue(obj) => obj.len(),
            Self::OwnedValue(obj) => obj.len(),
            Self::BorrowedBlob(obj) => obj.len(),
            Self::OwnedBlob(obj) => obj.len(),
            Self::BorrowedSafe(obj) => obj.len(),
            Self::OwnedSafe(obj) => obj.len(),
        }
    }

    /// Get the value of a key
    #[inline]
    pub fn get(&self, key: &str) -> Option<SafeRouteBlob<'_>> {
        match self {
            Self::BorrowedValue(obj) => obj.get(key).map(|x| x.ref_into_unchecked()),
            Self::OwnedValue(obj) => obj.get(key).map(|x| x.ref_into_unchecked()),
            Self::BorrowedBlob(obj) => obj.get(key).map(|x| x.ref_into_unchecked()),
            Self::OwnedBlob(obj) => obj.get(key).map(|x| x.ref_into_unchecked()),
            Self::BorrowedSafe(obj) => obj.get(key).map(|x| x.ref_into_unchecked()),
            Self::OwnedSafe(obj) => obj.get(key).map(|x| x.ref_into_unchecked()),
        }
    }
}

impl<'a> IntoIterator for SafeRouteObject<'a> {
    type Item = (Cow<'a, str>, SafeRouteBlob<'a>);
    type IntoIter = SafeRouteObjectIntoIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Self::BorrowedValue(obj) => SafeRouteObjectIntoIter::BorrowedValue(obj.iter()),
            Self::OwnedValue(obj) => SafeRouteObjectIntoIter::OwnedValue(obj.into_iter()),
            Self::BorrowedBlob(obj) => SafeRouteObjectIntoIter::BorrowedBlob(obj.iter()),
            Self::OwnedBlob(obj) => SafeRouteObjectIntoIter::OwnedBlob(obj.into_iter()),
            Self::BorrowedSafe(obj) => SafeRouteObjectIntoIter::BorrowedSafe(obj.iter()),
            Self::OwnedSafe(obj) => SafeRouteObjectIntoIter::OwnedSafe(obj.into_iter()),
        }
    }
}

pub enum SafeRouteObjectIntoIter<'a> {
    BorrowedValue(serde_json::map::Iter<'a>),
    OwnedValue(serde_json::map::IntoIter),
    BorrowedBlob(std::collections::btree_map::Iter<'a, String, RouteBlob>),
    OwnedBlob(std::collections::btree_map::IntoIter<String, RouteBlob>),
    BorrowedSafe(std::collections::btree_map::Iter<'a, String, SafeRouteBlob<'a>>),
    OwnedSafe(std::collections::btree_map::IntoIter<String, SafeRouteBlob<'a>>),
}

impl<'a> Iterator for SafeRouteObjectIntoIter<'a> {
    type Item = (Cow<'a, str>, SafeRouteBlob<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::BorrowedValue(iter) => iter
                .next()
                .map(|(k, v)| (Cow::Borrowed(k.as_str()), v.ref_into_unchecked())),
            Self::OwnedValue(iter) => iter
                .next()
                .map(|(k, v)| (Cow::Owned(k), v.into_unchecked())),
            Self::BorrowedBlob(iter) => iter
                .next()
                .map(|(k, v)| (Cow::Borrowed(k.as_str()), v.ref_into_unchecked())),
            Self::OwnedBlob(iter) => iter
                .next()
                .map(|(k, v)| (Cow::Owned(k), v.into_unchecked())),
            Self::BorrowedSafe(iter) => iter
                .next()
                .map(|(k, v)| (Cow::Borrowed(k.as_str()), v.ref_into_unchecked())),
            Self::OwnedSafe(iter) => iter.next().map(|(k, v)| (Cow::Owned(k), v)),
        }
    }
}
