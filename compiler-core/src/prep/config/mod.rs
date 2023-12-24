use std::marker::PhantomData;
use std::borrow::Cow;
use std::collections::BTreeMap;

use serde::{Serialize, Deserialize};
use serde_json::{Value, Map};

use crate::Setting;
use crate::res::{Resource, Loader};
use crate::macros::{async_recursion, derive_wasm};
use crate::util::{StringMap};
use crate::lang::{Preset};
use crate::plugin::{PluginInstance};

use super::{PrepResult, PrepError};

mod icon;
pub use icon::*;
mod map;
pub use map::*;
mod tag;
pub use tag::*;
mod trace;
pub use trace::*;


#[derive(Debug)]
pub struct PreparedConfig<L> where L: Loader {
    trace: ConfigTrace,
    pub map: Option<MapMetadata>,
    pub icons: BTreeMap<String, String>,
    pub tags: BTreeMap<String, DocTag>,
    pub presets: BTreeMap<String, Preset>,
    pub plugins: Vec<PluginInstance>,
    pub splits: Vec<String>,
    pub default_icon_priority: Option<i64>,
    pub setting: Setting,
    _l: PhantomData<L>,
}

impl<L> Default for PreparedConfig<L> where L: Loader {
    fn default() -> Self {
        Self {
            trace: Default::default(),
            map: None,
            icons: Default::default(),
            tags: Default::default(),
            presets: Default::default(),
            plugins: Default::default(),
            splits: Default::default(),
            default_icon_priority: None,
            setting: Default::default(),
            _l: Default::default(),
        }
    }
}

impl<L> PreparedConfig<L> where L: Loader {
    pub async fn from_vec(setting: Setting, resource: &Resource<'_, L>, configs: &[Value]) -> PrepResult<Self> {
        let mut s = Self {
            setting,
            ..Default::default()
        };
        s.load_configs(resource, configs).await?;
        Ok(s)
    }

    async fn load_configs(&mut self, res: &Resource<'_, L>, configs: &[Value]) -> PrepResult<()> {
        for (i, config) in configs.iter().enumerate() {
            self.trace.push(i);
            self.load_config(res, config).await?;
            self.trace.pop();
        }
        Ok(())
    }

    // #[async_recursion(auto)]
    async fn load_config(&mut self, res: &Resource<'_, L>, config: &Value) -> PrepResult<()> {
        todo!()
        // if trace.len() > MAX_CONFIG_DEPTH {
        //     return Err(PackerError::MaxConfigDepthExceeded(trace.clone()));
        // }
        // match Use::try_from(config) {
        //     Ok(Use::Invalid(path)) => Err(PackerError::InvalidUse(path)),
        //     Ok(Use::Valid(valid_use)) => {
        //         // load a config from top-level use object
        //         process_config_from_use(builder, project_resource, valid_use, trace, setting).await
        //     }
        //     Err(v) => {
        //         // load a config directly from the object
        //         process_config(builder, project_resource, v, trace, setting).await
        //     }
        // }
        // Ok(())
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
                    Err(PrepError::InvalidConfigPropertyType(
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
                    Err(PrepError::InvalidConfigPropertyType(
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
                    Err(PrepError::MissingConfigProperty(
                        $self.trace.clone(),
                        prop_name,
                    ))
                }
            }
        }
    };
}
pub(crate) use check_required_property;

/// Config of the route project
#[derive(PartialEq, Default, Serialize, Deserialize, Debug, Clone)]
#[derive_wasm]
pub struct RouteConfig {
    #[serde(flatten)]
    pub meta: RouteMetadata,
    pub map: MapMetadata,

    /// Arbitrary key-value pairs that can be used for statistics or any other value
    pub stats: StringMap<String>,
    /// Icon id to url map
    pub icons: StringMap<String>,
    /// Tag id to tag
    pub tags: StringMap<DocTag>,
    /// Default tags to split
    pub splits: Vec<String>,
}

#[derive(PartialEq, Default, Serialize, Deserialize, Debug, Clone)]
#[derive_wasm]
pub struct RouteMetadata {
    /// Source of the route, could be a URL or any string
    pub source: String,
    /// Version of the project
    pub version: String,
    /// Display title of the project
    pub title: String,
}
