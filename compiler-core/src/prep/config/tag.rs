//! Process the `tags` property

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::json::Coerce;
use crate::res::{Use, Resource, Loader, ResError};
use crate::env::yield_budget;
use crate::prop;
use crate::prep::{PreparedConfig, PrepResult, PrepError};
use crate::macros::derive_wasm;

use super::{check_map};

impl<L> PreparedConfig<L> where L: Loader {
    /// Process the `tags` property
    pub async fn process_tags_config(
        &mut self,
        tags: Value,
    ) -> PrepResult<()> {
        let tags = super::check_map!(self, tags, prop::TAGS)?;
        for (key, mut value) in tags.into_iter() {
            yield_budget(16).await;
            let mut tag = DocTag::default();
            // resolve includes
            if let Some(includes) = value
                .as_object_mut()
                .and_then(|map| map.remove(prop::INCLUDES))
            {
                let includes = match includes {
                    Value::Array(v) => v,
                    Value::Object(_) => {
                        return Err(PrepError::InvalidConfigPropertyType(
                            self.trace.clone(),
                            format!("{}.{}.{}", prop::TAGS, key, prop::INCLUDES).into(),
                            "tag name or array of tag names".into(),
                        ))
                    }
                    other => vec![other],
                };
                for include in includes {
                    let include = include.coerce_to_string();

                    let include_tag = match self.tags.get(&include) {
                        None if include != key => {
                            return Err(PrepError::TagNotFound(self.trace.clone(), include.into()))
                        }
                        other => other,
                    };
                    if let Some(t) = include_tag {
                        tag.apply_override(t);
                    }
                }
            }

            let last_tag = serde_json::from_value::<DocTag>(value).map_err(|_| {
                PrepError::InvalidConfigPropertyType(self.trace.clone(), 
                    format!("{}.{}", prop::TAGS, key).into(), "tag object".into())
            })?;
            tag.apply_override(&last_tag);

            self.tags.insert(key, tag);
        }

        Ok(())
    }
}

/// Document tag type
///
/// Used to style text and provide extra function to the engine
#[derive(PartialEq, Default, Serialize, Deserialize, Debug, Clone)]
#[derive_wasm]
pub struct DocTag {
    /// Bold style
    pub bold: Option<bool>,
    /// Italic style
    pub italic: Option<bool>,
    /// Underline style
    pub underline: Option<bool>,
    /// Strikethrough style
    pub strikethrough: Option<bool>,
    /// Color of the text (light, dark)
    pub color: Option<DocTagColor>,
    /// Background color of the text (light, dark)
    pub background: Option<DocTagColor>,
    /// Display name for the split type of this tag
    pub split_type: Option<String>,
}

macro_rules! apply_tag_prop {
    ($self:ident, $other:ident, $prop:ident) => {
        if $other.$prop.is_some() {
            $self.$prop = $other.$prop.as_ref().cloned();
        }
    };
}

macro_rules! apply_tag_if_none {
    ($self:ident, $other:ident, $prop:ident) => {
        if $self.$prop.is_none() {
            $self.$prop = $other.$prop.as_ref().cloned();
        }
    };
}

impl DocTag {
    /// Apply the styles from another tag if the other tag has the property
    pub fn apply_override(&mut self, other: &DocTag) {
        apply_tag_prop!(self, other, bold);
        apply_tag_prop!(self, other, italic);
        apply_tag_prop!(self, other, underline);
        apply_tag_prop!(self, other, strikethrough);
        apply_tag_prop!(self, other, color);
        apply_tag_prop!(self, other, background);
        apply_tag_prop!(self, other, split_type);
    }

    /// Apply the styles from another tag if self doesn't have the property
    pub fn apply_to_default(&mut self, other: &DocTag) {
        apply_tag_if_none!(self, other, bold);
        apply_tag_if_none!(self, other, italic);
        apply_tag_if_none!(self, other, underline);
        apply_tag_if_none!(self, other, strikethrough);
        apply_tag_if_none!(self, other, color);
        apply_tag_if_none!(self, other, background);
        apply_tag_if_none!(self, other, split_type);
    }
}

/// Used to specify color for [`DocTag`]s.
#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
#[derive_wasm]
#[serde(untagged)]
pub enum DocTagColor {
    /// Single color for any background
    Single(String),
    /// Different colors for light and dark backgrounds
    LightDark {
        /// Color to set if the text is displayed with a light background
        light: Option<String>,
        /// Color to set if the text is displayed with a dark background
        dark: Option<String>,
    },
}
