//! Server-side plugin options parsing

use celerc::res::Resource;
use celerc::{PluginOptions, PluginOptionsRaw};

use super::ServerResourceLoader;

/// Parse the plugin options, and return the error message if the options are invalid
pub async fn parse_plugin_options(
    plugin_options: &str,
    root_resource: &Resource<'_, ServerResourceLoader>,
) -> Result<PluginOptions, String> {
    let raw_options: PluginOptionsRaw =
        serde_json::from_str(plugin_options).map_err(|e| e.to_string())?;
    let options = raw_options
        .parse(root_resource)
        .await
        .map_err(|e| e.to_string())?;
    Ok(options)
}
