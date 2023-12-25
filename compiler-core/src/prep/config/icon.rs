//! Process the `icons` property

use serde_json::Value;

use crate::json::Coerce;
use crate::res::{Use, Resource, Loader, ResError};
use crate::env::yield_budget;
use crate::prop;
use crate::prep::{PreparedConfig, PrepResult, PrepError};

impl<'a> PreparedConfig<'a> {
    /// Process the `icons` property
    ///
    /// `use`'s are resolved in the context of `res`
    pub async fn load_icons<L>(&mut self, res: &Resource<'_, L>, icons: Value,) -> PrepResult<()>
    where
        L: Loader,
    {

        let icons = super::check_map!(self, icons, prop::ICONS)?;

        for (key, v) in icons.into_iter() {
            yield_budget(16).await;
            match Use::from_value(&v) {
                None => {
                    // not a use, just a icon url
                    if v.is_array() || v.is_object() {
                        return Err(PrepError::InvalidConfigPropertyType(
                            self.trace.clone(),
                            format!("{}.{}", prop::ICONS, key).into(),
                            "string".into(),
                        ));
                    }
                    self.icons.insert(key, v.coerce_to_string());
                }
                Some(Use::Invalid(path)) => Err(ResError::InvalidUse(path))?,
                Some(Use::Valid(valid_use)) => {
                    let url = res.resolve(&valid_use)?.load_image_url().await?;
                    self.icons.insert(key, url);
                }
            }
        }

        Ok(())
    }
}
