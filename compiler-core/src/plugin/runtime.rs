use std::borrow::Cow;

use serde_json::Value;

use crate::comp::CompDoc;
use crate::exec::ExecDoc;
use crate::expo::{ExpoDoc, ExportMetadata};
use crate::macros::async_trait;
use crate::pack::CompileContext;

use super::{PluginError, PluginResult};

/// The plugin runtime trait
///
/// A runtime of a plugin can store states that the plugin needs during the compilation.
/// Each compilation will spawn a new runtime with [`PluginInstance::create_runtime`]
#[async_trait(auto)]
pub trait Runtime {
    /// Get the id of the plugin
    ///
    /// This is used to identify the plugin, and should be either the name of
    /// a built-in plugin or the path/url to an external plugin
    fn get_id(&self) -> Cow<'static, str>;

    /// Get the name used as the diagnostics source for this plugin
    ///
    /// This is what shows up as the source of the diagnostics,
    /// for diagnostics that are caused by this plugin
    fn get_diagnostics_source(&self) -> Cow<'static, str> {
        Cow::Owned(format!("plugin/{}", self.get_id()))
    }

    /// Called before route is compiled, to make changes to the compiler
    async fn on_before_compile<'p>(&mut self, _ctx: &mut CompileContext<'p>) -> PluginResult<()> {
        Ok(())
    }

    /// Called after the route is compiled, to transform the route
    async fn on_after_compile<'p>(&mut self, _doc: &mut CompDoc<'p>) -> PluginResult<()> {
        Ok(())
    }

    /// Called after the route is turned into ExecDoc
    async fn on_after_execute<'p>(&mut self, _doc: &mut ExecDoc<'p>) -> PluginResult<()> {
        Ok(())
    }

    /// Called at the end of compilation to check what exports are available
    async fn on_prepare_export(&mut self) -> PluginResult<Option<Vec<ExportMetadata>>> {
        Ok(None)
    }

    /// Called only in export workflow, to let the exporter access the CompDoc
    ///
    /// If the exporter needs to access the ExecDoc as well, it should return `None`.
    /// Otherwise, the returned export data will be used and the exporter will not be called
    /// with the ExecDoc
    async fn on_export_comp_doc<'p>(
        &mut self,
        _export_id: &str,
        _payload: &Value,
        _doc: &CompDoc<'p>,
    ) -> PluginResult<Option<ExpoDoc>> {
        Ok(None)
    }

    /// Called only in export workflow, to let the exporter access the ExecDoc
    ///
    /// The exporter must return the export data or throw an error
    async fn on_export_exec_doc(
        &mut self,
        _export_id: &str,
        _payload: Value,
        _doc: &ExecDoc,
    ) -> PluginResult<ExpoDoc> {
        Err(PluginError::NotImplemented(
            self.get_diagnostics_source().into_owned(),
            "on_export_exec_doc".into(),
        ))
    }
}

pub type BoxedRuntime = Box<dyn Runtime>;
