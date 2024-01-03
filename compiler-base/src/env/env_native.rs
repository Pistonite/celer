//! Native environment implementation

use std::sync::Arc;
use std::future::Future;
use tokio::task::JoinSet;

/// Ref counted pointer, wrapper for Arc
#[derive(Debug)]
#[repr(transparent)]
pub struct RefCounted<T>
where
    T: ?Sized,
{
    pub(crate) inner: Arc<T>,
}

impl<T> Clone for RefCounted<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> RefCounted<T> {
    #[inline]
    pub fn new(inner: T) -> Self {
        Self {
            inner: Arc::new(inner),
        }
    }
}

impl From<&str> for RefCounted<str> {
    #[inline]
    fn from(s: &str) -> Self {
        Self {
            inner: Arc::from(s),
        }
    }
}

pub async fn yield_budget(limit: u32) {
    if super::coop_tick_increment(limit) {
        tokio::task::yield_now().await;
    }
}

/// Wait for multiple futures to complete
pub use tokio::join as join_futures;
