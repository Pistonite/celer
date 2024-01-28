use std::cell::RefCell;

use log::{info, error};

use celerc::{PluginOptions, PluginOptionsRaw};
use celerc::macros::derive_wasm;

#[derive(Debug, Clone)]
#[derive_wasm]
pub struct SetPluginOptionsResult(pub Option<String>);

thread_local! {
    static PLUGIN_OPTIONS: RefCell<Result<Option<PluginOptions>, String>> = RefCell::new(Ok(None));
}

pub async fn set_plugin_options(options: Option<PluginOptionsRaw>) -> SetPluginOptionsResult {
    let options = match options {
        Some(options) => options,
        None => {
            info!("clearing plugin options");
            let _ = PLUGIN_OPTIONS.with(|x| x.replace(Ok(None)));
            return SetPluginOptionsResult(None);
        },
    };
    info!("setting plugin options");
    match options.parse(&super::get_root_project_resource()).await {
        Ok(options) => {
            let _ = PLUGIN_OPTIONS.with(|x| x.replace(Ok(Some(options))));
            info!("plugin options set successfully");
            SetPluginOptionsResult(None)
        },
        Err(e) => {
            let e = e.to_string();
            let _ = PLUGIN_OPTIONS.with(|x| x.replace(Err(e.clone())));
            error!("failed to set plugin options: {}", e);
            SetPluginOptionsResult(Some(e))
        }
    }
}

pub fn get_plugin_options() -> Result<Option<PluginOptions>, String> {
    PLUGIN_OPTIONS.with(|x| x.borrow().clone())
}
