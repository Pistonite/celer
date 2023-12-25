//! Native environment implementation

use std::sync::Arc;

/// Ref counted pointer, wrapper for Arc
#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct RefCounted<T> where T: ?Sized {
    inner: Arc<T>,
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

pub use tokio::join as join_futures;
