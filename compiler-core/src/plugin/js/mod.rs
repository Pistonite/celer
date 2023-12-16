use serde_json::Value;

use crate::{CompilerMetadata, comp::{CompDoc, Compiler}};

use super::{PluginRuntime, PluginError, PluginResult};

pub struct ScriptPlugin {
    pub source: String,
    pub script: String,
}

impl ScriptPlugin {
    pub fn create_runtime<'a>(&self, compiler: &Compiler<'a>, props: &Value) -> Box<dyn PluginRuntime> {
        Box::new(ScriptPluginRuntime {
            source: self.source.clone(),
        })
    }
}

pub struct ScriptPluginRuntime {
    pub source: String,
}

impl PluginRuntime for ScriptPluginRuntime {
    fn on_before_compile(&mut self, _: &Compiler<'_>) -> PluginResult<()> {
        // TODO #24 implement JS plugin engine
        Err(PluginError::ScriptException(
            "Script plugins are not implemented yet".to_string(),
        ))
    }
}
