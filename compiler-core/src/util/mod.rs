use std::ops::Deref;

// mod path;
// pub use path::*;
mod coop;
pub use coop::*;
mod site_origin;
pub use site_origin::*;

/// Ref counted pointer.
///
/// Will be Rc for WASM and Arc otherwise.
#[derive(Debug)]
#[repr(transparent)]
pub struct RefCounted<T> where T: ?Sized {
    #[cfg(not(feature = "wasm"))]
    inner: std::sync::Arc<T>,
    #[cfg(feature = "wasm")]
    inner: std::rc::Rc<T>,
}

impl<T> RefCounted<T> {
    #[inline]
    pub fn new(inner: T) -> Self {
        Self {
            #[cfg(not(feature = "wasm"))]
            inner: std::sync::Arc::new(inner),
            #[cfg(feature = "wasm")]
            inner: std::rc::Rc::new(inner),
        }
    }
}

impl<L> Clone for RefCounted<L> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            #[cfg(not(feature = "wasm"))]
            inner: std::sync::Arc::clone(&self.inner),
            #[cfg(feature = "wasm")]
            inner: std::rc::Rc::clone(&self.inner),
        }
    }
}

impl<T> Deref for RefCounted<T> where T: ?Sized{
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<&str> for RefCounted<str> {
    #[inline]
    #[cfg(not(feature = "wasm"))]
    fn from(s: &str) -> Self {
        Self {
            inner: std::sync::Arc::from(s),
        }
    }
    #[inline]
    #[cfg(feature = "wasm")]
    fn from(s: &str) -> Self {
        Self {
            inner: std::rc::Rc::from(s),
        }
    }
}

// re-exports
pub use uni_path::{Path, PathBuf};
