use std::sync::Arc;

use cached::{Cached, TimedSizedCache};
use once_cell::sync::Lazy;
use tokio::sync::Mutex;

use celerc::prep::{PrepCtxData, PrepResult};
use celerc::PrepCtx;

use super::ServerResourceLoader;

static CACHE: Lazy<PrepCtxCache> = Lazy::new(PrepCtxCache::new);

pub async fn get_context(
    owner: &str,
    repo: &str,
    path: Option<&str>,
    reference: &str,
) -> PrepResult<PrepCtx<ServerResourceLoader>> {
    CACHE.get_or_create(owner, repo, path, reference).await
}

/// Cache for the output of prep phase
pub struct PrepCtxCache {
    cache: Mutex<TimedSizedCache<String, Arc<PrepCtxData>>>,
}

impl Default for PrepCtxCache {
    fn default() -> Self {
        Self::new()
    }
}

impl PrepCtxCache {
    pub fn new() -> Self {
        Self {
            cache: Mutex::new(TimedSizedCache::with_size_and_lifespan(32, 301)),
        }
    }

    /// Get a context for the given project, either from cache or newly created
    ///
    /// A new resource loader with the global resource cache will be created for the context
    pub async fn get_or_create(
        &self,
        owner: &str,
        repo: &str,
        path: Option<&str>,
        reference: &str,
    ) -> PrepResult<PrepCtx<ServerResourceLoader>> {
        let key = make_key(owner, repo, path, reference);

        // check if the context is in the cache
        {
            let mut cache = self.cache.lock().await;
            if let Some(data) = cache.cache_get(&key) {
                let loader = super::loader::get_loader()?;
                return Ok(PrepCtx::from_data(Arc::clone(data), loader));
            }
        }

        // build a new context
        let mut builder =
            super::new_context_builder(owner, repo, Some(reference))?.with_route_built();
        if let Some(path) = path {
            builder = builder.entry_point(Some(path.to_string()));
        }

        let ctx = builder.build_context().await?;
        {
            let mut cache = self.cache.lock().await;
            cache.cache_set(key, Arc::clone(ctx.get_data()));
        }
        Ok(ctx)
    }
}

/// Create a cache key for a project reference
pub fn make_key(owner: &str, repo: &str, path: Option<&str>, reference: &str) -> String {
    let p = path.unwrap_or("");
    if p.is_empty() {
        format!("{owner}/{repo}/{reference}")
    } else {
        format!("{owner}/{repo}/{reference}/{p}")
    }
}
