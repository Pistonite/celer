use std::sync::Arc;

use axum::body::Bytes;
use once_cell::sync::Lazy;
use reqwest::{Client, StatusCode};
use tracing::{error, info};

use celerc::macros::async_trait;
use celerc::res::{Loader, LoaderFactory, ResError, ResPath, ResResult};

use super::ResourceCache;

const MAX_RESOURCE_SIZE: usize = 1024 * 1024 * 10; // 10 MB
static CACHE: Lazy<ResourceCache> = Lazy::new(ResourceCache::new);

struct ServerResourceLoaderFactory;
impl LoaderFactory for ServerResourceLoaderFactory {
    fn create_loader(&self) -> ResResult<Arc<dyn Loader>> {
        let cache = CACHE.clone();
        let loader = ServerResourceLoader::with_cache(cache)?;
        Ok(Arc::new(loader))
    }
}

pub fn setup_global_loader() {
    info!("setting up global loader factory...");
    let loader: Arc<ServerResourceLoaderFactory> = Arc::new(ServerResourceLoaderFactory);
    if celerc::env::global_loader_factory::set(loader).is_err() {
        error!("failed to set global loader factory because it is already set!");
    }
}

pub fn get_loader() -> ResResult<Arc<ServerResourceLoader>> {
    let cache = CACHE.clone();
    let loader = ServerResourceLoader::with_cache(cache)?;
    Ok(Arc::new(loader))
}

/// Loader for loading resources from the web
pub struct ServerResourceLoader {
    http_client: Client,
    cache: ResourceCache,
}

impl ServerResourceLoader {
    pub fn with_cache(cache: ResourceCache) -> ResResult<Self> {
        let http_client = create_http_client()?;
        Ok(Self { http_client, cache })
    }

    /// Load a resource from Url. Automatically retry if the request fails with retriable error
    async fn load_url(&self, url: &str) -> ResResult<Arc<[u8]>> {
        self.cache
            .get_or_fetch(url, || async {
                // send the request, retry if failed
                let retry = 3;
                let mut last_error = None;
                for _ in 0..retry {
                    match self.fetch(url).await {
                        Ok(data) => {
                            if data.len() > MAX_RESOURCE_SIZE {
                                // don't retry if the resource is too big
                                let err = ResError::FailToLoadUrl(
                                    url.to_string(),
                                    "Resource is too large".to_string(),
                                );
                                return Err(err);
                            }
                            return Ok(data.to_vec());
                        }
                        Err(e) => {
                            error!("Failed to fetch resource: {e}");
                            last_error = Some(e);
                        }
                    }
                }
                error!("Failed to load resource after max retries!");

                let error = last_error.unwrap_or_else(|| {
                    ResError::FailToLoadUrl(url.to_string(), "Unknown error".to_string())
                });

                Err(error)
            })
            .await
    }

    async fn fetch(&self, url: &str) -> Result<Bytes, ResError> {
        let response = self.http_client.get(url).send().await.map_err(|e| {
            ResError::FailToLoadUrl(url.to_string(), format!("Failed to send request: {e}"))
        })?;

        let status = response.status();
        if status != StatusCode::OK {
            let err = ResError::FailToLoadUrl(
                url.to_string(),
                format!("Got response with status: {status}"),
            );
            return Err(err);
        }

        let bytes = response.bytes().await.map_err(|e| {
            ResError::FailToLoadUrl(url.to_string(), format!("Failed to parse response: {e}"))
        })?;

        Ok(bytes)
    }
}

fn create_http_client() -> ResResult<Client> {
    Client::builder()
        .user_agent("celery")
        .gzip(true)
        // For some reason idle sockets are not closed
        // after timeout, use this to explicitly close them
        .pool_max_idle_per_host(4)
        .build()
        .map_err(|e| ResError::Create(format!("Failed to create http client: {e}")))
}

#[async_trait]
impl Loader for ServerResourceLoader {
    async fn load_raw(&self, path: &ResPath) -> ResResult<Arc<[u8]>> {
        let url = match path {
            ResPath::Local(path) => {
                error!("Local path not supported! This is not supposed to happen.");
                return Err(ResError::FailToLoadFile(
                    path.to_string(),
                    "unreachable".to_string(),
                ));
            }
            ResPath::Remote(prefix, path) => format!("{prefix}{path}"),
        };
        info!("Loading resource url: {url}");

        self.load_url(&url).await
    }
}
