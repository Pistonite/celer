use std::{marker::PhantomData, borrow::Cow};

use crate::res::{Resource, Loader};

use super::{PrepResult, PrepError};

mod icon;
pub use icon::*;
mod map;
pub use map::*;
mod tag;
use serde_json::{Value, Map};
pub use tag::*;
mod trace;
pub use trace::*;


#[derive(Default, Debug)]
pub struct PreparedConfig<L> where L: Loader {
    trace: ConfigTrace,
    pub map: Option<MapMetadata>,
    pub icons: BTreeMap<String, String>,
    pub tags: BTreeMap<String, DocTag>,
    pub presets: BTreeMap<String, Preset>,
    pub plugins: Vec<PluginInstance>,
    pub splits: Vec<String>,
    pub default_icon_priority: Option<i64>,
    pub settings: Settings,
    _l: PhantomData<L>,
}

impl<L> PreparedConfig<L> where L: Loader {
    pub async fn from_vec(settings: Settings, resource: &Resource<'_, L>, configs: &[Value]) -> PrepResult<Self> {
        let mut s = Self {
            settings,
            ..Default::default()
        };
        s.load_configs(configs).await?;
        Ok(s)
    }

    async fn load_configs(&mut self, res: &Resource<'_, L>, configs: &[Value]) -> PrepResult<()> {
        for (i, config) in configs.iter().enumerate() {
            self.trace.push(i);
            self.load_config(config).await?;
            self.trace.pop();
        }
        Ok(())
    }

    #[async_recursion(auto)]
    async fn load_config(&mut self, res: &Resource<'_, L>, config: &Value) -> PrepResult<()> {
        if trace.len() > MAX_CONFIG_DEPTH {
            return Err(PackerError::MaxConfigDepthExceeded(trace.clone()));
        }
        match Use::try_from(config) {
            Ok(Use::Invalid(path)) => Err(PackerError::InvalidUse(path)),
            Ok(Use::Valid(valid_use)) => {
                // load a config from top-level use object
                process_config_from_use(builder, project_resource, valid_use, trace, setting).await
            }
            Err(v) => {
                // load a config directly from the object
                process_config(builder, project_resource, v, trace, setting).await
            }
        }
        Ok(())
    }

    /// Check if `prop` is `Some`, otherwise returns [`PrepError::MissingConfigProperty`] with the
    /// current trace.
    pub fn check_required_property<T, S>(&self, prop: Option<T>, prop_name: S) -> PrepResult<T> where S: Into<Cow<'static, str>> 
    {
        match prop {
            Some(v) => Ok(v),
            None => Err(PrepError::MissingConfigProperty(
                self.trace.clone(),
                prop_name.into(),
            )),
        }
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

    /// Convert `prop` into a Map, otherwise returns [`PrepError::InvalidConfigPropertyType`]
    pub fn check_map<TPropName, TType>(&self, prop: Value, prop_name: TPropName) -> PrepResult<Map<String, Value>>
    where
        TPropName: Into<Cow<'static, str>>,
        TType: Into<Cow<'static, str>>,
    {
        let map = prop
            .try_into_map()
            .map_err(|_| PrepError::InvalidConfigPropertyType(self.trace.clone(), prop_name.into(), "mapping object".into()))?;
        Ok(map)
    }

    /// Convert `prop` into an array, otherwise returns [`PrepError::InvalidConfigPropertyType`]
    pub fn check_array<TPropName, TType>(&self, prop: Value, prop_name: TPropName) -> PrepResult<Vec<Value>>
    where
        TPropName: Into<Cow<'static, str>>,
        TType: Into<Cow<'static, str>>,
    {
        let map = prop
            .try_into_array()
            .map_err(|_| PrepError::InvalidConfigPropertyType(self.trace.clone(), prop_name.into(), "array".into()))?;
        Ok(map)
    }
}

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
