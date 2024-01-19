use std::borrow::Cow;

use serde_json::Value;

use crate::pack::CompileContext;

use super::{PluginError, PluginResult, PluginRuntime};

#[derive(Debug, Clone, PartialEq)]
pub struct ScriptPlugin {
    pub source: String,
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
}

pub struct ScriptPluginRuntime {
    pub source: String,
}

impl PluginRuntime for ScriptPluginRuntime {
    fn on_before_compile(&mut self, _: &mut CompileContext<'_>) -> PluginResult<()> {
        // TODO #24 implement JS plugin engine
        Err(PluginError::ScriptException(
            "Script plugins are not implemented yet".to_string(),
        ))
    }

    fn get_source(&self) -> Cow<'static, str> {
        Cow::Owned(self.source.clone())
    }
}
