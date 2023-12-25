use std::marker::PhantomData;
use std::borrow::Cow;
use std::collections::BTreeMap;

use derivative::Derivative;
use serde::{Serialize, Deserialize};
use serde_json::{Value, Map};

use crate::json::{Cast, Coerce};
use crate::res::{Use, Resource, Loader, ResError};
use crate::macros::{async_recursion, derive_wasm};
use crate::util::{StringMap};
use crate::lang::{Preset};
use crate::plugin::{PluginInstance};
use crate::prop;
use crate::env::yield_budget;

use super::{Setting, PrepResult, PrepError};

mod icon;
pub use icon::*;
mod map;
pub use map::*;
mod tag;
pub use tag::*;
mod trace;
pub use trace::*;
mod plugin;
pub use plugin::*;
mod preset;
pub use preset::*;

#[derive(Derivative, Debug)]
#[derivative(Default)]
pub struct PreparedConfig<'a> {
    trace: ConfigTrace,
    pub map: Option<MapMetadata>,
    pub icons: BTreeMap<String, String>,
    pub tags: BTreeMap<String, DocTag>,
    pub presets: BTreeMap<String, Preset>,
    pub plugins: Vec<PluginInstance>,
    pub splits: Vec<String>,
    #[derivative(Default(value = "2"))]
    pub default_icon_priority: i64,
    pub setting: Cow<'a, Setting>,
}

impl<'a> PreparedConfig<'a> {
    pub fn new(setting: &Setting) -> Self {
        Self {
            setting: Cow::Borrowed(setting),
            ..Default::default()
        }
    }

    pub async fn load_configs<L, TIter>(&mut self, res: &Resource<'_, L>, configs: TIter) -> PrepResult<()> 
    where
        L: Loader,
        TIter: IntoIterator<Item = Value>,
    {
        for (i, config) in configs.into_iter().enumerate() {
            yield_budget(16).await;
            self.trace.push(i);
            self.load_config(res, config).await?;
            self.trace.pop();
        }
        Ok(())
    }

    #[async_recursion(auto)]
    async fn load_config<L>(&mut self, res: &Resource<'_, L>, config: Value) -> PrepResult<()> 
    where
        L: Loader,
    {
        if self.trace.len() > self.setting.max_config_depth {
            return Err(PrepError::MaxConfigDepthExceeded(self.trace.clone()));
        }
        match Use::from_value(&config) {
            Some(Use::Invalid(path)) => {
                // is a `use`, but invalid
                Err(ResError::InvalidUse(path))?
            }
            Some(Use::Valid(valid_use)) => {
                // load a config from top-level use object
                let config_res= res.resolve(&valid_use)?;
                let config = config_res.load_structured().await?;
                // process this config with the config resource context instead of the project context
                // so `use`'s inside are resolved correctly
                self.load_config_properties(&config_res, config).await?
            }
            None => {
                // load a config directly from the object
                self.load_config_properties(res, config).await?
            }
        }
        Ok(())
    }

    /// Load individual config properties in the config JSON blob
    async fn load_config_properties<L>(&mut self, res: &Resource<'_, L>, config: Value) -> PrepResult<()> 
    where
        L: Loader,
    {
        let config = config
            .try_into_object()
            .map_err(|_| PrepError::InvalidConfigType(self.trace.clone()))?;

        for (key, value) in config {
            yield_budget(64).await;
            match key.as_ref() {
                prop::MAP => {
                    if self.map.is_some() {
                        return Err(PrepError::DuplicateMap(self.trace.clone()));
                    }
                    self.load_map(value).await?;
                }
                prop::ICONS => {
                    self.load_icons(res, value).await?;
                }
                prop::TAGS => {
                    self.load_tags(value).await?;
                }
                prop::SPLITS => {
                    // splits is an array of strings
                    let splits = check_array!(self, value, prop::SPLITS)?;
                    for split in splits.into_iter() {
                        yield_budget(256).await;
                        self.splits.push(split.coerce_to_string());
                    }
                }
                prop::PRESETS => {
                    self.load_presets(value).await?;
                }
                prop::DEFAULT_ICON_PRIORITY => {
                    let priority = value.try_coerce_to_i64().ok_or_else(|| {
                        PrepError::InvalidConfigPropertyType(
                            self.trace.clone(),
                            prop::DEFAULT_ICON_PRIORITY.into(),
                            prop::DEFAULT_ICON_PRIORITY.into(),
                        )
                    })?;
                    self.default_icon_priority = priority;
                }
                prop::PLUGINS => {
                    self.load_plugins(res, value).await?;
                }
                prop::INCLUDES => {
                    let configs = check_array!(self, value, prop::INCLUDES)?;
                    self.load_configs(res, configs).await?;
                }
                _ => return Err(PrepError::UnusedConfigProperty(self.trace.clone(), key.into())),
            }
        }

        Ok(())
    }

    /// Check if `prop_name` is `None`, otherwise returns [`PrepError::UnusedConfigProperty`] with the
    /// current trace.
    pub fn check_unused_property<S>(&self, prop_name: Option<S>) -> PrepResult<()> where S: Into<Cow<'static, str>> 
    {
        match prop_name {
            None => Ok(()),
            Some(name) => Err(PrepError::MissingConfigProperty(
                self.trace.clone(),
                name.into(),
            )),
        }
    }

}

/// Convert `prop` into a Map, otherwise returns [`PrepError::InvalidConfigPropertyType`]
///
/// Note this is a macro so that `prop_name` is only evaluated when an error occurs.
macro_rules! check_map {
    ($self:ident, $prop:expr, $prop_name:expr) => {
        {
            use $crate::json::Cast;
            let prop = $prop;
            match prop.try_into_object() {
                Ok(map) => Ok(map),
                Err(_) => {
                    let prop_name = $prop_name.into();
                    Err($crate::prep::PrepError::InvalidConfigPropertyType(
                        $self.trace.clone(),
                        prop_name,
                        "mapping object".into(),
                    ))
                }
            }
        }
    };
}
pub(crate) use check_map;

/// Convert `prop` into an array, otherwise returns [`PrepError::InvalidConfigPropertyType`]
///
/// Note this is a macro so that `prop_name` is only evaluated when an error occurs.
macro_rules! check_array {
    ($self:ident, $prop:expr, $prop_name:expr) => {
        {
            use $crate::json::Cast;
            let prop = $prop;
            match prop.try_into_array() {
                Ok(array) => Ok(array),
                Err(_) => {
                    let prop_name = $prop_name.into();
                    Err($crate::prep::PrepError::InvalidConfigPropertyType(
                        $self.trace.clone(),
                        prop_name,
                        "array".into(),
                    ))
                }
            }
        }
    };
}
pub(crate) use check_array;

/// Check if `prop` is `Some`, otherwise returns [`PrepError::MissingConfigProperty`] with the
/// current trace.
///
/// Note this is a macro so that `prop_name` is only evaluated when an error occurs.
macro_rules! check_required_property {
    ($self:ident, $prop:expr, $prop_name:expr) => {
        {
            let prop = $prop;
            match prop {
                Some(v) => Ok(v),
                None => {
                    let prop_name = $prop_name.into();
                    Err($crate::prep::PrepError::MissingConfigProperty(
                        $self.trace.clone(),
                        prop_name,
                    ))
                }
            }
        }
    };
}
pub(crate) use check_required_property;

