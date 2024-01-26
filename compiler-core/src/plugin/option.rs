use super::PluginInstance;


/// Options for users to alter plugins defined in the route
pub struct PluginOptions {
    /// List of plugin ids to remove
    pub remove: Vec<String>,
    /// List of plugins to add
    pub add: Vec<PluginInstance>,
}

