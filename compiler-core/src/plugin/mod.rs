use std::borrow::Cow;

use serde_json::Value;

use crate::comp::CompDoc;
use crate::exec::ExecDoc;
use crate::expo::{ExportMetadata, ExpoDoc};
use crate::pack::CompileContext;
use crate::macros::derive_wasm;

mod error;
pub use error::*;
mod builtin;
mod js;
mod operation;
mod option;
pub use option::*;

pub use builtin::BuiltInPlugin;
pub use js::ScriptPlugin;

/// Metadata of a plugin. This is exported and can be
/// used to display plugin information after compilation
#[derive(Debug, Clone, PartialEq)]
#[derive_wasm]
pub struct PluginMetadata {
    /// Id used to identify the plugin
    pub id: Cow<'static, str>,
    /// Display name of the plugin
    pub name: Cow<'static, str>
}

/// The plugin runtime trait
///
/// A runtime of a plugin can store states that the plugin needs during the compilation.
/// Each compilation will spawn a new runtime with [`PluginInstance::create_runtime`]
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
    fn on_before_compile(&mut self, _ctx: &mut CompileContext) -> PluginResult<()> {
        Ok(())
    }
    /// Called after the route is compiled, to transform the route
    fn on_after_compile(&mut self, _doc: &mut CompDoc) -> PluginResult<()> {
        Ok(())
    }
    /// Called after the route is turned into ExecDoc
    fn on_after_execute(&mut self, _doc: &mut ExecDoc) -> PluginResult<()> {
        Ok(())
    }

    fn on_prepare_export(&mut self) -> PluginResult<Option<ExportMetadata>> {
        Ok(None)
    }

    fn on_export_comp_doc(&mut self, _doc: &CompDoc) -> PluginResult<Option<ExpoDoc>> {
        Ok(None)
    }
    fn on_export_exec_doc(&mut self, _doc: &ExecDoc) -> PluginResult<Option<ExpoDoc>> {
        Ok(None)
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
            Plugin::Script(p) => Cow::Borrowed(&p.id)
        }
    }

    pub fn get_display_name(&self) -> Cow<'_, str> {
        match &self.plugin {
            Plugin::BuiltIn(p) => Cow::Owned(p.id()),
            Plugin::Script(p) => Cow::Owned(p.get_display_name())
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
