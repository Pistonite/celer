use std::sync::Arc;

use celerc::api::Setting;
use celerc::pack::{LocalResourceResolver, Resource, ResourcePath};
use celerc::util::{self, Path};
use js_sys::Function;
use log::info;
use wasm_bindgen::JsValue;

use crate::loader_file;
use crate::loader_url;
use crate::logger;

const SOURCE_NAME: &str = "(local)";


// /// Compile a document from web editor
// ///
// /// Return None if the compilation was interrupted
// /// TODO #78: Option no longer needed
// ///
// /// This returns an Option<ExecDoc>, but must be converted to WASM immediately because of lifetime
// /// As part of TODO #86 this should be improved as well
// pub async fn compile_document() -> Result<JsValue, JsValue> {
//     // create root resource
//
//     let setting = Setting::default();
//     let project_resource = match celerc::api::resolve_project(&resource).await {
//         Ok(x) => x,
//         Err(e) => {
//             return celerc::api::make_doc_for_packer_error(SOURCE_NAME, e)
//                 .await
//                 .into_wasm();
//         }
//     };
//     // TODO #86 cache this
//     let context = match celerc::api::prepare(SOURCE_NAME, project_resource, setting).await {
//         Ok(x) => x,
//         Err(e) => {
//             return celerc::api::make_doc_for_packer_error(SOURCE_NAME, e)
//                 .await
//                 .into_wasm();
//         }
//     };
//
//     context.compile().await.into_wasm()
// }
