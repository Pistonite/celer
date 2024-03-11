//! Native environment implementation

use std::sync::Arc;

/// Ref counted pointer, wrapper for Arc
#[derive(Debug)]
#[repr(transparent)]
pub struct RefCounted<T>
where
    T: ?Sized,
{
    pub(crate) inner: Arc<T>,
}

impl<T> Clone for RefCounted<T>
where
    T: ?Sized,
{
    #[inline]
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
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

impl<T> From<Vec<T>> for RefCounted<[T]> {
    #[inline]
    fn from(v: Vec<T>) -> Self {
        Self {
            inner: Arc::from(v),
        }
    }
}

impl<T> From<Box<T>> for RefCounted<T> 
    where T: ?Sized {
    #[inline]
    fn from(v: Box<T>) -> Self {
        Self {
            inner: Arc::from(v),
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

/// Spawn futures and collect the results in a vec in the same order
pub async fn join_future_vec<TFuture>(v: Vec<TFuture>) -> Vec<Result<TFuture::Output, String>>
where
    TFuture: std::future::Future + Send + 'static,
    TFuture::Output: Send + 'static,
{
    let len = v.len();
    let mut handles = Vec::with_capacity(len);
    for future in v {
        handles.push(tokio::spawn(future));
    }
    let mut results = Vec::with_capacity(len);
    for handle in handles {
        match handle.await {
            Ok(res) => results.push(Ok(res)),
            Err(e) => results.push(Err(e.to_string())),
        }
    }
    results
}
