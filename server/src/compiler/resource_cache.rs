use std::sync::Arc;

use cached::{Cached, TimedSizedCache};
use futures::Future;
use tokio::sync::Mutex;

use celerc::res::{ResError, ResResult};

/// Cache for resources loaded over the network with URL
///
/// Cloning the cache instance is cheap and cloned instance will share
/// the same cache data.
/// The cache does not sync writes. When multiple requests
/// to the same resources are made at the same time, both requests
/// will be fetched and the last one that came back will be cached.
///
/// The cache also parses data urls internally without caching
pub struct ResourceCache {
    inner: Arc<Mutex<TimedSizedCache<String, Arc<[u8]>>>>,
}

impl Clone for ResourceCache {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl ResourceCache {
    pub fn new() -> Self {
        let cache = TimedSizedCache::with_size_and_lifespan(128, 301);
        Self {
            inner: Arc::new(Mutex::new(cache)),
        }
    }

    /// Get the data from cache or fetch it with the callback
    pub async fn get_or_fetch<TFunc, TFuture>(
        &self,
        url: &str,
        fetch: TFunc,
    ) -> ResResult<Arc<[u8]>>
    where
        TFunc: FnOnce() -> TFuture,
        TFuture: Future<Output = ResResult<Vec<u8>>>,
    {
        // handle data urls first, since parsing them is quick
        // and we don't need to cache them
        if url.starts_with("data:") {
            let data = match celerc::util::bytes_from_data_url(url) {
                Ok(data) => data.into_owned(),
                Err(e) => {
                    return Err(ResError::FailToLoadUrl(
                        url.to_string(),
                        format!("Failed to parse data URL: {e}"),
                    ));
                }
            };
            return Ok(Arc::from(data));
        }

        {
            let mut cache = self.inner.lock().await;
            if let Some(v) = cache.cache_get(url) {
                return Ok(Arc::clone(v));
            }
        }

        let v = fetch().await?;

        let v = Arc::from(v);
        {
            let mut cache = self.inner.lock().await;
            cache.cache_set(url.to_string(), Arc::clone(&v));
        }

        Ok(v)
    }
}
