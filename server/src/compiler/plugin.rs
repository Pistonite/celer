//! Server-side plugin options parsing

use celerc::plugin;
use celerc::res::Resource;

use super::ServerResourceLoader;

/// Parse the plugin options, and return the error message if the options are invalid
pub async fn parse_plugin_options(
    plugin_options: &str,
    root_resource: &Resource<'_, ServerResourceLoader>,
) -> Result<plugin::Options, String> {
    let raw_options: plugin::OptionsRaw =
        serde_json::from_str(plugin_options).map_err(|e| e.to_string())?;
    let options = raw_options
        .parse(root_resource)
        .await
        .map_err(|e| e.to_string())?;
    Ok(options)
}
