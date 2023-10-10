use serde_json::{json, Value};

use crate::pack::PackerResult;
use crate::plug::{BuiltInPlugin, Plugin, PluginRuntime};
use crate::prop;
use crate::util::async_for;

use super::CompilerContext;

impl CompilerContext {
    pub async fn compile(&self) -> PackerResult<()> {
        todo!()
    }
    async fn create_plugin_runtimes(&self) -> Vec<Box<dyn PluginRuntime>> {
        let mut runtimes = Vec::with_capacity(self.phase0.meta.plugins.len());
        let _ = async_for!(plugin_inst in &self.phase0.meta.plugins, {
            let runtime = plugin_inst.create_runtime(self);
            runtimes.push(runtime);
        });
        runtimes
    }
}
