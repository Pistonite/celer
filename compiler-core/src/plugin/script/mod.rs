use serde_json::Value;

use crate::pack::CompileContext;

use super::{BoxedEarlyRuntime, PluginError, PluginResult, BoxedRuntime};

#[derive(Debug, Clone, PartialEq)]
pub struct Script {
    /// The path or url of the plugin, used to identify duplicates
    pub id: String,
    /// The raw code of the plugin
    pub script: String,
}

impl Script {
    pub fn create_early_runtime(&self) -> PluginResult<BoxedEarlyRuntime> {
        // TODO #24 implement JS plugin engine
        Err(PluginError::ScriptException(
            "Script plugins are not implemented yet".to_string(),
        ))
    }

    pub fn create_runtime(
        &self,
        _ctx: &CompileContext<'_>,
        _props: &Value,
    ) -> PluginResult<BoxedRuntime> {
        // TODO #24 implement JS plugin engine
        Err(PluginError::ScriptException(
            "Script plugins are not implemented yet".to_string(),
        ))
    }

    /// Get the display name of the plugin, which is the file name (xxx.js)
    pub fn get_display_name(&self) -> String {
        self.id
            .rfind('/')
            .map(|x| &self.id[x + 1..])
            .unwrap_or(&self.id)
            .to_string()
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
