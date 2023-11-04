use std::borrow::Cow;

use crate::comp::Compiler;
use crate::pack::pack_route;
use crate::plug::PluginRuntime;
use crate::types::ExecDoc;
use crate::util::yield_budget;

use super::CompilerContext;

impl CompilerContext {
    pub async fn compile(&self) -> ExecDoc<'_> {
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
        let mut compiler = self.create_compiler();
        let mut comp_doc = match compiler.comp_doc(route).await {
            Ok(doc) => doc,
            Err(e) => {
                compiler.create_empty_doc_for_error(&[e])
            }
        };

        for plugin in plugin_runtimes.iter_mut() {
            if let Err(e) = plugin.on_compile(&self.phase0.meta, &mut comp_doc).await {
                e.add_to_diagnostics(&mut comp_doc.diagnostics);
            }
        }

        // exec phase
        let mut exec_doc = comp_doc.exec(&self.phase0.project).await;

        for plugin in plugin_runtimes.iter_mut() {
            if let Err(e) = plugin
                .on_post_compile(&self.phase0.meta, &mut exec_doc)
                .await
            {
                e.add_to_diagnostics(&mut exec_doc.diagnostics);
            }
        }
        exec_doc
    }

    async fn create_plugin_runtimes(&self) -> Vec<Box<dyn PluginRuntime>> {
        let mut runtimes = Vec::with_capacity(self.phase0.meta.plugins.len());
        for plugin_inst in &self.phase0.meta.plugins {
            yield_budget(1).await;
            let runtime = plugin_inst.create_runtime(self);
            runtimes.push(runtime);
        }
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
