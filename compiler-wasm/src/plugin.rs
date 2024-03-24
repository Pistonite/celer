use std::cell::RefCell;

use log::{error, info};

use celerc::plugin;

thread_local! {
    static PLUGIN_OPTIONS: RefCell<Result<Option<plugin::Options>, String>> = const { RefCell::new(Ok(None)) };
}

pub async fn set_plugin_options(options: Option<plugin::OptionsRaw>) {
    let options = match options {
        Some(options) => options,
        None => {
            info!("clearing plugin options");
            let _ = PLUGIN_OPTIONS.with(|x| x.replace(Ok(None)));
            return;
        }
    };

    info!("setting plugin options");
    match options.parse(&super::get_root_project_resource()).await {
        Ok(options) => {
            let _ = PLUGIN_OPTIONS.with(|x| x.replace(Ok(Some(options))));
            info!("plugin options set successfully");
        }
        Err(e) => {
            let e = e.to_string();
            let _ = PLUGIN_OPTIONS.with(|x| x.replace(Err(e.clone())));
            error!("failed to set plugin options: {}", e);
        }
    }
}

pub fn get_plugin_options() -> Result<Option<plugin::Options>, String> {
    PLUGIN_OPTIONS.with(|x| x.borrow().clone())
}
