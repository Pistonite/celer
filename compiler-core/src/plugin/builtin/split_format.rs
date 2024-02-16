//! Split format plugin
//!
//! Automatically set the `split-name` property based on the split type

use std::borrow::Cow;

use crate::comp::CompDoc;
use crate::plugin::{PluginResult, PluginRuntime};

pub struct SplitFormatPlugin;
impl PluginRuntime for SplitFormatPlugin {
    fn get_id(&self) -> Cow<'static, str> {
        Cow::Owned(super::BuiltInPlugin::SplitFormat.id())
    }
    fn on_after_compile(&mut self, comp_doc: &mut CompDoc) -> PluginResult<()> {
        todo!()
    }
}
