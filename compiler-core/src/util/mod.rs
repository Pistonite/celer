use std::ops::Deref;

mod path;
pub use path::*;
mod coop;
pub use coop::*;
mod site_origin;
pub use site_origin::*;

/// Ref counted pointer.
///
/// Will be Rc for WASM and Arc otherwise.
pub struct RefCounted<T> where T: ?Sized {
    #[cfg(not(feature = "wasm"))]
    inner: std::sync::Arc<T>,
    #[cfg(feature = "wasm")]
    inner: std::rc::Rc<T>,
}

impl<T> RefCounted<T> where T: ?Sized{
    pub fn new(inner: T) -> Self {
        Self {
            #[cfg(not(feature = "wasm"))]
            inner: std::sync::Arc::new(inner),
            #[cfg(feature = "wasm")]
            inner: std::rc::Rc::new(inner),
        }
    }
}

impl Clone for RefCounted<()> {
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

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
