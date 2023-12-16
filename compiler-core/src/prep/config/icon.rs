//! Process the `icons` property

use serde_json::Value;

use crate::res::{Use, Resource, Loader, ResError};
use crate::env::yield_budget;
use crate::prop;
use crate::prep::{PreparedConfig, PrepResult, PrepError};

impl<L> PreparedConfig<L> where L: Loader {
    /// Process the `icons` property
    ///
    /// `use`'s are resolved in the context of `res`
    pub async fn load_icons( &mut self, res: &Resource<'_, L>, icons: Value,) -> PrepResult<()>
    {
        let icons = icons
            .try_into_object()
            .map_err(|_| 
                PrepError::InvalidConfigPropertyType(
                    self.trace.clone(), prop::ICONS.into(), "object"
                ))?;

        for (key, v) in icons.into_iter() {
            yield_budget(16).await;
            match Use::try_from(v) {
                Err(v) => {
                    // not a use, just a icon url
                    if v.is_array() || v.is_object() {
                        return Err(PrepError::InvalidConfigPropertyType(
                            self.trace.clone(),
                            format!("{}.{}", prop::ICONS, key),
                            "string",
                        ));
                    }
                    self.icons.insert(key, v.coerce_to_string());
                }
                Ok(Use::Invalid(path)) => return Err(ResError::InvalidUse(path)),
                Ok(Use::Valid(valid_use)) => {
                    let url = res.resolve(&valid_use)?.load_image_url().await?;
                    self.icons.insert(key, url);
                }
            }
        }

        Ok(())
    }
}
