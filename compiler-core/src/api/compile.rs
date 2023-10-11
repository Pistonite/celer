use std::borrow::Cow;

use celerctypes::ExecDoc;
use serde_json::{json, Value};

use crate::comp::Compiler;
use crate::pack::{PackerResult, pack_route};
use crate::plug::{BuiltInPlugin, Plugin, PluginRuntime};
use crate::prop;
use crate::util::async_for;

use super::CompilerContext;

impl CompilerContext {
    pub async fn compile(&self) -> PackerResult<ExecDoc<'_>> {
        let plugin_runtimes = self.create_plugin_runtimes().await;

        // pack phase 1
        // resolve uses in the route
        let route = pack_route(
            &self.project_resource,
            self.phase0.route.clone(),
            self.setting.max_use_depth,
            self.setting.max_ref_depth,
        ).await;

        // comp phase
        let mut compiler = self.create_compiler();
        compiler.comp_doc();
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
    fn create_compiler(&self) -> Compiler<'_> {
        Compiler {
            meta: Cow::Borrowed(&self.phase0.meta),
            color: self.phase0.project.map.initial_color.clone(),
            coord: self.phase0.project.map.initial_coord.clone(),
            project: Cow::Borrowed(&self.phase0.project),
            max_preset_depth: self.setting.max_preset_ref_depth,
        }
    }
}
