// use http::Uri;
// use hyper::Client;
// use hyper::client::HttpConnector;
//
// use crate::pack::{PackerError, PackerResult};
//
// use super::ResourceLoader;
//
// pub struct UrlLoader {
//     client: Client<HttpConnector>,
// }
//
// #[cfg_attr(not(feature = "wasm"), async_trait::async_trait)]
// #[cfg_attr(feature = "wasm", async_trait::async_trait(?Send))]
// impl ResourceLoader for UrlLoader {
//     async fn load_raw(&self, url: &str) -> PackerResult<Vec<u8>> {
//         let uri: Uri = url.parse().map_err(|_| PackerError::InvalidUrl(url.to_string()))?;
//         let response = self.client.get(uri).await.map_err(|e| {
//             PackerError::LoadUrl(e.to_string())
//         })?;
//         Err(PackerError::NotImpl(
//             "UrlLoader::load_raw is not implemented".to_string(),
//         ))
//     }
//
//     async fn load_image_url(&self, url: &str) -> PackerResult<String> {
//         // image is already a URL, so just return it
//         Ok(url.to_string())
//     }
// }
