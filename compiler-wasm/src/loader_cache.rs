// // Currently not used. The implementation is kept for reference.
//
// use cached::proc_macro::cached;
// use serde_json::Value;
//
// use celerc::macros::async_trait;
// use celerc::pack::{ ResourceLoader, PackerResult};
//
// /// A loader that caches loaded JSON object in memory to skip parsing. The cache is global.
// pub struct CachedLoader<L> {
//     delegate: L,
// }
//
// impl<L> CachedLoader<L> {
//     pub fn new(delegate: L) -> Self {
//         Self { delegate }
//     }
//
//     pub fn inner(&self) -> &L {
//         &self.delegate
//     }
// }
//
// #[async_trait(?Send)]
// impl<L> ResourceLoader for CachedLoader<L> where L: ResourceLoader {
//     async fn load_raw(&self, r: &str) -> PackerResult<Vec<u8>> {
//         self.delegate.load_raw(r).await
//     }
//
//     async fn load_image_url(&self, path: &str) -> PackerResult<String> {
//         self.delegate.load_image_url(path).await
//     }
//
//     async fn load_structured(&self, path: &str) -> PackerResult<Value> {
//         load_structured_internal(&self.delegate, path).await
//     }
// }
//
// // associative function not supported by cached crate
// // so we need to use helpers
//
// #[cached(
//     size=512,
//     key="String",
//     convert = r#"{ path.to_string() }"#,
//     time=301,
// )]
// async fn load_structured_internal(loader: &dyn ResourceLoader, path: &str) -> PackerResult<Value> {
//     loader.load_structured(path).await
// }
