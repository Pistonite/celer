use serde_json::Value;

use crate::env::yield_budget;
use crate::plugin::{self, PluginParseError};
use crate::prep::{PrepError, PrepResult, PreparedConfig};
use crate::prop;
use crate::res::{Loader, Resource};

impl<'a> PreparedConfig<'a> {
    /// Process the `plugins` property
    ///
    /// `use`'s are resolved in the context of `res`
    pub async fn load_plugins<L>(&mut self, res: &Resource<'_, L>, value: Value) -> PrepResult<()>
    where
        L: Loader,
    {
        let plugins = super::check_array!(self, value, prop::PLUGINS)?;

        // parse each plugin
        for (i, v) in plugins.into_iter().enumerate() {
            yield_budget(16).await;

            let v = super::check_map!(self, v, format!("{}[{}]", prop::PLUGINS, i))?;
            match plugin::Instance::parse(v, res).await {
                Ok(plugin) => self.plugins.push(plugin),
                Err(PluginParseError::MissingPlugin) => {
                    return Err(PrepError::MissingConfigProperty(
                        self.trace.clone(),
                        format!("{}[{}].{}", prop::PLUGINS, i, prop::USE).into(),
                    ))
                }
                Err(PluginParseError::UnusedProperty(key)) => {
                    return Err(PrepError::UnusedConfigProperty(
                        self.trace.clone(),
                        format!("{}[{}].{}", prop::PLUGINS, i, key).into(),
                    ))
                }
                Err(PluginParseError::InvalidPlugin(path)) => {
                    return Err(PrepError::InvalidPlugin(self.trace.clone(), path))
                }
                Err(PluginParseError::LoadError(err)) => return Err(err.into()),
            }
        }

        Ok(())
    }
}
