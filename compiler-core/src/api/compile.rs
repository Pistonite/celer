use std::borrow::Cow;

use crate::comp::Compiler;
use crate::pack::pack_route;
use crate::plug::PluginRuntime;
use crate::types::ExecDoc;
use crate::util::async_for;

use super::CompilerContext;

impl CompilerContext {
    // TODO #78: will no longer need Option after compiler become not cancelable
    pub async fn compile(&self) -> Option<ExecDoc<'_>> {
        let mut plugin_runtimes = self.create_plugin_runtimes().await;

        // pack phase 1
        // resolve uses in the route
        let route = pack_route(
            &self.project_resource,
            self.phase0.route.clone(),
            self.setting.max_use_depth,
            self.setting.max_ref_depth,
        )
        .await;

        // comp phase
        let compiler = self.create_compiler();
        let mut comp_doc = match compiler.comp_doc(route).await {
            Ok(doc) => doc,
            // TODO #78: will no longer need Option after compiler become not cancelable
            Err(_) => {
                return None;
            }
        };

        for plugin in plugin_runtimes.iter_mut() {
            if let Err(e) = plugin.on_compile(&self.phase0.meta, &mut comp_doc).await {
                e.add_to_diagnostics(&mut comp_doc.diagnostics);
            }
        }

        // exec phase
        let mut exec_doc = match comp_doc.exec(&self.phase0.project).await {
            Ok(doc) => doc,
            // TODO #78: will no longer need Option after compiler become not cancelable
            Err(_) => {
                return None;
            }
        };

        for plugin in plugin_runtimes.iter_mut() {
            if let Err(e) = plugin
                .on_post_compile(&self.phase0.meta, &mut exec_doc)
                .await
            {
                e.add_to_diagnostics(&mut exec_doc.diagnostics);
            }
        }
        Some(exec_doc)
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
