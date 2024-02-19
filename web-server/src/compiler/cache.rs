use cached::{Cached, TimedSizedCache};
use once_cell::sync::Lazy;
use tokio::sync::Mutex;

use celerc::env::RefCounted;
use celerc::prep::PrepResult;
use celerc::PreparedContext;

use super::ServerResourceLoader;

/// Create a cache key for a project reference
pub fn cache_key(owner: &str, repo: &str, path: Option<&str>, reference: &str) -> String {
    let p = path.unwrap_or("");
    if p.is_empty() {
        format!("{owner}/{repo}/{reference}")
    } else {
        format!("{owner}/{repo}/{reference}/{p}")
    }
}

static CACHE: Lazy<Mutex<PreparedContextCache>> =
    Lazy::new(|| Mutex::new(PreparedContextCache::new()));
pub async fn get_context(
    owner: &str,
    repo: &str,
    path: Option<&str>,
    reference: &str,
) -> PrepResult<RefCounted<PreparedContext<ServerResourceLoader>>> {
    let mut cache = CACHE.lock().await;
    cache.get(owner, repo, path, reference).await
}

pub struct PreparedContextCache {
    cache: TimedSizedCache<String, RefCounted<PreparedContext<ServerResourceLoader>>>,
}

impl Default for PreparedContextCache {
    fn default() -> Self {
        Self::new()
    }
}

impl PreparedContextCache {
    pub fn new() -> Self {
        Self {
            cache: TimedSizedCache::with_size_and_lifespan(32, 301),
        }
    }
    pub async fn get(
        &mut self,
        owner: &str,
        repo: &str,
        path: Option<&str>,
        reference: &str,
    ) -> PrepResult<RefCounted<PreparedContext<ServerResourceLoader>>> {
        let key = cache_key(owner, repo, path, reference);

        if let Some(ctx) = self.cache.cache_get(&key) {
            return Ok(RefCounted::clone(ctx));
        }

        let mut builder =
            super::new_context_builder(owner, repo, Some(reference)).with_route_built();
        if let Some(path) = path {
            builder = builder.entry_point(Some(path.to_string()));
        }

        let ctx = builder.build_context().await?;
        let ctx = RefCounted::new(ctx);
        self.cache.cache_set(key, RefCounted::clone(&ctx));
        Ok(ctx)
    }
}
