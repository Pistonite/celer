use serde_json::Value;

use crate::pack::CompileContext;

use super::{PluginError, PluginResult, PluginRuntime};

#[derive(Debug, Clone, PartialEq)]
pub struct ScriptPlugin {
    /// The path or url of the plugin, used to identify duplicates
    pub id: String,
    /// The raw code of the plugin
    pub script: String,
}

impl ScriptPlugin {
    pub fn create_runtime(
        &self,
        _ctx: &CompileContext<'_>,
        _props: &Value,
    ) -> PluginResult<Box<dyn PluginRuntime>> {
        // TODO #24 implement JS plugin engine
        Err(PluginError::ScriptException(
            "Script plugins are not implemented yet".to_string(),
        ))
    }
    pub fn get_display_name(&self) -> String {
        let name = self.id.rfind('/').map(|x| &self.id[x + 1..]).unwrap_or(&self.id);
        format!("plugin/{name}")
    }
}


// pub struct ScriptPluginRuntime {
//     pub source: String,
// }
//
// impl PluginRuntime for ScriptPluginRuntime {
//     fn on_before_compile(&mut self, _: &mut CompileContext<'_>) -> PluginResult<()> {
//         // TODO #24 implement JS plugin engine
//         Err(PluginError::ScriptException(
//             "Script plugins are not implemented yet".to_string(),
//         ))
//     }
//
//     fn get_display_name(&self) -> Cow<'static, str> {
//         Cow::Owned(self.source.clone())
//     }
// }
