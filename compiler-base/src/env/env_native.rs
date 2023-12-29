//! Native environment implementation

use std::sync::Arc;
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

pub fn iter_futures<I, T>(budget: u32, iter: I) -> Vec<T>
where
    I: IntoIterator,
    I::Item: Future<Output = T>,
{
    let mut set = JoinSet::new();
    let mut results = Vec::new();
    for (i, future) in iter.into_iter().enumerate() {
        set.spawn(async move {
            (i, future.await)
        });
        results.push(None);
    }
    let mut joined: usize = 0;
    while let Some(result) = set.join_next().await {
        joined += 1;
        match result {
            Ok((i, result)) => {
                results.get_mut(i).unwrap() = Some(result);
            }
            Err(e) => {
                if e.is_panic() {
                    panic!("Panic in async task: {:?}", e);
                }
            }
        }
        yield_budget(budget).await;
    }
    if joined != results.len() {
        panic!("Not all futures joined");
    }
    results.into_iter().map(|r| r.unwrap())
}

