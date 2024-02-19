
use std::io::Read;

use cached::{Cached, TimedSizedCache};
use flate2::read::GzDecoder;
use once_cell::sync::Lazy;
use reqwest::{Client, StatusCode};
use tokio::sync::Mutex;
use tracing::{info, error};

use celerc::env::RefCounted;
use celerc::res::{Loader, ResError, ResPath, ResResult};
use celerc::macros::async_trait;

const MAX_RESOURCE_SIZE: usize = 1024 * 1024 * 10; // 10 MB
static LOADER: Lazy<RefCounted<ServerResourceLoader>> = Lazy::new(|| RefCounted::new(ServerResourceLoader::default()));

pub fn get_loader() -> RefCounted<ServerResourceLoader> {
    RefCounted::clone(&LOADER)
}

/// Loader for loading resources from the web
pub struct ServerResourceLoader {
    http_client: Client,
    cache: Mutex<TimedSizedCache<String, RefCounted<[u8]>>>
}

impl Default for ServerResourceLoader {
    fn default() -> Self {
        Self::new()
    }
}

impl ServerResourceLoader {
    pub fn new() -> Self {
        let http_client= Client::new();
        let cache = Mutex::new(TimedSizedCache::with_size_and_lifespan(128, 301));
        Self {
            http_client,
            cache
        }
    }
    /// Load a resource from Url or cache. On error, returns an additional should_retry flag.
    async fn load_url(&self, url: &str) -> Result<RefCounted<[u8]>, (ResError, bool)> {
        let mut cache = self.cache.lock().await;
        if let Some(data) = cache.cache_get(url) {
            return Ok(RefCounted::clone(data));
        }
        let response = self.http_client.get(url)
            .header("User-Agent", "celery")
            .header("Accept-Encoding", "gzip")
            .send()
            .await
            .map_err(|e| {
                let err = ResError::FailToLoadUrl(url.to_string(), format!("Failed to send request: {e}"));
                (err, true)
            })?;

        let status = response.status();
        if status != StatusCode::OK {
            let err = ResError::FailToLoadUrl(url.to_string(), format!("Got response with status: {status}"));
            return Err((err, true));
        }

        // check Content-Encoding
        let is_gzipped = match response.headers().get("Content-Encoding") {
            Some(encoding) => {
                if encoding != "gzip" {
                    let encoding = encoding.to_str().unwrap_or("unknown");
                    let err = ResError::FailToLoadUrl(url.to_string(), format!("Server responded with unsupported encoding: {encoding}"));
                    return Err((err, true));
                }
                true
            }
            None => false
        };

        let bytes = response.bytes().await
            .map_err(|e| {
                let err = ResError::FailToLoadUrl(url.to_string(), format!("Failed to parse response: {e}"));
                (err, true)
            })?;

        if bytes.len() > MAX_RESOURCE_SIZE {
            // don't retry if the resource is too big
            let err = ResError::FailToLoadUrl(url.to_string(), format!("Resource is too large"));
            return Err((err, false));
        }

        let bytes = if is_gzipped {
            let mut decoder = GzDecoder::new(&bytes[..]);
            let mut buffer = Vec::new();
            if let Err(e) = decoder.read_to_end(&mut buffer) {
                let err  = ResError::FailToLoadUrl(url.to_string(), format!("Failed to decode response: {e}"));
                return Err((err, true));
            }
            buffer
        } else {
            bytes.to_vec()
        };
        let data = RefCounted::from(bytes);
        cache.cache_set(url.to_string(), RefCounted::clone(&data));
        return Ok(data);
    }
}

#[async_trait]
impl Loader for ServerResourceLoader {
    async fn load_raw(&self, path: &ResPath) -> ResResult<RefCounted<[u8]>> { 
        let url = match path {
            ResPath::Local(path) => {
                error!("Local path not supported! This is not supposed to happen.");
                return Err(ResError::FailToLoadFile(path.to_string(), "unreachable".to_string()))
            }
            ResPath::Remote(prefix, path) => format!("{prefix}{path}")
        };
        info!("Loading resource url: {url}");
        let retry = 3;
        let mut last_error = None;
        for _ in 0..retry {
            match self.load_url(&url).await {
                Ok(data) => {
                    info!("Resource loaded from url: {}", url);
                    return Ok(data);
                }
                Err((e, should_retry)) => {
                    if !should_retry {
                        error!("Non-retryable error encounted!");
                        return Err(e);
                    }
                    last_error = Some(e);
                }
            }
        }

        error!("Failed to load resource after max retries!");
        match last_error {
            Some(e) => Err(e),
            None => Err(ResError::FailToLoadUrl(url.clone(), "Unknown error".to_string()))
        }
    }

}
