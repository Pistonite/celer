//! Native environment implementation

use std::sync::Arc;

/// Ref counted pointer, wrapper for Arc
pub type RefCounted<T> = Arc<T>;

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
