use std::borrow::Cow;

use serde_json::Value;

use crate::comp::CompDoc;
use crate::exec::ExecDoc;
use crate::expo::{ExpoDoc, ExportMetadata};
use crate::macros::{derive_wasm, async_trait};
use crate::pack::CompileContext;

mod error;
pub use error::*;
mod builtin;
mod js;
mod option;
pub use option::*;
mod parse;
pub use parse::*;

pub use builtin::BuiltInPlugin;
pub use js::ScriptPlugin;

/// Metadata of a plugin. This is exported and can be
/// used to display plugin information after compilation
#[derive(Debug, Clone, PartialEq)]
#[derive_wasm]
pub struct PluginMetadata {
    /// Id used to identify the plugin
    pub id: String,
    /// Display name of the plugin
    pub name: String,
    /// If the plugin is from a user plugin config
    pub is_from_user: bool,
}

/// The plugin runtime trait
///
/// A runtime of a plugin can store states that the plugin needs during the compilation.
/// Each compilation will spawn a new runtime with [`PluginInstance::create_runtime`]
#[async_trait(auto)]
pub trait PluginRuntime {
    /// Get the id of the plugin
    ///
    /// This is used to identify the plugin, and should be either the name of
    /// a built-in plugin or the path/url to an external plugin
    fn get_id(&self) -> Cow<'static, str>;

    /// Get the display name of the plugin.
    ///
    /// This should be used with diagnostics, and will be displayed in the settings
    fn get_display_name(&self) -> Cow<'static, str> {
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
            self.get_display_name().into_owned(),
            "on_export_exec_doc".into(),
        ))
    }
}

/// An instance of a plugin read from the config file, with a source where the plugin can be loaded
/// from and properties to pass into the plugin
#[derive(Debug, Clone)]
pub struct PluginInstance {
    /// The plugin definition
    pub plugin: Plugin,
    /// Props passed to the plugin
    pub props: Value,
    /// If the plugin should be added even if a duplicate exists
    pub allow_duplicate: bool,
}

impl PluginInstance {
    pub fn create_runtime(&self, ctx: &CompileContext<'_>) -> PluginResult<Box<dyn PluginRuntime>> {
        match &self.plugin {
            Plugin::BuiltIn(p) => p.create_runtime(ctx, &self.props),
            Plugin::Script(p) => p.create_runtime(ctx, &self.props),
        }
    }
    pub fn get_id(&self) -> Cow<'_, str> {
        match &self.plugin {
            Plugin::BuiltIn(p) => Cow::Owned(p.id()),
            Plugin::Script(p) => Cow::Borrowed(&p.id),
        }
    }

    pub fn get_display_name(&self) -> Cow<'_, str> {
        match &self.plugin {
            Plugin::BuiltIn(p) => Cow::Owned(format!("plugin/{}", p.id())),
            Plugin::Script(p) => Cow::Owned(p.get_display_name()),
        }
    }

    pub fn get_metadata(&self, is_from_user: bool) -> PluginMetadata {
        PluginMetadata {
            id: self.get_id().into_owned(),
            name: self.get_display_name().into_owned(),
            is_from_user,
        }
    }
}

/// The source of a plugin, from which a [`PluginInstance`] can be created
#[derive(Debug, Clone)]
pub enum Plugin {
    /// A built-in plugin
    BuiltIn(BuiltInPlugin),
    /// A script that is downloaded but not parsed.
    Script(ScriptPlugin),
}
