use cached::{Return, TimedSizedCache};
use cached::proc_macro::cached;
use celerc::macros::async_trait;
use reqwest::{Client, StatusCode};
use tracing::info;

mod loader;
pub use loader::*;

// pub fn create_loader() -> MarcLoader {
//     Marc::new(UrlLoader::new())
// }
//
// pub struct UrlLoader {
//     http_client: Client,
//
// }
//
// impl UrlLoader {
//     pub fn new() -> Self {
//         let client = Client::new();
//         Self {
//             http_client: client
//         }
//     }
// }
//
// #[async_trait]
// impl ResourceLoader for UrlLoader {
//     async fn load_raw(&self, path: &str) -> PackerResult<Vec<u8>> {
//         info!("loading url: {path}");
//         let value = get_url(&self.http_client, path).await?;
//         if value.was_cached {
//             info!("cached: {path}");
//         } else {
//             info!("downloaded: {path}");
//         }
//         Ok(value.value)
//     }
//
//     async fn load_image_url(&self, path: &str) -> PackerResult<String> {
//         Ok(path.to_string())
//     }
// }
//
// #[cached(
//     size=128,
//     result=true,
//     with_cached_flag=true,
//     time=300,
//     key="String",
//     convert=r#"{ String::from(url) }"#
// )]
// async fn get_url(client: &Client, url: &str) -> PackerResult<Return<Vec<u8>>> {
//     let response = client.get(url).send().await.map_err(|_| {
//         PackerError::LoadUrl(format!("Failed to load url: {url}"))
//     })?;
//
//     let bytes = match response.status() {
//         StatusCode::OK => {
//             response.bytes().await.map_err(|_| {
//                 PackerError::LoadUrl(format!("Failed to parse body when loading url: {url}"))
//             })?
//         }
//         other => {
//             return Err(PackerError::LoadUrl(format!("Failed to load url: {url}, status: {status}", url=url, status=other)));
//         }
//     };
//
//     Ok(Return::new(bytes.to_vec()))
// }
