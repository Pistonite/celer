//! Compatibility Plugin

use crate::api::CompilerMetadata;
use crate::comp::CompDoc;
use crate::json::Coerce;
use crate::macros::async_trait;

// use super::{operation, PlugResult, PluginRuntime};

const HIDE_ICON_ON_MAP: &str = "hide-icon-on-map";
const HIDE_ICON_ON_DOC: &str = "hide-icon-on-doc";

// pub struct CompatPlugin;
// #[async_trait(?Send)]
// impl PluginRuntime for CompatPlugin {
//     async fn on_compile(&mut self, _: &CompilerMetadata, comp_doc: &mut CompDoc) -> PlugResult<()> {
//         operation::for_each_line!(line in comp_doc {
//             if let Some(flag) = line.properties.remove(HIDE_ICON_ON_MAP) {
//                 if flag.coerce_truthy() {
//                     line.map_icon = None;
//                 }
//             }
//             if let Some(flag) = line.properties.remove(HIDE_ICON_ON_DOC) {
//                 if flag.coerce_truthy() {
//                     line.doc_icon = None;
//                 }
//             }
//
//             line
//         });
//
//         Ok(())
//     }
// }
