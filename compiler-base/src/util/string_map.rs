//! A wrapper for BTreeMap to be compatible with TypeScript Record type

use std::collections::BTreeMap;
use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};

use crate::macros::derive_wasm;

#[derive(PartialEq, Default, Serialize, Deserialize, Debug, Clone)]
#[derive_wasm]
#[repr(transparent)]
pub struct StringMap<T>(
    #[allow_map]
    #[tsify(type = "Record<string, T>")]
    pub(crate) BTreeMap<String, T>,
)
where
    T: Serialize;

impl<T> From<BTreeMap<String, T>> for StringMap<T>
where
    T: Serialize,
{
    #[inline]
    fn from(map: BTreeMap<String, T>) -> Self {
        Self(map)
    }
}

impl<T> Deref for StringMap<T>
where
    T: Serialize,
{
    type Target = BTreeMap<String, T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for StringMap<T>
where
    T: Serialize,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
